[VkPipelineViewportCoarseSampleOrderStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html) - Structure specifying parameters controlling sample order in coarse fragments

# C Specifications
If the [`p_next`] chain of [`PipelineViewportStateCreateInfo`] includes
a [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] structure, then
that structure includes parameters controlling the order of coverage samples
in fragments larger than one pixel.The [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] structure is
defined as:
```c
// Provided by VK_NV_shading_rate_image
typedef struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
    VkStructureType                       sType;
    const void*                           pNext;
    VkCoarseSampleOrderTypeNV             sampleOrderType;
    uint32_t                              customSampleOrderCount;
    const VkCoarseSampleOrderCustomNV*    pCustomSampleOrders;
} VkPipelineViewportCoarseSampleOrderStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`sample_order_type`] specifies the mechanism used to order coverage samples in fragments larger than one pixel.
- [`custom_sample_order_count`] specifies the number of custom sample orderings to use when ordering coverage samples.
- [`custom_sample_orders`] is a pointer to an array of [`custom_sample_order_count`][`CoarseSampleOrderCustomNV`] structures, each structure specifying the coverage sample order for a single combination of fragment area and coverage sample count.

# Description
If this structure is not present, [`sample_order_type`] is considered to be
`VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV`.If [`sample_order_type`] is `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`, the
coverage sample order used for any combination of fragment area and coverage
sample count not enumerated in [`custom_sample_orders`] will be identical
to that used for `VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV`.If the pipeline was created with
`VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV`, the contents of this
structure (if present) are ignored, and the coverage sample order is instead
specified by [`cmd_set_coarse_sample_order_nv`].
## Valid Usage
-    If [`sample_order_type`] is not `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`, `customSamplerOrderCount` **must**  be `0`
-    The array [`custom_sample_orders`] **must**  not contain two structures with matching values for both the `shadingRate` and `sampleCount` members

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`
-  [`sample_order_type`] **must**  be a valid [`CoarseSampleOrderTypeNV`] value
-    If [`custom_sample_order_count`] is not `0`, [`custom_sample_orders`] **must**  be a valid pointer to an array of [`custom_sample_order_count`] valid [`CoarseSampleOrderCustomNV`] structures

# Related
- [`VK_NV_shading_rate_image`]
- [`CoarseSampleOrderCustomNV`]
- [`CoarseSampleOrderTypeNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        