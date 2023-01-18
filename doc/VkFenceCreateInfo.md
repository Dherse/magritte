[VkFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceCreateInfo.html) - Structure specifying parameters of a newly created fence

# C Specifications
The [`FenceCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkFenceCreateInfo {
    VkStructureType       sType;
    const void*           pNext;
    VkFenceCreateFlags    flags;
} VkFenceCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`FenceCreateFlagBits`] specifying the initial state and behavior of the fence.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FENCE_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`ExportFenceCreateInfo`] or [`ExportFenceWin32HandleInfoKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`FenceCreateFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [`FenceCreateFlags`]
- [`StructureType`]
- [`create_fence`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        