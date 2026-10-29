use super::{
    duration::{format_human, parse_signed},
    proxy::{connect, format_error},
};
use crate::cli::CliAction;

/// Executes the add command.
///
/// # Errors
/// Returns error if D-Bus communication fails or the delta is invalid.
pub async fn execute(value: String) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let delta = parse_signed(&value)?;
    proxy
        .add_time(delta)
        .await
        .map_err(|e| format_error("adjust timer remaining", e))?;

    let magnitude = format_human(delta.unsigned_abs());
    if delta >= 0 {
        println!("Added {magnitude}");
    } else {
        println!("Subtracted {magnitude}");
    }

    Ok(())
}
