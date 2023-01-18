[`FIFO_RELAXED`] specifies that the presentation
engine generally waits for the next vertical blanking period to update
the current image.
If a vertical blanking period has already passed since the last update
of the current image then the presentation engine does not wait for
another vertical blanking period for the update, meaning this mode  **may** 
result in visible tearing in this case.
This mode is useful for reducing visual stutter with an application that
will mostly present a new image before the next vertical blanking
period, but may occasionally be late, and present a new image just after
the next vertical blanking period.
An internal queue is used to hold pending presentation requests.
New requests are appended to the end of the queue, and one request is
removed from the beginning of the queue and processed during or after
each vertical blanking period in which the queue is non-empty.