use super::proxy::{connect, format_error};
use crate::cli::CliAction;

/// Executes the toggle-pause command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    proxy
        .toggle_pause()
        .await
        .map_err(|e| format_error("toggle timer pause", e))?;

    println!("Toggled pause");

    Ok(())
}
