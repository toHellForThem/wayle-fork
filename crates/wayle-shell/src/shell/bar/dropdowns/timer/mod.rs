mod factory;
mod messages;

use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*};
use wayle_config::ConfigService;
use wayle_widgets::watch;

pub(super) use self::factory::Factory;
use self::messages::{TimerDropdownCmd, TimerDropdownInit, TimerDropdownInput, TimerField};
use crate::{
    i18n::t,
    services::timer::{Granularity, TimerState},
    shell::bar::dropdowns::scaled_dimension,
};

const BASE_WIDTH: f32 = 320.0;
const STEP_BUTTON_SIZE: i32 = 36;

pub(crate) struct TimerDropdown {
    state: TimerState,
    config: Arc<ConfigService>,
    popover: gtk::Popover,
    scaled_width: i32,

    show_days: bool,
    show_hours: bool,
    show_minutes: bool,
    show_seconds: bool,
    show_command: bool,

    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,

    days_value: gtk::Label,
    hours_value: gtk::Label,
    minutes_value: gtk::Label,
    seconds_value: gtk::Label,
    command_entry: gtk::Entry,
    pause_button: gtk::Button,
}

#[relm4::component(pub(crate))]
impl Component for TimerDropdown {
    type Init = TimerDropdownInit;
    type Input = TimerDropdownInput;
    type Output = ();
    type CommandOutput = TimerDropdownCmd;

    view! {
        #[root]
        gtk::Popover {
            set_css_classes: &["dropdown", "timer-dropdown"],
            set_has_arrow: false,

            #[watch]
            set_width_request: model.scaled_width,

            #[wrap(Some)]
            #[name = "container"]
            set_child = &gtk::Box {
                add_css_class: "dropdown",
                set_orientation: gtk::Orientation::Vertical,
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let state = init.timer.state();
        let config = init.config.config();
        let timer = &config.modules.timer;

        let step = i64::from(timer.step.get().max(1));
        let show_days = timer.show_days.get();
        let show_hours = timer.show_hours.get();
        let show_minutes = timer.show_minutes.get();
        let show_seconds = timer.show_seconds.get();
        let show_command = timer.show_command.get();
        let scale = config.styling.scale.get().value();

        let (days_block, days_value) =
            build_field_block(&t!("dropdown-timer-days"), step, TimerField::Days, &sender);
        let (hours_block, hours_value) = build_field_block(
            &t!("dropdown-timer-hours"),
            step,
            TimerField::Hours,
            &sender,
        );
        let (minutes_block, minutes_value) = build_field_block(
            &t!("dropdown-timer-minutes"),
            step,
            TimerField::Minutes,
            &sender,
        );
        let (seconds_block, seconds_value) = build_field_block(
            &t!("dropdown-timer-seconds"),
            step,
            TimerField::Seconds,
            &sender,
        );

        let command_entry = gtk::Entry::new();
        command_entry.set_placeholder_text(Some(&t!("dropdown-timer-command")));
        let cmd_sender = sender.clone();
        command_entry.connect_changed(move |entry| {
            cmd_sender.input(TimerDropdownInput::CommandChanged(entry.text().to_string()));
        });

        let (footer, pause_button) = build_footer(&sender);

        let popover = root.clone();
        let sync_sender = sender.clone();
        root.connect_map(move |_| {
            sync_sender.input(TimerDropdownInput::SyncFromState);
        });

        let mut model = Self {
            state,
            config: init.config,
            popover,
            scaled_width: scaled_dimension(BASE_WIDTH, scale),
            show_days,
            show_hours,
            show_minutes,
            show_seconds,
            show_command,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            days_value,
            hours_value,
            minutes_value,
            seconds_value,
            command_entry,
            pause_button,
        };

        model.sync_from_state();
        model.refresh_pause_button();

        let running = model.state.running.clone();
        let paused = model.state.paused.clone();
        watch!(sender, [running.watch(), paused.watch()], |out| {
            let _ = out.send(TimerDropdownCmd::StateChanged);
        });

        let widgets = view_output!();

        let container = &widgets.container;
        container.append(&build_header());
        container.append(&build_content(
            &model,
            &days_block,
            &hours_block,
            &minutes_block,
            &seconds_block,
        ));
        container.append(&footer);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match msg {
            TimerDropdownInput::Adjust { field, amount } => self.adjust(field, amount),
            TimerDropdownInput::CommandChanged(command) => self.state.set_command(command),
            TimerDropdownInput::Start => {
                self.start();
                self.refresh_pause_button();
            }
            TimerDropdownInput::TogglePause => {
                self.state.toggle_pause();
                self.refresh_pause_button();
            }
            TimerDropdownInput::Reset => {
                self.reset();
                self.refresh_pause_button();
            }
            TimerDropdownInput::SyncFromState => {
                self.sync_from_state();
                self.refresh_pause_button();
            }
        }
    }

    fn update_cmd(
        &mut self,
        msg: TimerDropdownCmd,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            TimerDropdownCmd::StateChanged => self.refresh_pause_button(),
        }
    }
}

impl TimerDropdown {
    fn split_duration(duration: u64, granularity: Granularity) -> (u32, u32, u32, u32) {
        let days = (duration / 86_400) as u32;
        let hours = ((duration % 86_400) / 3_600) as u32;
        let minutes = ((duration % 3_600) / 60) as u32;
        let seconds = (duration % 60) as u32;

        match granularity {
            Granularity::Days => (days, hours, minutes, seconds),
            Granularity::Hours => (0, (duration / 3_600) as u32, minutes, seconds),
            Granularity::Minutes => (0, 0, (duration / 60) as u32, seconds),
            Granularity::Seconds => (0, 0, 0, duration as u32),
        }
    }

    fn sync_from_state(&mut self) {
        let duration = self.state.duration_secs.get();
        let (days, hours, minutes, seconds) =
            Self::split_duration(duration, self.state.granularity.get());

        self.days = days;
        self.hours = hours;
        self.minutes = minutes;
        self.seconds = seconds;
        self.refresh_values();

        let command = self.state.command.get();
        if self.command_entry.text().as_str() != command.as_str() {
            self.command_entry.set_text(&command);
        }
    }

    fn refresh_values(&self) {
        self.days_value.set_label(&self.days.to_string());
        self.hours_value.set_label(&self.hours.to_string());
        self.minutes_value.set_label(&self.minutes.to_string());
        self.seconds_value.set_label(&self.seconds.to_string());
    }

    fn refresh_pause_button(&self) {
        let label = if self.state.paused.get() {
            t!("dropdown-timer-play")
        } else {
            t!("dropdown-timer-pause")
        };
        self.pause_button.set_label(&label);
    }

    fn adjust(&mut self, field: TimerField, amount: i64) {
        match field {
            TimerField::Days => {
                self.days = clamp_step(self.days, amount, u32::MAX);
            }
            TimerField::Hours => {
                let max = if self.show_days { 23 } else { u32::MAX };
                self.hours = clamp_step(self.hours, amount, max);
            }
            TimerField::Minutes => {
                self.minutes = clamp_step(self.minutes, amount, 59);
            }
            TimerField::Seconds => {
                self.seconds = clamp_step(self.seconds, amount, 59);
            }
        }

        self.refresh_values();
        self.state
            .set_time_parts(self.days, self.hours, self.minutes, self.seconds);
    }

    fn start(&self) {
        let config = self.config.config();
        let timer = &config.modules.timer;

        self.state.start_current(
            timer.play_melody.get(),
            timer.melody.get(),
            timer.run_command.get(),
            self.command_entry.text().to_string(),
        );
        self.popover.popdown();
    }

    fn reset(&mut self) {
        let default = u64::from(self.config.config().modules.timer.default_duration.get());
        self.state.reset_to(default);
        self.sync_from_state();
    }
}

fn clamp_step(current: u32, amount: i64, max: u32) -> u32 {
    let next = i64::from(current) + amount;
    next.clamp(0, i64::from(max)) as u32
}

fn build_header() -> gtk::Box {
    let header = gtk::Box::default();
    header.add_css_class("dropdown-header");

    let title = gtk::Box::default();
    title.add_css_class("dropdown-title");
    title.set_hexpand(true);

    let icon = gtk::Image::from_icon_name("ld-watch-symbolic");
    icon.set_visible(true);

    let label = gtk::Label::new(Some(&t!("dropdown-timer-title")));

    title.append(&icon);
    title.append(&label);
    header.append(&title);

    header
}

fn build_content(
    model: &TimerDropdown,
    days_block: &gtk::Box,
    hours_block: &gtk::Box,
    minutes_block: &gtk::Box,
    seconds_block: &gtk::Box,
) -> gtk::Box {
    let content = gtk::Box::default();
    content.set_orientation(gtk::Orientation::Vertical);
    content.set_spacing(10);
    content.add_css_class("dropdown-content");

    if model.show_days {
        content.append(days_block);
    }
    if model.show_hours {
        content.append(hours_block);
    }
    if model.show_minutes {
        content.append(minutes_block);
    }
    if model.show_seconds {
        content.append(seconds_block);
    }
    if model.show_command {
        content.append(&model.command_entry);
    }

    content
}

fn build_footer(sender: &ComponentSender<TimerDropdown>) -> (gtk::Box, gtk::Button) {
    let footer = gtk::Box::default();
    footer.add_css_class("dropdown-footer");
    footer.set_spacing(6);

    let start = gtk::Button::with_label(&t!("dropdown-timer-start"));
    start.add_css_class("primary");
    start.set_hexpand(true);
    start.set_cursor_from_name(Some("pointer"));
    let start_sender = sender.clone();
    start.connect_clicked(move |_| {
        start_sender.input(TimerDropdownInput::Start);
    });

    let pause = gtk::Button::with_label(&t!("dropdown-timer-pause"));
    pause.add_css_class("ghost");
    pause.set_cursor_from_name(Some("pointer"));
    let pause_sender = sender.clone();
    pause.connect_clicked(move |_| {
        pause_sender.input(TimerDropdownInput::TogglePause);
    });

    let reset = gtk::Button::with_label(&t!("dropdown-timer-reset"));
    reset.add_css_class("ghost");
    reset.set_cursor_from_name(Some("pointer"));
    let reset_sender = sender.clone();
    reset.connect_clicked(move |_| {
        reset_sender.input(TimerDropdownInput::Reset);
    });

    footer.append(&start);
    footer.append(&pause);
    footer.append(&reset);

    (footer, pause)
}

fn build_field_block(
    name: &str,
    step: i64,
    field: TimerField,
    sender: &ComponentSender<TimerDropdown>,
) -> (gtk::Box, gtk::Label) {
    let block = gtk::Box::default();
    block.set_orientation(gtk::Orientation::Vertical);
    block.set_spacing(4);
    block.add_css_class("timer-field");

    let label = gtk::Label::new(Some(name));
    label.add_css_class("timer-field-label");
    label.set_halign(gtk::Align::Center);

    let row = gtk::Box::default();
    row.set_orientation(gtk::Orientation::Horizontal);
    row.set_spacing(6);
    row.set_halign(gtk::Align::Center);

    let dec_step = step_button(
        &["ld-chevron-left-symbolic", "ld-chevron-left-symbolic"],
        field,
        -step,
        sender,
    );
    let dec_one = step_button(&["ld-chevron-left-symbolic"], field, -1, sender);

    let value = gtk::Label::new(Some("0"));
    value.set_width_chars(4);
    value.add_css_class("timer-value");
    value.add_css_class("title-4");

    let inc_one = step_button(&["ld-chevron-right-symbolic"], field, 1, sender);
    let inc_step = step_button(
        &["ld-chevron-right-symbolic", "ld-chevron-right-symbolic"],
        field,
        step,
        sender,
    );

    row.append(&dec_step);
    row.append(&dec_one);
    row.append(&value);
    row.append(&inc_one);
    row.append(&inc_step);

    block.append(&label);
    block.append(&row);

    (block, value)
}

fn step_button(
    icons: &[&str],
    field: TimerField,
    amount: i64,
    sender: &ComponentSender<TimerDropdown>,
) -> gtk::Button {
    let button = gtk::Button::new();
    button.add_css_class("icon");
    button.add_css_class("timer-step");
    button.set_size_request(STEP_BUTTON_SIZE, STEP_BUTTON_SIZE);
    button.set_cursor_from_name(Some("pointer"));

    let content = gtk::Box::default();
    content.set_orientation(gtk::Orientation::Horizontal);
    content.set_spacing(0);
    content.set_halign(gtk::Align::Center);
    content.set_valign(gtk::Align::Center);

    for icon in icons {
        content.append(&gtk::Image::from_icon_name(icon));
    }
    button.set_child(Some(&content));

    let sender = sender.clone();
    button.connect_clicked(move |_| {
        sender.input(TimerDropdownInput::Adjust { field, amount });
    });

    button
}
