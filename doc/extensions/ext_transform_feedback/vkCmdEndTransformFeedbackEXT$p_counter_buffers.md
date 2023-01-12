[`p_counter_buffers`] is `NULL` or a pointer to an array of
[`Buffer`] handles to counter buffers.
The counter buffers are used to record the current byte positions of
each transform feedback buffer where the next vertex output data would
be captured.
This  **can**  be used by a subsequent [`cmd_begin_transform_feedback_ext`]
call to resume transform feedback capture from this position.
It can also be used by [`cmd_draw_indirect_byte_count_ext`] to determine
the vertex count of the draw call.