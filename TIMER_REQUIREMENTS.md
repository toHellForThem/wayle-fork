# Timer module for Wayle — requirements

Countdown timer integrated into the Wayle bar (Rust / GTK4 / Relm4).
Lives inside the Wayle fork at `timer-for-wayle`.

## Type

- Countdown timer (обратный отсчёт): set a duration, press start, it counts down to zero.

## Bar module (panel, not dropdown)

- Shows the remaining time.
- **Default formatting: largest unit → smallest unit.** Leading zero units are
  omitted, e.g. `1d 2h 3m 4s`, `5m 3s`, `10s`.
- **Configurable formatting mode**, selectable in settings:
  - `largest-to-smallest` (default) — normalize to the largest unit, as above.
  - `as-set` — keep the granularity that was configured. Example: a duration set
    as 66 minutes shows `66:00`, then `65:59`, `65:58`, … (no promotion to hours).
- Default icons: inactive = `ld-watch-symbolic`, active = `ld-pause-symbolic`,
  paused = `ld-play-symbolic`.

## Persistence

- The timer is **always** persisted to `~/.local/state/wayle/timer.json`
  (`$XDG_STATE_HOME/wayle/timer.json`).
- A running timer stores an absolute deadline, so it keeps counting across
  system sleep and is restored when the shell starts again.
- A paused timer restores its paused remaining time.
- If the deadline passed while the machine was off, the timer is restored as
  **stopped** — the melody and command are **not** run.

## Dropdown (popover opened from the bar button)

Duration is edited with independent fields for **days, hours, minutes, seconds**.

- Each unit is a **vertical block**: the unit label on top, then a row
  `<< < value > >>` below it.
- Buttons are large (~36×36) and use chevron icons; `<<` / `>>` are drawn as
  two chevrons.
- `<<` / `>>` adjust by the configured step (e.g. 10); `<` / `>` adjust by 1.
- A **Start** button.
- A **Pause/Play** button that toggles its label with state: `Pause` while
  running, `Play` while paused. After Start/Play it shows `Pause`.
- A **Reset** button that stops the timer and resets it to the configured
  `default-duration`.
- Fields shown in the dropdown are controlled by settings toggles. When every
  toggle is enabled the dropdown shows, in this order:
  **command, seconds, minutes, hours, days**.
- The **command** field (when enabled) is a text field holding the command to run
  on expiry.

## Settings (config, NOT the dropdown)

- `step` — increment/decrement step used by `<<` / `>>` buttons (e.g. 10).
- **Melody** — the sound/melody played when the timer reaches zero (configurable).
- **Flag to run a command on expiry** — an on/off switch in settings (not in the
  dropdown). The command text itself may be shown/edited in the dropdown when the
  command field is enabled.
- Toggles for which dropdown fields are visible: `days`, `hours`, `minutes`,
  `seconds`, `command`.
- Formatting mode for the bar label: `largest-to-smallest` or `as-set`.
- Standard bar-button config fields (icon, colors, click actions, etc.).

## Behavior notes

- Timer states: stopped, running, paused.
- Controls available from CLI (`wayle timer ...`) and dropdown; click actions
  configurable.
- On expiry: play the configured melody, and if the run-command flag is enabled,
  execute the command.
