use wayle_ipc::timer::TimerProxy;
use zbus::{Connection, Error as ZbusError};

use crate::cli::dbus;

const SERVICE_NAME: &str = "Timer";

/// Creates a `TimerProxy` connection.
///
/// # Errors
/// Returns error if the D-Bus connection or proxy creation fails.
pub async fn connect() -> Result<(Connection, TimerProxy<'static>), String> {
    let connection = dbus::session().await?;

    let proxy = TimerProxy::new(&connection)
        .await
        .map_err(|e| format!("Failed to create timer proxy: {e}"))?;

    Ok((connection, proxy))
}

/// Transforms zbus errors into user-friendly messages.
pub fn format_error(operation: &str, error: ZbusError) -> String {
    dbus::format_error(SERVICE_NAME, operation, error)
}
