[`robust_buffer_access2`] indicates
whether buffer accesses are tightly bounds-checked against the range of
the descriptor.
Uniform buffers  **must**  be bounds-checked to the range of the descriptor,
where the range is rounded up to a multiple of
[robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment).
Storage buffers  **must**  be bounds-checked to the range of the descriptor,
where the range is rounded up to a multiple of
[robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment).
Out of bounds buffer loads will return zero values, and formatted loads
will have (0,0,1) values inserted for missing G, B, or A
components based on the format.