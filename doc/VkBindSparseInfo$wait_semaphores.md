[`wait_semaphores`] is a pointer to an array of semaphores upon which
to wait on before the sparse binding operations for this batch begin
execution.
If semaphores to wait on are provided, they define a
[semaphore wait operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting).