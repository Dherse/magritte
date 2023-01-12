[`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of
[`DeviceSize`] values specifying offsets within each of the
[`p_counter_buffers`] where the counter values can be written.
The location in each counter buffer at these offsets  **must**  be large
enough to contain 4 bytes of data.
The data stored at this location is the byte offset from the start of
the transform feedback buffer binding where the next vertex data would
be written.
If [`p_counter_buffer_offsets`] is `NULL`, then it is assumed the offsets
are zero.