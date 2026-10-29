use std::sync::Arc;

use wayle_config::ConfigService;

use crate::services::timer::TimerService;

pub(crate) struct TimerDropdownInit {
    pub timer: Arc<TimerService>,
    pub config: Arc<ConfigService>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum TimerField {
    Days,
    Hours,
    Minutes,
    Seconds,
}

#[derive(Debug)]
pub(crate) enum TimerDropdownInput {
    /// Adjust one duration field by `amount` (positive or negative).
    Adjust { field: TimerField, amount: i64 },
    /// The command text field changed.
    CommandChanged(String),
    /// Start the timer with the current duration.
    Start,
    /// Pause or resume the running timer.
    TogglePause,
    /// Stop and reset to the configured default duration.
    Reset,
    /// Re-read duration and command from the shared state (on open).
    SyncFromState,
}

#[derive(Debug)]
pub(crate) enum TimerDropdownCmd {
    /// Running/paused state changed; refresh the Pause/Play button.
    StateChanged,
}
