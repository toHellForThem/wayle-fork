//! Timer service with D-Bus interface.
//!
//! Owns the shared [`TimerState`] that the bar module watches for display and
//! that the dropdown edits. The D-Bus interface at `com.wayle.Timer1` allows
//! external control from the CLI.

mod dbus;
mod state;

use std::sync::Arc;

use dbus::{SERVICE_NAME, SERVICE_PATH, TimerDaemon};
pub use state::{Granularity, TimerState};
use tracing::info;
use wayle_config::ConfigService;
use zbus::Connection;

/// Timer service providing a D-Bus control interface.
pub struct TimerService {
    state: TimerState,
    _connection: Connection,
}

impl TimerService {
    /// Creates the service and registers its D-Bus interface.
    ///
    /// The initial duration and command are read from `[modules.timer]`.
    pub async fn new(config: Arc<ConfigService>) -> Result<Self, Error> {
        let timer = &config.config().modules.timer;
        let default_duration = u64::from(timer.default_duration.get());
        let command = timer.command.get();

        let state = TimerState::new(default_duration, command);
        state.restore();

        let connection = Connection::session()
            .await
            .map_err(|err| Error::Connection(err.to_string()))?;

        let daemon = TimerDaemon::new(state.clone(), config);

        connection
            .object_server()
            .at(SERVICE_PATH, daemon)
            .await
            .map_err(|err| Error::Registration(err.to_string()))?;

        connection
            .request_name(SERVICE_NAME)
            .await
            .map_err(|err| Error::NameRequest(err.to_string()))?;

        info!("Timer service registered at {SERVICE_NAME}");

        Ok(Self {
            state,
            _connection: connection,
        })
    }

    /// Returns a clone of the shared state for modules to watch.
    pub fn state(&self) -> TimerState {
        self.state.clone()
    }
}

/// Errors from timer service initialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot connect to session bus")]
    Connection(String),

    #[error("cannot register D-Bus object")]
    Registration(String),

    #[error("cannot request D-Bus name")]
    NameRequest(String),
}
