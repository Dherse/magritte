[VkSubpassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassBeginInfo.html) - Structure specifying subpass begin information

# C Specifications
The [`SubpassBeginInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSubpassBeginInfo {
    VkStructureType      sType;
    const void*          pNext;
    VkSubpassContents    contents;
} VkSubpassBeginInfo;
```
or the equivalent
```c
// Provided by VK_KHR_create_renderpass2
typedef VkSubpassBeginInfo VkSubpassBeginInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`contents`] is a [`SubpassContents`] value specifying how the commands in the next subpass will be provided.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`contents`] **must**  be a valid [`SubpassContents`] value

# Related
- [`VK_KHR_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`StructureType`]
- [`SubpassContents`]
- [`cmd_begin_render_pass2`]
- [`cmd_begin_render_pass2_khr`]
- [`cmd_next_subpass2`]
- [`cmd_next_subpass2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        