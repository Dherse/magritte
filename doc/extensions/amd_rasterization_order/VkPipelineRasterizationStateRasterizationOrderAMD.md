[VkPipelineRasterizationStateRasterizationOrderAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html) - Structure defining rasterization order for a graphics pipeline

# C Specifications
The rasterization order to use for a graphics pipeline is specified by
adding a [`PipelineRasterizationStateRasterizationOrderAMD`] structure
to the [`p_next`] chain of a [`PipelineRasterizationStateCreateInfo`]
structure.The [`PipelineRasterizationStateRasterizationOrderAMD`] structure is
defined as:
```c
// Provided by VK_AMD_rasterization_order
typedef struct VkPipelineRasterizationStateRasterizationOrderAMD {
    VkStructureType            sType;
    const void*                pNext;
    VkRasterizationOrderAMD    rasterizationOrder;
} VkPipelineRasterizationStateRasterizationOrderAMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`rasterization_order`] is a [`RasterizationOrderAMD`] value specifying the primitive rasterization order to use.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD`
-  [`rasterization_order`] **must**  be a valid [`RasterizationOrderAMD`] value
If the `[`amd_rasterization_order`]` device extension is not enabled
or the application does not request a particular rasterization order through
specifying a [`PipelineRasterizationStateRasterizationOrderAMD`]
structure then the rasterization order used by the graphics pipeline
defaults to `VK_RASTERIZATION_ORDER_STRICT_AMD`.

# Related
- [`amd_rasterization_order`]
- [`RasterizationOrderAMD`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        