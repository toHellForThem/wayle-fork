//! Timer control command definitions and dispatch.

mod add;
mod command;
/// Duration parsing helpers.
pub mod commands;
mod duration;
mod pause;
mod proxy;
mod reset;
mod resume;
mod set;
mod start;
/// Status command implementation.
pub mod status;
mod stop;
mod toggle_pause;

use commands::TimerCommands;

use super::CliAction;

/// Executes timer control commands.
///
/// # Errors
/// Returns error if the command execution fails.
pub async fn execute(command: TimerCommands) -> CliAction {
    match command {
        TimerCommands::Start { duration } => start::execute(duration).await,
        TimerCommands::Stop => stop::execute().await,
        TimerCommands::Pause => pause::execute().await,
        TimerCommands::Resume => resume::execute().await,
        TimerCommands::TogglePause => toggle_pause::execute().await,
        TimerCommands::Reset => reset::execute().await,
        TimerCommands::Set { duration } => set::execute(duration).await,
        TimerCommands::Add { value } => add::execute(value).await,
        TimerCommands::Command { command } => command::execute(command).await,
        TimerCommands::Status => status::execute().await,
    }
}
