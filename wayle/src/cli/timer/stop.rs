use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the stop command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .stop()
        .await
        .map_err(|e| format_error("stop timer", e))?;

    println!("Stopped");

    Ok(())
}
