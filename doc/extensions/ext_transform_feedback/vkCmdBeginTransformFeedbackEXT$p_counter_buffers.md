[`p_counter_buffers`] is `NULL` or a pointer to an array of
[`Buffer`] handles to counter buffers.
Each buffer contains a 4 byte integer value representing the byte offset
from the start of the corresponding transform feedback buffer from where
to start capturing vertex data.
If the byte offset stored to the counter buffer location was done using
[`cmd_end_transform_feedback_ext`] it can be used to resume transform
feedback from the previous location.
If [`p_counter_buffers`] is `NULL`, then transform feedback will start
capturing vertex data to byte offset zero in all bound transform
feedback buffers.
For each element of [`p_counter_buffers`] that is [`crate::Handle::null`],
transform feedback will start capturing vertex data to byte zero in the
corresponding bound transform feedback buffer.