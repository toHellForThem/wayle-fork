//! D-Bus client proxy for the timer service.
#![allow(missing_docs)]

use zbus::{Result, proxy};

/// D-Bus service name for the timer service.
pub const SERVICE_NAME: &str = "com.wayle.Timer1";

/// D-Bus object path for the timer service.
pub const SERVICE_PATH: &str = "/com/wayle/Timer";

#[proxy(
    interface = "com.wayle.Timer1",
    default_service = "com.wayle.Timer1",
    default_path = "/com/wayle/Timer",
    gen_blocking = false
)]
pub trait Timer {
    async fn start(&self, duration_secs: u64) -> Result<()>;

    async fn start_current(&self) -> Result<()>;

    async fn stop(&self) -> Result<()>;

    async fn pause(&self) -> Result<()>;

    async fn resume(&self) -> Result<()>;

    async fn toggle_pause(&self) -> Result<()>;

    async fn reset(&self) -> Result<()>;

    async fn set_duration(&self, secs: u64) -> Result<()>;

    async fn set_time_parts(&self, days: u32, hours: u32, minutes: u32, seconds: u32)
    -> Result<()>;

    async fn add_time(&self, delta_secs: i64) -> Result<()>;

    async fn set_command(&self, command: &str) -> Result<()>;

    #[zbus(property)]
    fn running(&self) -> Result<bool>;

    #[zbus(property)]
    fn paused(&self) -> Result<bool>;

    #[zbus(property)]
    fn remaining(&self) -> Result<u64>;

    #[zbus(property)]
    fn duration(&self) -> Result<u64>;

    #[zbus(property)]
    fn command(&self) -> Result<String>;
}
