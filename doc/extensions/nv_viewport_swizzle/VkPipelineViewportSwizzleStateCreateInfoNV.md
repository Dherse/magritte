[VkPipelineViewportSwizzleStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) - Structure specifying swizzle applied to primitive clip coordinates

# C Specifications
Each primitive sent to a given viewport has a swizzle and  **optional**  negation
applied to its clip coordinates.
The swizzle that is applied depends on the viewport index, and is controlled
by the [`PipelineViewportSwizzleStateCreateInfoNV`] pipeline state:
```c
// Provided by VK_NV_viewport_swizzle
typedef struct VkPipelineViewportSwizzleStateCreateInfoNV {
    VkStructureType                                sType;
    const void*                                    pNext;
    VkPipelineViewportSwizzleStateCreateFlagsNV    flags;
    uint32_t                                       viewportCount;
    const VkViewportSwizzleNV*                     pViewportSwizzles;
} VkPipelineViewportSwizzleStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`viewport_count`] is the number of viewport swizzles used by the pipeline.
- [`viewport_swizzles`] is a pointer to an array of [`ViewportSwizzleNV`] structures, defining the viewport swizzles.

# Description
## Valid Usage
-  [`viewport_count`] **must**  be greater than or equal to the [`viewport_count`] set in [`PipelineViewportStateCreateInfo`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`
-  [`flags`] **must**  be `0`
-  [`viewport_swizzles`] **must**  be a valid pointer to an array of [`viewport_count`] valid [`ViewportSwizzleNV`] structures
-  [`viewport_count`] **must**  be greater than `0`

# Related
- [`nv_viewport_swizzle`]
- [`PipelineViewportSwizzleStateCreateFlagsNV`]
- [`StructureType`]
- [`ViewportSwizzleNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        