use std::{rc::Rc, sync::Arc};

use wayle_config::ConfigService;
use wayle_widgets::prelude::BarSettings;

use crate::{services::timer::TimerService, shell::bar::dropdowns::DropdownRegistry};

pub(crate) struct TimerInit {
    pub settings: BarSettings,
    pub timer: Arc<TimerService>,
    pub config: Arc<ConfigService>,
    pub dropdowns: Rc<DropdownRegistry>,
}

#[derive(Debug)]
pub(crate) enum TimerMsg {
    LeftClick,
    RightClick,
    MiddleClick,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug)]
pub(crate) enum TimerCmd {
    ConfigChanged,
    StateChanged,
}
