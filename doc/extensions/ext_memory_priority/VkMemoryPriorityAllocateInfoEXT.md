[VkMemoryPriorityAllocateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html) - Specify a memory allocation priority

# C Specifications
If the [`p_next`] chain includes a [`MemoryPriorityAllocateInfoEXT`]
structure, then that structure includes a priority for the memory.The [`MemoryPriorityAllocateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_memory_priority
typedef struct VkMemoryPriorityAllocateInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    float              priority;
} VkMemoryPriorityAllocateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`priority`] is a floating-point value between `0` and `1`, indicating the priority of the allocation relative to other memory allocations. Larger values are higher priority. The granularity of the priorities is implementation-dependent.

# Description
Memory allocations with higher priority  **may**  be more likely to stay in
device-local memory when the system is under memory pressure.If this structure is not included, it is as if the [`priority`] value were
`0.5`.
## Valid Usage
-  [`priority`] **must**  be between `0` and `1`, inclusive

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`

# Related
- [`VK_EXT_memory_priority`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        