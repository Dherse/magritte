[vkInvalidateMappedMemoryRanges](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInvalidateMappedMemoryRanges.html) - Invalidate ranges of mapped memory objects

# C Specifications
To invalidate ranges of non-coherent memory from the host caches, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkInvalidateMappedMemoryRanges(
    VkDevice                                    device,
    uint32_t                                    memoryRangeCount,
    const VkMappedMemoryRange*                  pMemoryRanges);
```

# Parameters
- [`device`] is the logical device that owns the memory ranges.
- [`memory_range_count`] is the length of the [`p_memory_ranges`] array.
- [`p_memory_ranges`] is a pointer to an array of [`MappedMemoryRange`] structures describing the memory ranges to invalidate.

# Description
[`invalidate_mapped_memory_ranges`] guarantees that device writes to the
memory ranges described by [`p_memory_ranges`], which have been made
available to the host memory domain using the `VK_ACCESS_HOST_WRITE_BIT`
and `VK_ACCESS_HOST_READ_BIT`[access
types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types), are made visible to the host.
If a range of non-coherent memory is written by the host and then
invalidated without first being flushed, its contents are undefined.Within each range described by [`p_memory_ranges`], each set of
`nonCoherentAtomSize` bytes in that range is invalidated if any byte in
that set has been written by the device since it was first host mapped, or
the last time it was invalidated.
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
        