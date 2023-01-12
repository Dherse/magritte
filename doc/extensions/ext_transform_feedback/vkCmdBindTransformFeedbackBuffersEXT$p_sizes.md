[`p_sizes`] is `NULL` or a pointer to an array of [`DeviceSize`]
buffer sizes, specifying the maximum number of bytes to capture to the
corresponding transform feedback buffer.
If [`p_sizes`] is `NULL`, or the value of the [`p_sizes`] array
element is `VK_WHOLE_SIZE`, then the maximum number of bytes
captured will be the size of the corresponding buffer minus the buffer
offset.