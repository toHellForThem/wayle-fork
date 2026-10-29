use relm4::prelude::*;

use super::{TimerDropdown, messages::TimerDropdownInit};
use crate::shell::{
    bar::dropdowns::{DropdownFactory, DropdownInstance},
    services::ShellServices,
};

pub(crate) struct Factory;

impl DropdownFactory for Factory {
    fn create(services: &ShellServices) -> Option<DropdownInstance> {
        let init = TimerDropdownInit {
            timer: services.timer.clone(),
            config: services.config.clone(),
        };
        let controller = TimerDropdown::builder().launch(init).detach();

        let popover = controller.widget().clone();
        Some(DropdownInstance::new(popover, Box::new(controller)))
    }
}
