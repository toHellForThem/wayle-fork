use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the pause command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .pause()
        .await
        .map_err(|e| format_error("pause timer", e))?;

    println!("Paused");

    Ok(())
}
