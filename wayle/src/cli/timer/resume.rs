use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the resume command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .resume()
        .await
        .map_err(|e| format_error("resume timer", e))?;

    println!("Resumed");

    Ok(())
}
