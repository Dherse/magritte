[`persistent`]: If this is [`TRUE`], the display engine will
enable buffered mode on displays that support it.
This allows the display engine to stop sending content to the display
until a new image is presented.
The display will instead maintain a copy of the last presented image.
This allows less power to be used, but  **may**  increase presentation
latency.
If [`DisplayPresentInfoKHR`] is not specified, persistent mode will
not be used.