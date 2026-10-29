use super::{
    duration::{format_human, parse_duration},
    proxy::{connect, format_error},
};
use crate::cli::CliAction;

/// Executes the set command.
///
/// # Errors
/// Returns error if D-Bus communication fails or the duration is invalid.
pub async fn execute(spec: String) -> CliAction {
    let (_connection, proxy) = connect().await?;

    let secs = parse_duration(&spec)?;
    proxy
        .set_duration(secs)
        .await
        .map_err(|e| format_error("set timer duration", e))?;

    println!("Duration set to {}", format_human(secs));

    Ok(())
}
