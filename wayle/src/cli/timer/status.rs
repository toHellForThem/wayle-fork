use super::{
    duration::format_human,
    proxy::{connect, format_error},
};
use crate::cli::CliAction;

/// Executes the status command.
///
/// # Errors
/// Returns error if D-Bus communication fails.
pub async fn execute() -> CliAction {
    let (_connection, proxy) = connect().await?;

    let running = proxy
        .running()
        .await
        .map_err(|e| format_error("get running state", e))?;
    let paused = proxy
        .paused()
        .await
        .map_err(|e| format_error("get paused state", e))?;
    let duration = proxy
        .duration()
        .await
        .map_err(|e| format_error("get duration", e))?;

    if !running {
        println!("Stopped (duration: {})", format_human(duration));
        return Ok(());
    }

    let remaining = proxy
        .remaining()
        .await
        .map_err(|e| format_error("get remaining", e))?;

    let state = if paused { "Paused" } else { "Running" };
    println!(
        "{state} ({} remaining of {})",
        format_human(remaining),
        format_human(duration)
    );

    Ok(())
}
