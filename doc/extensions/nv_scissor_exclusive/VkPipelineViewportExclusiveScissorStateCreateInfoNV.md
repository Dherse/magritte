[VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html) - Structure specifying parameters controlling exclusive scissor testing

# C Specifications
The [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure is
defined as:
```c
// Provided by VK_NV_scissor_exclusive
typedef struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           exclusiveScissorCount;
    const VkRect2D*    pExclusiveScissors;
} VkPipelineViewportExclusiveScissorStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`exclusive_scissor_count`] is the number of exclusive scissor rectangles.
- [`exclusive_scissors`] is a pointer to an array of [`Rect2D`] structures defining exclusive scissor rectangles.

# Description
If the `VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV` dynamic state is enabled
for a pipeline, the [`exclusive_scissors`] member is ignored.When this structure is included in the [`p_next`] chain of
[`GraphicsPipelineCreateInfo`], it defines parameters of the exclusive
scissor test.
If this structure is not included in the [`p_next`] chain, it is equivalent
to specifying this structure with a [`exclusive_scissor_count`] of `0`.
## Valid Usage
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`exclusive_scissor_count`] **must**  be `0` or `1`
-  [`exclusive_scissor_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_viewports`]
-  [`exclusive_scissor_count`] **must**  be `0` or greater than or equal to the `viewportCount` member of [`PipelineViewportStateCreateInfo`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`

# Related
- [`nv_scissor_exclusive`]
- [`Rect2D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        