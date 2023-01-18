[`ENABLE_AUTOMATIC_CHECKPOINTS`]
enables automatic insertion of [diagnostic checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-diagnostic-checkpoints) for draw calls, dispatches,
trace rays,
and copies.
The CPU call stack at the time of the command will be associated as the
marker data for the automatically inserted checkpoints.