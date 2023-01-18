[VkMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier.html) - Structure specifying a global memory barrier

# C Specifications
The [`MemoryBarrier`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkMemoryBarrier {
    VkStructureType    sType;
    const void*        pNext;
    VkAccessFlags      srcAccessMask;
    VkAccessFlags      dstAccessMask;
} VkMemoryBarrier;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).

# Description
The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to access types in the [source access
mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks) specified by [`src_access_mask`].The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to access types in the [destination
access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks) specified by [`dst_access_mask`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_BARRIER`
-  [`p_next`] **must**  be `NULL`
-  [`src_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values
-  [`dst_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [`AccessFlags`]
- [`StructureType`]
- [`cmd_pipeline_barrier`]
- [`cmd_wait_events`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        