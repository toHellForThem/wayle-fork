use relm4::ComponentSender;
use wayle_config::schemas::modules::TimerConfig;
use wayle_widgets::watch;

use super::{TimerModule, messages::TimerCmd};
use crate::services::timer::TimerState;

pub(super) fn spawn_config_watchers(sender: &ComponentSender<TimerModule>, config: &TimerConfig) {
    let format = config.format.clone();
    let format_mode = config.format_mode.clone();
    let icon_inactive = config.icon_inactive.clone();
    let icon_active = config.icon_active.clone();
    let icon_paused = config.icon_paused.clone();

    watch!(
        sender,
        [
            format.watch(),
            format_mode.watch(),
            icon_inactive.watch(),
            icon_active.watch(),
            icon_paused.watch()
        ],
        |out| {
            let _ = out.send(TimerCmd::ConfigChanged);
        }
    );
}

pub(super) fn spawn_state_watchers(sender: &ComponentSender<TimerModule>, state: &TimerState) {
    let running = state.running.clone();
    let paused = state.paused.clone();
    let duration_secs = state.duration_secs.clone();
    let remaining_secs = state.remaining_secs.clone();
    let granularity = state.granularity.clone();

    watch!(
        sender,
        [
            running.watch(),
            paused.watch(),
            duration_secs.watch(),
            remaining_secs.watch(),
            granularity.watch()
        ],
        |out| {
            let _ = out.send(TimerCmd::StateChanged);
        }
    );
}
