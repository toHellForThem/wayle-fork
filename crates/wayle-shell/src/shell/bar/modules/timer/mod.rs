mod factory;
mod helpers;
mod messages;
mod watchers;

use std::{rc::Rc, sync::Arc};

use gtk::prelude::*;
use relm4::prelude::*;
use wayle_config::{
    ConfigProperty, ConfigService,
    schemas::{modules::TimerConfig, styling::CssToken},
};
use wayle_widgets::{
    prelude::{
        BarButton, BarButtonBehavior, BarButtonColors, BarButtonInit, BarButtonInput,
        BarButtonOutput,
    },
    utils::force_window_resize,
};

pub(crate) use self::{
    factory::Factory,
    messages::{TimerCmd, TimerInit, TimerMsg},
};
use crate::{
    services::timer::TimerState,
    shell::bar::dropdowns::{self, DropdownRegistry},
};

pub(crate) struct TimerModule {
    bar_button: Controller<BarButton>,
    config: Arc<ConfigService>,
    state: TimerState,
    dropdowns: Rc<DropdownRegistry>,
    last_label_len: usize,
}

#[relm4::component(pub(crate))]
impl Component for TimerModule {
    type Init = TimerInit;
    type Input = TimerMsg;
    type Output = ();
    type CommandOutput = TimerCmd;

    view! {
        gtk::Box {
            add_css_class: "timer",

            #[local_ref]
            bar_button -> gtk::MenuButton {},
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let config = init.config.config();
        let timer = &config.modules.timer;
        let state = init.timer.state();

        let running = state.running.get();
        let paused = state.paused.get();
        let icon = helpers::select_icon(
            running,
            paused,
            &timer.icon_inactive.get(),
            &timer.icon_active.get(),
            &timer.icon_paused.get(),
        );
        let label = helpers::build_label(
            &timer.format.get(),
            &helpers::LabelContext {
                running,
                paused,
                remaining_secs: state.remaining_secs.get(),
                duration_secs: state.duration_secs.get(),
                format_mode: timer.format_mode.get(),
                granularity: state.granularity.get(),
            },
        );
        let initial_label_len = label.chars().count();

        let bar_button = BarButton::builder()
            .launch(BarButtonInit {
                icon,
                label,
                tooltip: None,
                colors: BarButtonColors {
                    icon_color: timer.icon_color.clone(),
                    label_color: timer.label_color.clone(),
                    icon_background: timer.icon_bg_color.clone(),
                    button_background: timer.button_bg_color.clone(),
                    border_color: timer.border_color.clone(),
                    auto_icon_color: CssToken::Yellow,
                },
                behavior: BarButtonBehavior {
                    label_max_chars: timer.label_max_length.clone(),
                    show_icon: timer.icon_show.clone(),
                    show_label: timer.label_show.clone(),
                    show_border: timer.border_show.clone(),
                    visible: ConfigProperty::new(true),
                },
                settings: init.settings,
            })
            .forward(sender.input_sender(), |output| match output {
                BarButtonOutput::LeftClick => TimerMsg::LeftClick,
                BarButtonOutput::RightClick => TimerMsg::RightClick,
                BarButtonOutput::MiddleClick => TimerMsg::MiddleClick,
                BarButtonOutput::ScrollUp => TimerMsg::ScrollUp,
                BarButtonOutput::ScrollDown => TimerMsg::ScrollDown,
            });

        watchers::spawn_config_watchers(&sender, timer);
        watchers::spawn_state_watchers(&sender, &state);

        let model = Self {
            bar_button,
            config: init.config,
            state,
            dropdowns: init.dropdowns,
            last_label_len: initial_label_len,
        };
        let bar_button = model.bar_button.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let timer = &self.config.config().modules.timer;

        let action = match msg {
            TimerMsg::LeftClick => timer.left_click.get(),
            TimerMsg::RightClick => timer.right_click.get(),
            TimerMsg::MiddleClick => timer.middle_click.get(),
            TimerMsg::ScrollUp => timer.scroll_up.get(),
            TimerMsg::ScrollDown => timer.scroll_down.get(),
        };

        dropdowns::dispatch_click(&action, &self.dropdowns, &self.bar_button);
    }

    fn update_cmd(&mut self, msg: TimerCmd, _sender: ComponentSender<Self>, root: &Self::Root) {
        match msg {
            TimerCmd::ConfigChanged | TimerCmd::StateChanged => {
                let timer = self.config.config().modules.timer.clone();
                self.update_display(&timer, root);
            }
        }
    }
}

impl TimerModule {
    fn update_display(&mut self, config: &TimerConfig, root: &gtk::Box) {
        let running = self.state.running.get();
        let paused = self.state.paused.get();

        let icon = helpers::select_icon(
            running,
            paused,
            &config.icon_inactive.get(),
            &config.icon_active.get(),
            &config.icon_paused.get(),
        );
        self.bar_button.emit(BarButtonInput::SetIcon(icon));

        let label = helpers::build_label(
            &config.format.get(),
            &helpers::LabelContext {
                running,
                paused,
                remaining_secs: self.state.remaining_secs.get(),
                duration_secs: self.state.duration_secs.get(),
                format_mode: config.format_mode.get(),
                granularity: self.state.granularity.get(),
            },
        );

        let new_len = label.chars().count();
        self.bar_button.emit(BarButtonInput::SetLabel(label));
        if new_len != self.last_label_len {
            self.last_label_len = new_len;
            force_window_resize(root);
        }
    }
}
