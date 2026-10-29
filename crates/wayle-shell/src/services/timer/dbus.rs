//! D-Bus interface for timer control.

use std::sync::Arc;

use tracing::instrument;
use wayle_config::ConfigService;
use zbus::interface;

use super::state::TimerState;

pub const SERVICE_NAME: &str = "com.wayle.Timer1";
pub const SERVICE_PATH: &str = "/com/wayle/Timer";

/// D-Bus daemon exposing countdown timer controls.
pub struct TimerDaemon {
    state: TimerState,
    config: Arc<ConfigService>,
}

impl TimerDaemon {
    pub fn new(state: TimerState, config: Arc<ConfigService>) -> Self {
        Self { state, config }
    }

    /// Snapshot of the expiry configuration plus the editable command.
    fn expiry(&self) -> (bool, String, bool, String) {
        let config = self.config.config();
        let timer = &config.modules.timer;
        (
            timer.play_melody.get(),
            timer.melody.get(),
            timer.run_command.get(),
            self.state.command.get(),
        )
    }
}

#[interface(name = "com.wayle.Timer1")]
impl TimerDaemon {
    /// Starts the timer with the given duration in seconds.
    #[instrument(skip(self), fields(duration_secs))]
    pub async fn start(&self, duration_secs: u64) {
        let (play_melody, melody, run_command, command) = self.expiry();
        self.state.start(
            Some(duration_secs),
            play_melody,
            melody,
            run_command,
            command,
        );
    }

    /// Starts the timer using its currently configured duration.
    #[instrument(skip(self))]
    pub async fn start_current(&self) {
        let (play_melody, melody, run_command, command) = self.expiry();
        self.state
            .start_current(play_melody, melody, run_command, command);
    }

    /// Stops the timer and clears the remaining time.
    #[instrument(skip(self))]
    pub async fn stop(&self) {
        self.state.stop();
    }

    /// Pauses the running timer.
    #[instrument(skip(self))]
    pub async fn pause(&self) {
        self.state.pause();
    }

    /// Resumes the paused timer.
    #[instrument(skip(self))]
    pub async fn resume(&self) {
        self.state.resume();
    }

    /// Toggles between paused and running.
    #[instrument(skip(self))]
    pub async fn toggle_pause(&self) {
        self.state.toggle_pause();
    }

    /// Stops the timer and resets it to the configured default duration.
    #[instrument(skip(self))]
    pub async fn reset(&self) {
        let default = u64::from(self.config.config().modules.timer.default_duration.get());
        self.state.reset_to(default);
    }

    /// Sets the duration in seconds.
    #[instrument(skip(self), fields(secs))]
    pub async fn set_duration(&self, secs: u64) {
        self.state.set_duration(secs);
    }

    /// Sets the duration from explicit day/hour/minute/second parts.
    #[instrument(skip(self), fields(days, hours, minutes, seconds))]
    pub async fn set_time_parts(&self, days: u32, hours: u32, minutes: u32, seconds: u32) {
        self.state.set_time_parts(days, hours, minutes, seconds);
    }

    /// Adds (or subtracts, with a negative value) seconds to a running timer.
    #[instrument(skip(self), fields(delta_secs))]
    pub async fn add_time(&self, delta_secs: i64) {
        self.state.add_time(delta_secs);
    }

    /// Sets the command executed on expiry.
    #[instrument(skip(self))]
    pub async fn set_command(&self, command: &str) {
        self.state.set_command(command.to_owned());
    }

    /// Whether the timer is counting down.
    #[zbus(property)]
    pub async fn running(&self) -> bool {
        self.state.running.get()
    }

    /// Whether the timer is paused.
    #[zbus(property)]
    pub async fn paused(&self) -> bool {
        self.state.paused.get()
    }

    /// Remaining seconds (0 when idle).
    #[zbus(property)]
    pub async fn remaining(&self) -> u64 {
        self.state.remaining_secs.get().unwrap_or(0)
    }

    /// Configured duration in seconds.
    #[zbus(property)]
    pub async fn duration(&self) -> u64 {
        self.state.duration_secs.get()
    }

    /// Command executed on expiry.
    #[zbus(property)]
    pub async fn command(&self) -> String {
        self.state.command.get()
    }
}
