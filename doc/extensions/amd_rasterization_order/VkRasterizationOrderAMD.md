[VkRasterizationOrderAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRasterizationOrderAMD.html) - Specify rasterization order for a graphics pipeline

# C Specifications
Possible values of
[`PipelineRasterizationStateRasterizationOrderAMD::rasterization_order`],
specifying the primitive rasterization order, are:
```c
// Provided by VK_AMD_rasterization_order
typedef enum VkRasterizationOrderAMD {
    VK_RASTERIZATION_ORDER_STRICT_AMD = 0,
    VK_RASTERIZATION_ORDER_RELAXED_AMD = 1,
} VkRasterizationOrderAMD;
```

# Description
- [`STRICT`] specifies that operations for each primitive in a subpass  **must**  occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
- [`RELAXED`] specifies that operations for each primitive in a subpass  **may**  not occur in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).

# Related
- [`VK_AMD_rasterization_order`]
- [`PipelineRasterizationStateRasterizationOrderAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        