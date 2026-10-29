use serde_json::json;
use wayle_config::schemas::modules::TimerFormatMode;

use crate::{i18n::t, services::timer::Granularity};

/// Formats a duration in seconds for the `as-set` mode.
fn format_as_set(total_secs: u64, granularity: Granularity) -> String {
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let minutes = (total_secs % 3_600) / 60;
    let seconds = total_secs % 60;

    match granularity {
        Granularity::Days => format!("{days}d {hours:02}:{minutes:02}:{seconds:02}"),
        Granularity::Hours => {
            let total_hours = total_secs / 3_600;
            format!("{total_hours}:{minutes:02}:{seconds:02}")
        }
        Granularity::Minutes => {
            let total_minutes = total_secs / 60;
            format!("{total_minutes}:{seconds:02}")
        }
        Granularity::Seconds => format!("{total_secs}"),
    }
}

/// Formats a duration normalizing to the largest unit, biggest → smallest.
///
/// Leading zero units are omitted; lower units are kept down to the last
/// non-zero component (e.g. `1h 0m 5s`).
fn format_largest_to_smallest(total_secs: u64) -> String {
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let minutes = (total_secs % 3_600) / 60;
    let seconds = total_secs % 60;

    let parts = [(days, "d"), (hours, "h"), (minutes, "m"), (seconds, "s")];

    let first_nonzero = parts.iter().position(|(value, _)| *value > 0);
    let Some(first) = first_nonzero else {
        return String::from("0s");
    };

    let last_nonzero = parts
        .iter()
        .rposition(|(value, _)| *value > 0)
        .unwrap_or(first);

    parts[first..=last_nonzero]
        .iter()
        .map(|(value, unit)| format!("{value}{unit}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Formats a duration according to the selected format mode.
pub(crate) fn format_duration(
    total_secs: u64,
    mode: TimerFormatMode,
    granularity: Granularity,
) -> String {
    match mode {
        TimerFormatMode::LargestToSmallest => format_largest_to_smallest(total_secs),
        TimerFormatMode::AsSet => format_as_set(total_secs, granularity),
    }
}

/// State text for the `state` template placeholder.
fn state_text(running: bool, paused: bool) -> String {
    if paused {
        t!("bar-timer-paused")
    } else if running {
        t!("bar-timer-running")
    } else {
        t!("bar-timer-stopped")
    }
}

/// Context passed to the label template.
pub(crate) struct LabelContext {
    pub running: bool,
    pub paused: bool,
    pub remaining_secs: Option<u64>,
    pub duration_secs: u64,
    pub format_mode: TimerFormatMode,
    pub granularity: Granularity,
}

/// Renders the bar label template.
pub(super) fn build_label(format: &str, ctx: &LabelContext) -> String {
    let remaining_secs = ctx.remaining_secs.unwrap_or(ctx.duration_secs);
    let remaining = format_duration(remaining_secs, ctx.format_mode, ctx.granularity);
    let duration = format_duration(ctx.duration_secs, ctx.format_mode, ctx.granularity);
    let state = state_text(ctx.running, ctx.paused);

    let template_ctx = json!({
        "state": state,
        "remaining": remaining,
        "duration": duration,
    });

    crate::template::render(format, template_ctx).unwrap_or_default()
}

/// Selects the icon for the current timer state.
pub(super) fn select_icon(
    running: bool,
    paused: bool,
    icon_inactive: &str,
    icon_active: &str,
    icon_paused: &str,
) -> String {
    if paused {
        icon_paused.to_string()
    } else if running {
        icon_active.to_string()
    } else {
        icon_inactive.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_to_smallest_basic() {
        assert_eq!(format_largest_to_smallest(0), "0s");
        assert_eq!(format_largest_to_smallest(5), "5s");
        assert_eq!(format_largest_to_smallest(65), "1m 5s");
        assert_eq!(format_largest_to_smallest(3_661), "1h 1m 1s");
        assert_eq!(format_largest_to_smallest(90_061), "1d 1h 1m 1s");
    }

    #[test]
    fn largest_to_smallest_keeps_inner_zero_units() {
        assert_eq!(format_largest_to_smallest(3_605), "1h 0m 5s");
        assert_eq!(format_largest_to_smallest(3_600), "1h");
    }

    #[test]
    fn as_set_minutes_example() {
        // 66 minutes = 3960 seconds.
        assert_eq!(format_as_set(3_960, Granularity::Minutes), "66:00");
        assert_eq!(format_as_set(3_959, Granularity::Minutes), "65:59");
    }

    #[test]
    fn as_set_hours_and_days() {
        assert_eq!(format_as_set(3_661, Granularity::Hours), "1:01:01");
        assert_eq!(format_as_set(90_061, Granularity::Days), "1d 01:01:01");
    }

    #[test]
    fn state_text_variants() {
        assert_eq!(state_text(false, false), "Stopped");
        assert_eq!(state_text(false, true), "Paused");
        assert_eq!(state_text(true, false), "Running");
    }
}
