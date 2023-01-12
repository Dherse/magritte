[`signal_semaphores`] is a pointer to an array of semaphores which
will be signaled when the sparse binding operations for this batch have
completed execution.
If semaphores to be signaled are provided, they define a
[semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling).