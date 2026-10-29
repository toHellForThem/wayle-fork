//! Shared reactive state for the countdown timer.
//!
//! The timer is deadline-based: a running timer stores an absolute
//! [`SystemTime`] deadline, so it stays correct across system sleep and is
//! restored after a reboot from `~/.local/state/wayle/timer.json`.

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use tracing::warn;
use wayle_config::infrastructure::paths::ConfigPaths;
use wayle_core::Property;

/// Largest unit the timer was configured with. Used by the `as-set`
/// formatting mode to decide the display granularity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Granularity {
    /// Display as `d hh:mm:ss`.
    Days,
    /// Display as `hh:mm:ss`.
    Hours,
    /// Display as `mm:ss`.
    Minutes,
    /// Display as `ss`.
    Seconds,
}

impl Granularity {
    /// Picks the largest unit that the given duration covers.
    pub fn from_secs(secs: u64) -> Self {
        if secs >= 86_400 {
            Self::Days
        } else if secs >= 3_600 {
            Self::Hours
        } else if secs >= 60 {
            Self::Minutes
        } else {
            Self::Seconds
        }
    }

    /// Picks the largest unit that was explicitly set among the parts.
    pub fn from_parts(days: u32, hours: u32, minutes: u32, _seconds: u32) -> Self {
        if days > 0 {
            Self::Days
        } else if hours > 0 {
            Self::Hours
        } else if minutes > 0 {
            Self::Minutes
        } else {
            Self::Seconds
        }
    }
}

/// Snapshot of the actions to perform when the timer reaches zero.
#[derive(Clone, Default)]
struct ExpiryHooks {
    play_melody: bool,
    melody: String,
    run_command: bool,
    command: String,
}

/// On-disk representation of a running or paused timer.
#[derive(Serialize, Deserialize)]
struct PersistedTimer {
    running: bool,
    paused: bool,
    duration_secs: u64,
    remaining_secs: Option<u64>,
    deadline_unix: Option<u64>,
    granularity: Granularity,
    command: String,
}

/// Reactive state for the countdown timer.
///
/// Properties update automatically; bar modules watch them for display and the
/// dropdown edits them through the D-Bus / service API.
#[derive(Clone)]
pub struct TimerState {
    /// Whether the timer is currently counting down.
    pub running: Property<bool>,
    /// Whether the timer is paused.
    pub paused: Property<bool>,
    /// Total configured duration in seconds.
    pub duration_secs: Property<u64>,
    /// Remaining seconds, `None` when idle.
    pub remaining_secs: Property<Option<u64>>,
    /// Largest unit the timer was configured with.
    pub granularity: Property<Granularity>,
    /// Command executed on expiry when `run-command` is enabled.
    pub command: Property<String>,
    timer_token: Arc<Mutex<CancellationToken>>,
    hooks: Arc<Mutex<ExpiryHooks>>,
    deadline: Arc<Mutex<Option<SystemTime>>>,
    paused_remaining: Arc<Mutex<Option<u64>>>,
}

impl TimerState {
    /// Creates a timer preloaded with the given duration and command.
    pub fn new(default_duration_secs: u64, command: String) -> Self {
        let duration = default_duration_secs.max(1);
        Self {
            running: Property::new(false),
            paused: Property::new(false),
            duration_secs: Property::new(duration),
            remaining_secs: Property::new(None),
            granularity: Property::new(Granularity::from_secs(duration)),
            command: Property::new(command),
            timer_token: Arc::new(Mutex::new(CancellationToken::new())),
            hooks: Arc::new(Mutex::new(ExpiryHooks::default())),
            deadline: Arc::new(Mutex::new(None)),
            paused_remaining: Arc::new(Mutex::new(None)),
        }
    }

    /// Restores a persisted timer from disk, if any.
    ///
    /// - Running with a future deadline: resumes with the remaining time.
    /// - Running with a past deadline: stops silently (no melody/command).
    /// - Paused: restores the paused remaining time.
    pub fn restore(&self) {
        let Some(path) = state_file() else {
            return;
        };
        let Ok(text) = std::fs::read_to_string(&path) else {
            return;
        };
        let Ok(data) = serde_json::from_str::<PersistedTimer>(&text) else {
            let _ = std::fs::remove_file(&path);
            return;
        };

        self.duration_secs.set(data.duration_secs.max(1));
        self.granularity.set(data.granularity);
        self.command.set(data.command);

        if data.paused {
            let remaining = data.remaining_secs.unwrap_or(data.duration_secs).max(1);
            *self
                .paused_remaining
                .lock()
                .unwrap_or_else(|p| p.into_inner()) = Some(remaining);
            self.remaining_secs.set(Some(remaining));
            self.running.set(true);
            self.paused.set(true);
            return;
        }

        if !data.running {
            self.clear_persisted();
            return;
        }

        let Some(deadline_unix) = data.deadline_unix else {
            self.clear_persisted();
            return;
        };

        let now = unix_now();
        if deadline_unix > now {
            let remaining = deadline_unix - now;
            let deadline = UNIX_EPOCH + Duration::from_secs(deadline_unix);
            *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) = Some(deadline);
            self.remaining_secs.set(Some(remaining));
            self.running.set(true);
            self.paused.set(false);
            self.spawn_timer();
        } else {
            // Expired while the machine was off: stop without running hooks.
            self.running.set(false);
            self.paused.set(false);
            self.remaining_secs.set(None);
            self.clear_persisted();
        }
    }

    /// Starts (or restarts) the timer with the given duration and hooks.
    pub fn start(
        &self,
        duration_secs: Option<u64>,
        play_melody: bool,
        melody: String,
        run_command: bool,
        command: String,
    ) {
        if let Some(secs) = duration_secs {
            let secs = secs.max(1);
            self.duration_secs.set(secs);
            self.granularity.set(Granularity::from_secs(secs));
        }

        self.begin(play_melody, melody, run_command, command);
    }

    /// Starts the timer using the currently configured duration without
    /// recomputing the display granularity (keeps `as-set` intact).
    pub fn start_current(
        &self,
        play_melody: bool,
        melody: String,
        run_command: bool,
        command: String,
    ) {
        self.begin(play_melody, melody, run_command, command);
    }

    fn begin(&self, play_melody: bool, melody: String, run_command: bool, command: String) {
        self.cancel_timer();

        let duration = self.duration_secs.get().max(1);
        self.duration_secs.set(duration);

        let deadline = SystemTime::now() + Duration::from_secs(duration);
        *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) = Some(deadline);
        *self
            .paused_remaining
            .lock()
            .unwrap_or_else(|p| p.into_inner()) = None;

        self.remaining_secs.set(Some(duration));
        self.paused.set(false);
        self.running.set(true);

        self.store_hooks(play_melody, melody, run_command, command);
        self.persist();
        self.spawn_timer();
    }

    /// Stops the timer and clears the remaining time.
    pub fn stop(&self) {
        self.cancel_timer();
        *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) = None;
        *self
            .paused_remaining
            .lock()
            .unwrap_or_else(|p| p.into_inner()) = None;
        self.running.set(false);
        self.paused.set(false);
        self.remaining_secs.set(None);
        self.clear_persisted();
    }

    /// Pauses a running timer.
    pub fn pause(&self) {
        if self.running.get() && !self.paused.get() {
            self.cancel_timer();
            let remaining = self
                .current_remaining()
                .unwrap_or_else(|| self.duration_secs.get().max(1));
            *self
                .paused_remaining
                .lock()
                .unwrap_or_else(|p| p.into_inner()) = Some(remaining);
            self.remaining_secs.set(Some(remaining));
            self.paused.set(true);
            self.persist();
        }
    }

    /// Resumes a paused timer.
    pub fn resume(&self) {
        if self.running.get() && self.paused.get() {
            let remaining = self
                .paused_remaining
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .unwrap_or_else(|| self.duration_secs.get().max(1))
                .max(1);

            let deadline = SystemTime::now() + Duration::from_secs(remaining);
            *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) = Some(deadline);
            *self
                .paused_remaining
                .lock()
                .unwrap_or_else(|p| p.into_inner()) = None;

            self.remaining_secs.set(Some(remaining));
            self.paused.set(false);
            self.persist();
            self.spawn_timer();
        }
    }

    /// Pauses a running timer or resumes a paused one.
    pub fn toggle_pause(&self) {
        if !self.running.get() {
            return;
        }
        if self.paused.get() {
            self.resume();
        } else {
            self.pause();
        }
    }

    /// Stops the timer and resets it to the given default duration.
    pub fn reset_to(&self, secs: u64) {
        self.cancel_timer();
        *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) = None;
        *self
            .paused_remaining
            .lock()
            .unwrap_or_else(|p| p.into_inner()) = None;

        let secs = secs.max(1);
        self.duration_secs.set(secs);
        self.granularity.set(Granularity::from_secs(secs));
        self.running.set(false);
        self.paused.set(false);
        self.remaining_secs.set(Some(secs));
        self.clear_persisted();
    }

    /// Sets the timer duration from whole seconds.
    pub fn set_duration(&self, secs: u64) {
        let secs = secs.max(1);
        self.duration_secs.set(secs);
        self.granularity.set(Granularity::from_secs(secs));
        if !self.running.get() {
            self.remaining_secs.set(Some(secs));
        }
    }

    /// Sets the timer duration from explicit parts, keeping the largest
    /// non-zero unit for `as-set` formatting.
    pub fn set_time_parts(&self, days: u32, hours: u32, minutes: u32, seconds: u32) {
        let total = u64::from(days) * 86_400
            + u64::from(hours) * 3_600
            + u64::from(minutes) * 60
            + u64::from(seconds);
        let secs = total.max(1);

        self.granularity
            .set(Granularity::from_parts(days, hours, minutes, seconds));
        self.duration_secs.set(secs);
        if !self.running.get() {
            self.remaining_secs.set(Some(secs));
        }
    }

    /// Adds (or subtracts, with a negative value) time to the running timer.
    pub fn add_time(&self, delta_secs: i64) {
        if !self.running.get() {
            return;
        }

        if self.paused.get() {
            let current = self
                .paused_remaining
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .unwrap_or(0);
            let new_remaining = apply_delta(current, delta_secs);
            if new_remaining == 0 {
                self.stop();
            } else {
                *self
                    .paused_remaining
                    .lock()
                    .unwrap_or_else(|p| p.into_inner()) = Some(new_remaining);
                self.remaining_secs.set(Some(new_remaining));
                self.persist();
            }
            return;
        }

        let Some(deadline) = *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) else {
            return;
        };

        let new_deadline = if delta_secs >= 0 {
            deadline + Duration::from_secs(delta_secs.unsigned_abs())
        } else {
            deadline
                .checked_sub(Duration::from_secs(delta_secs.unsigned_abs()))
                .unwrap_or_else(SystemTime::now)
        };

        if new_deadline <= SystemTime::now() {
            self.stop();
        } else {
            *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) = Some(new_deadline);
            if let Some(remaining) = self.current_remaining() {
                self.remaining_secs.set(Some(remaining));
            }
            self.persist();
        }
    }

    /// Updates the command executed on expiry.
    pub fn set_command(&self, command: String) {
        self.command.set(command);
        if self.running.get() {
            self.persist();
        }
    }

    fn current_remaining(&self) -> Option<u64> {
        let deadline = (*self.deadline.lock().unwrap_or_else(|p| p.into_inner()))?;
        deadline
            .duration_since(SystemTime::now())
            .ok()
            .map(ceil_secs)
    }

    fn store_hooks(&self, play_melody: bool, melody: String, run_command: bool, command: String) {
        let mut hooks = self.hooks.lock().unwrap_or_else(|p| p.into_inner());
        *hooks = ExpiryHooks {
            play_melody,
            melody,
            run_command,
            command,
        };
    }

    fn spawn_timer(&self) {
        let token = CancellationToken::new();
        {
            let mut guard = self.timer_token.lock().unwrap_or_else(|p| p.into_inner());
            *guard = token.clone();
        }

        let hooks = self.hooks.lock().unwrap_or_else(|p| p.into_inner()).clone();
        let state = self.clone();

        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(1));
            tick.tick().await;

            loop {
                tokio::select! {
                    () = token.cancelled() => break,
                    _ = tick.tick() => {
                        let deadline = *state
                            .deadline
                            .lock()
                            .unwrap_or_else(|p| p.into_inner());
                        let Some(deadline) = deadline else {
                            break;
                        };

                        match deadline.duration_since(SystemTime::now()) {
                            Ok(remaining) if !remaining.is_zero() => {
                                state.remaining_secs.set(Some(ceil_secs(remaining)));
                            }
                            _ => {
                                state.finish(&hooks);
                                break;
                            }
                        }
                    }
                }
            }
        });
    }

    fn finish(&self, hooks: &ExpiryHooks) {
        *self.deadline.lock().unwrap_or_else(|p| p.into_inner()) = None;
        *self
            .paused_remaining
            .lock()
            .unwrap_or_else(|p| p.into_inner()) = None;
        self.running.set(false);
        self.paused.set(false);
        self.remaining_secs.set(None);
        self.clear_persisted();

        if hooks.play_melody && !hooks.melody.is_empty() {
            crate::process::run_if_set(&hooks.melody);
        }
        if hooks.run_command && !hooks.command.is_empty() {
            crate::process::run_if_set(&hooks.command);
        }
    }

    fn cancel_timer(&self) {
        let guard = self.timer_token.lock().unwrap_or_else(|p| p.into_inner());
        guard.cancel();
    }

    fn persist(&self) {
        let Some(path) = state_file() else {
            return;
        };

        if !self.running.get() {
            let _ = std::fs::remove_file(&path);
            return;
        }

        let deadline_unix = self
            .deadline
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .and_then(|deadline| deadline.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs());

        let data = PersistedTimer {
            running: self.running.get(),
            paused: self.paused.get(),
            duration_secs: self.duration_secs.get(),
            remaining_secs: self.remaining_secs.get(),
            deadline_unix,
            granularity: self.granularity.get(),
            command: self.command.get(),
        };

        match serde_json::to_string(&data) {
            Ok(json) => {
                if let Err(err) = std::fs::write(&path, json) {
                    warn!(error = %err, path = %path.display(), "cannot persist timer state");
                }
            }
            Err(err) => warn!(error = %err, "cannot serialize timer state"),
        }
    }

    fn clear_persisted(&self) {
        if let Some(path) = state_file() {
            let _ = std::fs::remove_file(path);
        }
    }
}

impl Default for TimerState {
    fn default() -> Self {
        Self::new(300, String::new())
    }
}

fn state_file() -> Option<PathBuf> {
    ConfigPaths::state_dir()
        .ok()
        .map(|dir| dir.join("timer.json"))
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn ceil_secs(duration: Duration) -> u64 {
    let extra = if duration.subsec_nanos() > 0 { 1 } else { 0 };
    duration.as_secs() + extra
}

fn apply_delta(current: u64, delta_secs: i64) -> u64 {
    if delta_secs >= 0 {
        current.saturating_add(delta_secs.unsigned_abs())
    } else {
        current.saturating_sub(delta_secs.unsigned_abs())
    }
}
