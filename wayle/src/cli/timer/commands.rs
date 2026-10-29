use clap::Subcommand;

/// Timer control subcommands.
#[derive(Subcommand, Debug)]
pub enum TimerCommands {
    /// Start the timer (optionally with a duration, e.g. 5m, 1h30m, 90s)
    Start {
        /// Duration (bare seconds or units: d, h, m, s)
        #[arg(value_name = "DURATION")]
        duration: Option<String>,
    },

    /// Stop the timer
    Stop,

    /// Pause the running timer
    Pause,

    /// Resume the paused timer
    Resume,

    /// Toggle between paused and running
    TogglePause,

    /// Stop the timer and restore its configured duration
    Reset,

    /// Set the duration without starting (e.g. 5m)
    Set {
        /// Duration (bare seconds or units: d, h, m, s)
        #[arg(value_name = "DURATION")]
        duration: String,
    },

    /// Add (or subtract with a leading '-') time to a running timer
    #[command(allow_hyphen_values = true)]
    Add {
        /// Delta, e.g. +5m or -30s
        #[arg(value_name = "DELTA")]
        value: String,
    },

    /// Set the command executed when the timer expires
    Command {
        /// Shell command
        #[arg(value_name = "COMMAND")]
        command: String,
    },

    /// Show current timer status
    Status,
}
