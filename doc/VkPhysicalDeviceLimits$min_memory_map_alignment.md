[`min_memory_map_alignment`] is the
minimum  **required**  alignment, in bytes, of host visible memory
allocations within the host address space.
When mapping a memory allocation with [`map_memory`], subtracting
`offset` bytes from the returned pointer will always produce an
integer multiple of this limit.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-hostaccess](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-hostaccess).
The value  **must**  be a power of two.