[`SHARED_DEMAND_REFRESH`] specifies that the
presentation engine and application have concurrent access to a single
image, which is referred to as a *shared presentable image*.
The presentation engine is only required to update the current image
after a new presentation request is received.
Therefore the application  **must**  make a presentation request whenever an
update is required.
However, the presentation engine  **may**  update the current image at any
point, meaning this mode  **may**  result in visible tearing.