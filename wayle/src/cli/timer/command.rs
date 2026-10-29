use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the command-set subcommand.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute(command: String) -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .set_command(&command)
        .await
        .map_err(|e| format_error("set timer command", e))?;

    println!("Command set");

    Ok(())
}
