use super::{
    duration::{format_human, parse_duration},
    proxy::{connect, format_error},
};
use crate::cli::CliAction;

/// Executes the start command.
///
/// # Errors
/// Returns error if D-Bus communication fails or the duration is invalid.
pub async fn execute(duration: Option<String>) -> CliAction {
    let (_connection, proxy) = connect().await?;

    match duration {
        Some(spec) => {
            let secs = parse_duration(&spec)?;
            proxy
                .start(secs)
                .await
                .map_err(|e| format_error("start timer", e))?;
            println!("Started for {}", format_human(secs));
        }
        None => {
            proxy
                .start_current()
                .await
                .map_err(|e| format_error("start timer", e))?;
            println!("Started");
        }
    }

    Ok(())
}
