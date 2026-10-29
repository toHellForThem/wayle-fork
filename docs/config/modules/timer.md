---
title: timer
outline: [2, 3]
---

# timer

<div v-pre>

Countdown timer with an interactive dropdown for setting the duration (days,
hours, minutes, seconds) and a Start button.

Controllable from the CLI: `wayle timer start|stop|pause|resume|toggle-pause|reset|set|add|command|status`.

Add it to your layout with `timer`:

```toml
[[bar.layout]]
monitor = "*"
right = ["timer"]
```

## Persistence

The timer is always persisted to `~/.local/state/wayle/timer.json` (or
`$XDG_STATE_HOME/wayle/timer.json`). A running timer stores an absolute
deadline, so it keeps counting across system sleep and is restored when the
shell starts again. A paused timer restores its paused remaining time.

If the deadline passed while the machine was off, the timer is restored as
stopped — the melody and command are **not** run.

## General

| Field | Type | Default | Description |
|---|---|---|---|
| `default-duration` | u32 | `300` | Default duration in seconds, preloaded into the dropdown. |
| `step` | u32 | `10` | Increment/decrement step used by the `<<` and `>>` buttons. |
| `format-mode` | enum | `"largest-to-smallest"` | Bar label formatting. `largest-to-smallest` normalizes to the largest unit (`1h 5m`); `as-set` keeps the configured granularity (`66:00`, then `65:59`, …). |
| `format` | string | `"{{ remaining }}"` | Template for the bar label. |
| `show-days` | bool | `true` | Show the days field in the dropdown. |
| `show-hours` | bool | `true` | Show the hours field in the dropdown. |
| `show-minutes` | bool | `true` | Show the minutes field in the dropdown. |
| `show-seconds` | bool | `true` | Show the seconds field in the dropdown. |
| `show-command` | bool | `false` | Show the command field in the dropdown. |
| `play-melody` | bool | `true` | Play the configured melody when the timer reaches zero. |
| `melody` | string | `""` | Melody played on expiry, as a shell command (e.g. `paplay /usr/share/sounds/finish.oga`). |
| `run-command` | bool | `false` | Run the configured command when the timer reaches zero. |
| `command` | string | `""` | Command executed on expiry when `run-command` is enabled. |
| `icon-inactive` | string | `"ld-watch-symbolic"` | Icon when the timer is stopped. |
| `icon-active` | string | `"ld-pause-symbolic"` | Icon when the timer is running. |
| `icon-paused` | string | `"ld-play-symbolic"` | Icon when the timer is paused. |
| `border-show` | bool | `false` | Display border around button. |
| `icon-show` | bool | `true` | Display module icon. |
| `label-show` | bool | `true` | Display text label. |
| `label-max-length` | u32 | `0` | Max label characters before truncation with ellipsis. Set to 0 to disable. |

::: details More about `format`

#### Placeholders

- `{{ remaining }}` — remaining time, formatted per `format-mode`
- `{{ state }}` — `Running`, `Paused`, or `Stopped`
- `{{ duration }}` — total configured duration

#### Examples

- `"{{ remaining }}"` — `1h 5m`
- `"{{ state }} {{ remaining }}"` — `Running 1h 5m`
- `"{{ remaining }} / {{ duration }}"` — `4:30 / 5:00`

:::

::: details More about `format-mode`

- `largest-to-smallest` (default) — biggest unit first, leading zero units
  omitted. `1d 2h 3m 4s`, `5m 3s`, `10s`.
- `as-set` — keep the granularity the timer was configured with. A duration set
  as 66 minutes shows `66:00`, then `65:59`, `65:58`, …

:::

## Colors

| Field | Type | Default | Description |
|---|---|---|---|
| `border-color` | [`ColorValue`](/config/types#color-value) | `"yellow"` | Border color token. |
| `icon-color` | [`ColorValue`](/config/types#color-value) | `"auto"` | Icon foreground color. Auto selects based on variant for contrast. |
| `icon-bg-color` | [`ColorValue`](/config/types#color-value) | `"yellow"` | Icon container background color token. |
| `label-color` | [`ColorValue`](/config/types#color-value) | `"yellow"` | Label text color token. |
| `button-bg-color` | [`ColorValue`](/config/types#color-value) | `"bg-surface-elevated"` | Button background color token. |

## Click actions

| Field | Type | Default | Description |
|---|---|---|---|
| `left-click` | [`ClickAction`](/config/types#click-action) | `"dropdown:timer"` | Action on left click. Default opens the timer dropdown. |
| `right-click` | [`ClickAction`](/config/types#click-action) | `"wayle timer stop"` | Action on right click. |
| `middle-click` | [`ClickAction`](/config/types#click-action) | `"wayle timer toggle-pause"` | Action on middle click. |
| `scroll-up` | [`ClickAction`](/config/types#click-action) | `""` | Action on scroll up. |
| `scroll-down` | [`ClickAction`](/config/types#click-action) | `""` | Action on scroll down. |

## Default configuration

```toml
[modules.timer]
default-duration = 300
step = 10
format-mode = "largest-to-smallest"
format = "{{ remaining }}"
show-days = true
show-hours = true
show-minutes = true
show-seconds = true
show-command = false
play-melody = true
melody = ""
run-command = false
command = ""
icon-inactive = "ld-watch-symbolic"
icon-active = "ld-pause-symbolic"
icon-paused = "ld-play-symbolic"
border-show = false
border-color = "yellow"
icon-show = true
icon-color = "auto"
icon-bg-color = "yellow"
label-show = true
label-color = "yellow"
label-max-length = 0
button-bg-color = "bg-surface-elevated"
left-click = "dropdown:timer"
right-click = "wayle timer stop"
middle-click = "wayle timer toggle-pause"
scroll-up = ""
scroll-down = ""
```

</div>
