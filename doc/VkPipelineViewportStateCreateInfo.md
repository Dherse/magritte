[VkPipelineViewportStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline viewport state

# C Specifications
The [`PipelineViewportStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineViewportStateCreateInfo {
    VkStructureType                       sType;
    const void*                           pNext;
    VkPipelineViewportStateCreateFlags    flags;
    uint32_t                              viewportCount;
    const VkViewport*                     pViewports;
    uint32_t                              scissorCount;
    const VkRect2D*                       pScissors;
} VkPipelineViewportStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`viewport_count`] is the number of viewports used by the pipeline.
- [`viewports`] is a pointer to an array of [`Viewport`] structures, defining the viewport transforms. If the viewport state is dynamic, this member is ignored.
- [`scissor_count`] is the number of [scissors](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-scissor) and  **must**  match the number of viewports.
- [`scissors`] is a pointer to an array of [`Rect2D`] structures defining the rectangular bounds of the scissor for the corresponding viewport. If the scissor state is dynamic, this member is ignored.

# Description
## Valid Usage
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`viewport_count`] **must**  not be greater than `1`
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`scissor_count`] **must**  not be greater than `1`
-  [`viewport_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_viewports`]
-  [`scissor_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_viewports`]
-    The `x` and `y` members of `offset` member of any element of [`scissors`] **must**  be greater than or equal to `0`
-    Evaluation of (`offset.x` +  `extent.width`) **must**  not cause a signed integer addition overflow for any element of [`scissors`]
-    Evaluation of (`offset.y` +  `extent.height`) **must**  not cause a signed integer addition overflow for any element of [`scissors`]
-    If the graphics pipeline is being created without `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` set then [`scissor_count`] and [`viewport_count`] **must**  be identical
-    If the graphics pipeline is being created with `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` set then [`viewport_count`] **must**  be `0`, otherwise it  **must**  be greater than `0`
-    If the graphics pipeline is being created with `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` set then [`scissor_count`] **must**  be `0`, otherwise it  **must**  be greater than `0`
-    If the `viewportWScalingEnable` member of a [`PipelineViewportWScalingStateCreateInfoNV`] structure included in the [`p_next`] chain is [`TRUE`], the [`viewport_count`] member of the [`PipelineViewportWScalingStateCreateInfoNV`] structure  **must**  be greater than or equal to [`PipelineViewportStateCreateInfo`]::[`viewport_count`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`], [`PipelineViewportDepthClipControlCreateInfoEXT`], [`PipelineViewportExclusiveScissorStateCreateInfoNV`], [`PipelineViewportShadingRateImageStateCreateInfoNV`], [`PipelineViewportSwizzleStateCreateInfoNV`], or [`PipelineViewportWScalingStateCreateInfoNV`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be `0`

# Related
- [`crate::vulkan1_0`]
- [`GraphicsPipelineCreateInfo`]
- [`PipelineViewportStateCreateFlags`]
- [`Rect2D`]
- [`StructureType`]
- [`Viewport`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        