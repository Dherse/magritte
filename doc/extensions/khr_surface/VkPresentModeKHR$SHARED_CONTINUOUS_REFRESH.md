[`SHARED_CONTINUOUS_REFRESH`] specifies that the
presentation engine and application have concurrent access to a single
image, which is referred to as a *shared presentable image*.
The presentation engine periodically updates the current image on its
regular refresh cycle.
The application is only required to make one initial presentation
request, after which the presentation engine  **must**  update the current
image without any need for further presentation requests.
The application  **can**  indicate the image contents have been updated by
making a presentation request, but this does not guarantee the timing of
when it will be updated.
This mode  **may**  result in visible tearing if rendering to the image is
not timed correctly.