[VkSubpassEndInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfo.html) - Structure specifying subpass end information

# C Specifications
The [`SubpassEndInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSubpassEndInfo {
    VkStructureType    sType;
    const void*        pNext;
} VkSubpassEndInfo;
```
or the equivalent
```c
// Provided by VK_KHR_create_renderpass2
typedef VkSubpassEndInfo VkSubpassEndInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`VK_KHR_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`StructureType`]
- [`cmd_end_render_pass2`]
- [`cmd_end_render_pass2_khr`]
- [`cmd_next_subpass2`]
- [`cmd_next_subpass2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        