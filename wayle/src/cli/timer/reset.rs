use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the reset command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .reset()
        .await
        .map_err(|e| format_error("reset timer", e))?;

    println!("Reset");

    Ok(())
}
