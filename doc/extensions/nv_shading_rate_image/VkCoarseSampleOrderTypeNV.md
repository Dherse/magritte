[VkCoarseSampleOrderTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderTypeNV.html) - Shading rate image sample ordering types

# C Specifications
The type [`CoarseSampleOrderTypeNV`] specifies the technique used to
order coverage samples in fragments larger than one pixel, and is defined
as:
```c
// Provided by VK_NV_shading_rate_image
typedef enum VkCoarseSampleOrderTypeNV {
    VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV = 0,
    VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV = 1,
    VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV = 2,
    VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV = 3,
} VkCoarseSampleOrderTypeNV;
```

# Description
- [`DEFAULT`] specifies that coverage samples will be ordered in an implementation-dependent manner.
- [`CUSTOM`] specifies that coverage samples will be ordered according to the array of custom orderings provided in either the `pCustomSampleOrders` member of [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] or the `pCustomSampleOrders` member of [`cmd_set_coarse_sample_order_nv`].
- [`PIXEL_MAJOR`] specifies that coverage samples will be ordered sequentially, sorted first by pixel coordinate (in row-major order) and then by [sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask).
- [`SAMPLE_MAJOR`] specifies that coverage samples will be ordered sequentially, sorted first by [sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) and then by pixel coordinate (in row-major order).

# Related
- [`VK_NV_shading_rate_image`]
- [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
- [`cmd_set_coarse_sample_order_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        