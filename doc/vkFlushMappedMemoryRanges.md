[vkFlushMappedMemoryRanges](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFlushMappedMemoryRanges.html) - Flush mapped memory ranges

# C Specifications
To flush ranges of non-coherent memory from the host caches, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkFlushMappedMemoryRanges(
    VkDevice                                    device,
    uint32_t                                    memoryRangeCount,
    const VkMappedMemoryRange*                  pMemoryRanges);
```

# Parameters
- [`device`] is the logical device that owns the memory ranges.
- [`memory_range_count`] is the length of the [`p_memory_ranges`] array.
- [`p_memory_ranges`] is a pointer to an array of [`MappedMemoryRange`] structures describing the memory ranges to flush.

# Description
[`flush_mapped_memory_ranges`] guarantees that host writes to the memory
ranges described by [`p_memory_ranges`] are made available to the host
memory domain, such that they  **can**  be made available to the device memory
domain via [memory
domain operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-available-and-visible) using the `VK_ACCESS_HOST_WRITE_BIT`[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types).Within each range described by [`p_memory_ranges`], each set of
`nonCoherentAtomSize` bytes in that range is flushed if any byte in that
set has been written by the host since it was first host mapped, or the last
time it was flushed.
If [`p_memory_ranges`] includes sets of `nonCoherentAtomSize` bytes
where no bytes have been written by the host, those bytes  **must**  not be
flushed.Unmapping non-coherent memory does not implicitly flush the host mapped
memory, and host writes that have not been flushed  **may**  not ever be visible
to the device.
However, implementations  **must**  ensure that writes that have not been flushed
do not become visible to any other memory.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_memory_ranges`] **must**  be a valid pointer to an array of [`memory_range_count`] valid [`MappedMemoryRange`] structures
-  [`memory_range_count`] **must**  be greater than `0`

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`MappedMemoryRange`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        