[`MAILBOX`] specifies that the presentation engine
waits for the next vertical blanking period to update the current image.
Tearing  **cannot**  be observed.
An internal single-entry queue is used to hold pending presentation
requests.
If the queue is full when a new presentation request is received, the
new request replaces the existing entry, and any images associated with
the prior entry become available for re-use by the application.
One request is removed from the queue and processed during each vertical
blanking period in which the queue is non-empty.