[VkPipelineDepthStencilStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline depth stencil state

# C Specifications
The [`PipelineDepthStencilStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineDepthStencilStateCreateInfo {
    VkStructureType                           sType;
    const void*                               pNext;
    VkPipelineDepthStencilStateCreateFlags    flags;
    VkBool32                                  depthTestEnable;
    VkBool32                                  depthWriteEnable;
    VkCompareOp                               depthCompareOp;
    VkBool32                                  depthBoundsTestEnable;
    VkBool32                                  stencilTestEnable;
    VkStencilOpState                          front;
    VkStencilOpState                          back;
    float                                     minDepthBounds;
    float                                     maxDepthBounds;
} VkPipelineDepthStencilStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`PipelineDepthStencilStateCreateFlagBits`] specifying additional depth/stencil state information.
- [`depth_test_enable`] controls whether [depth testing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-depth) is enabled.
- [`depth_write_enable`] controls whether [depth writes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-depth-write) are enabled when [`depth_test_enable`] is `VK_TRUE`. Depth writes are always disabled when [`depth_test_enable`] is `VK_FALSE`.
- [`depth_compare_op`] is the comparison operator used in the [depth test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-depth).
- [`depth_bounds_test_enable`] controls whether [depth bounds testing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-dbt) is enabled.
- [`stencil_test_enable`] controls whether [stencil testing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil) is enabled.
- [`front`] and [`back`] control the parameters of the [stencil test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil).
- [`min_depth_bounds`] is the minimum depth bound used in the [depth bounds test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-dbt).
- [`max_depth_bounds`] is the maximum depth bound used in the [depth bounds test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-dbt).

# Description
## Valid Usage
-    If the [depth bounds testing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-depthBounds) feature is not enabled, [`depth_bounds_test_enable`] **must**  be `VK_FALSE`
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::separate_stencil_mask_ref`] is `VK_FALSE`, and the value of [`PipelineDepthStencilStateCreateInfo`]::[`stencil_test_enable`] is `VK_TRUE`, and the value of [`PipelineRasterizationStateCreateInfo::cull_mode`] is `VK_CULL_MODE_NONE`, the value of `reference` in each of the [`StencilOpState`] structs in [`front`] and [`back`] **must**  be the same
-    If the [`rasterizationOrderDepthAttachmentAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rasterizationOrderDepthAttachmentAccess) feature is not enabled, [`flags`] **must**  not include `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`
-    If the [`rasterizationOrderStencilAttachmentAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rasterizationOrderStencilAttachmentAccess) feature is not enabled, [`flags`] **must**  not include `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`PipelineDepthStencilStateCreateFlagBits`] values
-  [`depth_compare_op`] **must**  be a valid [`CompareOp`] value
-  [`front`] **must**  be a valid [`StencilOpState`] structure
-  [`back`] **must**  be a valid [`StencilOpState`] structure

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`CompareOp`]
- [`GraphicsPipelineCreateInfo`]
- [VkPipelineDepthStencilStateCreateFlags]()
- [`StencilOpState`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        