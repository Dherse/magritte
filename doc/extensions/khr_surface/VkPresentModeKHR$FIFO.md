[`FIFO`] specifies that the presentation engine
waits for the next vertical blanking period to update the current image.
Tearing  **cannot**  be observed.
An internal queue is used to hold pending presentation requests.
New requests are appended to the end of the queue, and one request is
removed from the beginning of the queue and processed during each
vertical blanking period in which the queue is non-empty.
This is the only value of `presentMode` that is  **required**  to be
supported.