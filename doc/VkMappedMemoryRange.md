[VkMappedMemoryRange](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMappedMemoryRange.html) - Structure specifying a mapped memory range

# C Specifications
The [`MappedMemoryRange`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkMappedMemoryRange {
    VkStructureType    sType;
    const void*        pNext;
    VkDeviceMemory     memory;
    VkDeviceSize       offset;
    VkDeviceSize       size;
} VkMappedMemoryRange;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`memory`] is the memory object to which this range belongs.
- [`offset`] is the zero-based byte offset from the beginning of the memory object.
- [`size`] is either the size of range, or [`WHOLE_SIZE`] to affect the range from [`offset`] to the end of the current mapping of the allocation.

# Description
## Valid Usage
-  [`memory`] **must**  be currently host mapped
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`offset`] and [`size`] **must**  specify a range contained within the currently mapped range of [`memory`]
-    If [`size`] is equal to [`WHOLE_SIZE`], [`offset`] **must**  be within the currently mapped range of [`memory`]
-  [`offset`] **must**  be a multiple of [`PhysicalDeviceLimits::non_coherent_atom_size`]
-    If [`size`] is equal to [`WHOLE_SIZE`], the end of the current mapping of [`memory`] **must**  either be a multiple of [`PhysicalDeviceLimits::non_coherent_atom_size`] bytes from the beginning of the memory object, or be equal to the end of the memory object
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  either be a multiple of [`PhysicalDeviceLimits::non_coherent_atom_size`], or [`offset`] plus [`size`] **must**  equal the size of [`memory`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE`
-  [`p_next`] **must**  be `NULL`
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle

# Related
- [`crate::vulkan1_0`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`StructureType`]
- [`flush_mapped_memory_ranges`]
- [`invalidate_mapped_memory_ranges`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        