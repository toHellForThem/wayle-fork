use schemars::schema_for;
use wayle_derive::{wayle_config, wayle_enum};

use crate::{
    ClickAction, ConfigProperty,
    docs::{ConfigGroup, GroupDefaults, ModuleInfo, ModuleInfoProvider},
    schemas::styling::{ColorValue, CssToken},
};

/// How the remaining time is rendered in the bar label.
#[derive(Hash)]
#[wayle_enum(default)]
pub enum TimerFormatMode {
    /// Normalize to the largest sensible unit, biggest → smallest
    /// (e.g. `1h 5m`, `5m 3s`). Leading zero units are omitted.
    #[default]
    LargestToSmallest,
    /// Keep the granularity the timer was configured with. A duration set as
    /// 66 minutes shows `66:00`, then `65:59`, `65:58`, … without promoting
    /// to hours.
    AsSet,
}

/// Countdown timer with an interactive dropdown for setting the duration.
///
/// Controllable from the CLI: `wayle timer start|stop|pause|resume|toggle-pause|reset|status`.
#[wayle_config(bar_button, i18n_prefix = "settings-modules-timer")]
pub struct TimerConfig {
    /// Default timer duration in seconds, preloaded into the dropdown.
    #[serde(rename = "default-duration")]
    #[default(300)]
    pub default_duration: ConfigProperty<u32>,

    /// Increment/decrement step used by the `<<` and `>>` buttons.
    #[serde(rename = "step")]
    #[default(10)]
    pub step: ConfigProperty<u32>,

    /// How the remaining time is formatted in the bar label.
    #[serde(rename = "format-mode")]
    #[default(TimerFormatMode::LargestToSmallest)]
    pub format_mode: ConfigProperty<TimerFormatMode>,

    /// Template for the bar label.
    ///
    /// ## Placeholders
    ///
    /// - `{{ remaining }}` — remaining time, formatted per `format-mode`
    /// - `{{ state }}` — `Running`, `Paused`, or `Stopped`
    /// - `{{ duration }}` — total configured duration
    ///
    /// ## Examples
    ///
    /// - `"{{ remaining }}"` — `1h 5m`
    /// - `"{{ state }} {{ remaining }}"` — `Running 1h 5m`
    #[serde(rename = "format")]
    #[default(String::from("{{ remaining }}"))]
    pub format: ConfigProperty<String>,

    /// Show the days field in the dropdown.
    #[serde(rename = "show-days")]
    #[default(true)]
    pub show_days: ConfigProperty<bool>,

    /// Show the hours field in the dropdown.
    #[serde(rename = "show-hours")]
    #[default(true)]
    pub show_hours: ConfigProperty<bool>,

    /// Show the minutes field in the dropdown.
    #[serde(rename = "show-minutes")]
    #[default(true)]
    pub show_minutes: ConfigProperty<bool>,

    /// Show the seconds field in the dropdown.
    #[serde(rename = "show-seconds")]
    #[default(true)]
    pub show_seconds: ConfigProperty<bool>,

    /// Show the command field in the dropdown.
    #[serde(rename = "show-command")]
    #[default(false)]
    pub show_command: ConfigProperty<bool>,

    /// Play the configured melody when the timer reaches zero.
    #[serde(rename = "play-melody")]
    #[default(true)]
    pub play_melody: ConfigProperty<bool>,

    /// Melody played on expiry. A shell command, e.g.
    /// `paplay /usr/share/sounds/finish.oga`. Empty disables playback.
    #[serde(rename = "melody")]
    #[default(String::from(""))]
    pub melody: ConfigProperty<String>,

    /// Run the configured command when the timer reaches zero.
    #[serde(rename = "run-command")]
    #[default(false)]
    pub run_command: ConfigProperty<bool>,

    /// Command executed on expiry when `run-command` is enabled.
    #[serde(rename = "command")]
    #[default(String::from(""))]
    pub command: ConfigProperty<String>,

    /// Icon when the timer is stopped.
    #[serde(rename = "icon-inactive")]
    #[default(String::from("ld-watch-symbolic"))]
    pub icon_inactive: ConfigProperty<String>,

    /// Icon when the timer is running.
    #[serde(rename = "icon-active")]
    #[default(String::from("ld-pause-symbolic"))]
    pub icon_active: ConfigProperty<String>,

    /// Icon when the timer is paused.
    #[serde(rename = "icon-paused")]
    #[default(String::from("ld-play-symbolic"))]
    pub icon_paused: ConfigProperty<String>,

    /// Display border around button.
    #[serde(rename = "border-show")]
    #[default(false)]
    pub border_show: ConfigProperty<bool>,

    /// Border color token.
    #[serde(rename = "border-color")]
    #[default(ColorValue::Token(CssToken::Yellow))]
    pub border_color: ConfigProperty<ColorValue>,

    /// Display module icon.
    #[serde(rename = "icon-show")]
    #[default(true)]
    pub icon_show: ConfigProperty<bool>,

    /// Icon foreground color. Auto selects based on variant for contrast.
    #[serde(rename = "icon-color")]
    #[default(ColorValue::Auto)]
    pub icon_color: ConfigProperty<ColorValue>,

    /// Icon container background color token.
    #[serde(rename = "icon-bg-color")]
    #[default(ColorValue::Token(CssToken::Yellow))]
    pub icon_bg_color: ConfigProperty<ColorValue>,

    /// Display text label.
    #[serde(rename = "label-show")]
    #[default(true)]
    pub label_show: ConfigProperty<bool>,

    /// Label text color token.
    #[serde(rename = "label-color")]
    #[default(ColorValue::Token(CssToken::Yellow))]
    pub label_color: ConfigProperty<ColorValue>,

    /// Max label characters before truncation with ellipsis. Set to 0 to disable.
    #[serde(rename = "label-max-length")]
    #[default(0)]
    pub label_max_length: ConfigProperty<u32>,

    /// Button background color token.
    #[serde(rename = "button-bg-color")]
    #[default(ColorValue::Token(CssToken::BgSurfaceElevated))]
    pub button_bg_color: ConfigProperty<ColorValue>,

    /// Action on left click. Default opens the timer dropdown.
    #[serde(rename = "left-click")]
    #[default(ClickAction::Dropdown(String::from("timer")))]
    pub left_click: ConfigProperty<ClickAction>,

    /// Action on right click. Default stops the timer.
    #[serde(rename = "right-click")]
    #[default(ClickAction::Shell(String::from("wayle timer stop")))]
    pub right_click: ConfigProperty<ClickAction>,

    /// Action on middle click. Default pauses/resumes the timer.
    #[serde(rename = "middle-click")]
    #[default(ClickAction::Shell(String::from("wayle timer toggle-pause")))]
    pub middle_click: ConfigProperty<ClickAction>,

    /// Action on scroll up.
    #[serde(rename = "scroll-up")]
    #[default(ClickAction::None)]
    pub scroll_up: ConfigProperty<ClickAction>,

    /// Action on scroll down.
    #[serde(rename = "scroll-down")]
    #[default(ClickAction::None)]
    pub scroll_down: ConfigProperty<ClickAction>,
}

impl ModuleInfoProvider for TimerConfig {
    fn module_info() -> ModuleInfo {
        ModuleInfo {
            name: String::from("timer"),
            schema: || schema_for!(TimerConfig),
            layout_id: Some(String::from("timer")),
            array_entry: false,
        }
    }

    fn groups() -> Vec<ConfigGroup> {
        GroupDefaults::bar_button()
    }
}

crate::register_module!(TimerConfig);
