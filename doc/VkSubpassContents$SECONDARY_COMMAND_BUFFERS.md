[`SECONDARY_COMMAND_BUFFERS`] specifies that the
contents are recorded in secondary command buffers that will be called
from the primary command buffer, and [`cmd_execute_commands`] is the
only valid command on the command buffer until [`cmd_next_subpass`] or
[`cmd_end_render_pass`].