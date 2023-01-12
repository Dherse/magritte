[`robust_uniform_buffer_access_size_alignment`] is the number of bytes that
the range of a uniform buffer descriptor is rounded up to when used for
bounds-checking when
[`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled.
This value  **must**  be a power of two in the range [1, 256].