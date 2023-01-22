[`fence`] is an  **optional**  handle to a fence to be signaled once all
submitted command buffers have completed execution.
If [`fence`] is not [`crate::Handle::null`], it defines a
[fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling).