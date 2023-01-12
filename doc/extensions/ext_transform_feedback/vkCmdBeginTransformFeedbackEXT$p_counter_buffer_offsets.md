[`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of
[`DeviceSize`] values specifying offsets within each of the
[`p_counter_buffers`] where the counter values were previously written.
The location in each counter buffer at these offsets  **must**  be large
enough to contain 4 bytes of data.
This data is the number of bytes captured by the previous transform
feedback to this buffer.
If [`p_counter_buffer_offsets`] is `NULL`, then it is assumed the offsets
are zero.