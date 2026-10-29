### Wayle Configuration - Timer Module

## Timer Module Configuration

settings-modules-timer-default-duration = Default Duration
    .description = Default timer duration in seconds, preloaded into the dropdown

settings-modules-timer-step = Step
    .description = Increment/decrement step used by the double-arrow buttons

settings-modules-timer-format-mode = Format Mode
    .description = How the remaining time is formatted in the bar label

settings-modules-timer-format = Format
    .description = Template for the bar label using {"{{ remaining }}"}, {"{{ state }}"}, {"{{ duration }}"}

settings-modules-timer-show-days = Show Days
    .description = Show the days field in the dropdown

settings-modules-timer-show-hours = Show Hours
    .description = Show the hours field in the dropdown

settings-modules-timer-show-minutes = Show Minutes
    .description = Show the minutes field in the dropdown

settings-modules-timer-show-seconds = Show Seconds
    .description = Show the seconds field in the dropdown

settings-modules-timer-show-command = Show Command
    .description = Show the command field in the dropdown

settings-modules-timer-play-melody = Play Melody
    .description = Play the configured melody when the timer reaches zero

settings-modules-timer-melody = Melody
    .description = Melody played on expiry, as a shell command

settings-modules-timer-run-command = Run Command
    .description = Run the configured command when the timer reaches zero

settings-modules-timer-command = Command
    .description = Command executed on expiry when run-command is enabled

settings-modules-timer-icon-inactive = Inactive Icon
    .description = Icon when the timer is stopped

settings-modules-timer-icon-active = Active Icon
    .description = Icon when the timer is running

settings-modules-timer-icon-paused = Paused Icon
    .description = Icon when the timer is paused

settings-modules-timer-border-show = Show Border
    .description = Display border around button

settings-modules-timer-border-color = Border Color
    .description = Border color token

settings-modules-timer-icon-show = Show Icon
    .description = Display module icon

settings-modules-timer-icon-color = Icon Color
    .description = Icon foreground color

settings-modules-timer-icon-bg-color = Icon Background
    .description = Icon container background color

settings-modules-timer-label-show = Show Label
    .description = Display text label

settings-modules-timer-label-color = Label Color
    .description = Label text color

settings-modules-timer-label-max-length = Label Max Length
    .description = Max characters before truncation

settings-modules-timer-button-bg-color = Button Background
    .description = Button background color

settings-modules-timer-left-click = Left Click
    .description = Shell command on left click

settings-modules-timer-right-click = Right Click
    .description = Shell command on right click

settings-modules-timer-middle-click = Middle Click
    .description = Shell command on middle click

settings-modules-timer-scroll-up = Scroll Up
    .description = Shell command on scroll up

settings-modules-timer-scroll-down = Scroll Down
    .description = Shell command on scroll down

## Format mode enum

enum-timer-format-mode-largest-to-smallest = Largest to smallest
    .description = Normalize to the largest unit, e.g. 1h 5m

enum-timer-format-mode-as-set = As set
    .description = Keep the configured granularity, e.g. 66:00 then 65:59
