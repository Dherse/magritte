[`desired_present_time`] specifies that the image given  **should**  not be
displayed to the user any earlier than this time.
[`desired_present_time`] is a time in nanoseconds, relative to a
monotonically-increasing clock (e.g. `CLOCK_MONOTONIC` (see
clock_gettime(2)) on Android and Linux).
A value of zero specifies that the presentation engine  **may**  display the
image at any time.
This is useful when the application desires to provide [`present_id`],