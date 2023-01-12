[VkPipelineTessellationStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline tessellation state

# C Specifications
The [`PipelineTessellationStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineTessellationStateCreateInfo {
    VkStructureType                           sType;
    const void*                               pNext;
    VkPipelineTessellationStateCreateFlags    flags;
    uint32_t                                  patchControlPoints;
} VkPipelineTessellationStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`patch_control_points`] is the number of control points per patch.

# Description
## Valid Usage
-  [`patch_control_points`] **must**  be greater than zero and less than or equal to [`PhysicalDeviceLimits::max_tessellation_patch_size`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`PipelineTessellationDomainOriginStateCreateInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be `0`

# Related
- [`crate::vulkan1_0`]
- [`GraphicsPipelineCreateInfo`]
- [`GraphicsShaderGroupCreateInfoNV`]
- [`PipelineTessellationStateCreateFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        