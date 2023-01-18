[`RESET_COMMAND_BUFFER`] allows any command
buffer allocated from a pool to be individually reset to the
[initial state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle); either by calling
[`reset_command_buffer`], or via the implicit reset when calling
[`begin_command_buffer`].
If this flag is not set on a pool, then [`reset_command_buffer`] **must** 
not be called for any command buffer allocated from that pool.