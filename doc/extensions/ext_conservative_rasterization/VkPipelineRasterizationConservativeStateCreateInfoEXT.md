[VkPipelineRasterizationConservativeStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html) - Structure specifying conservative raster state

# C Specifications
Polygon rasterization  **can**  be made conservative by setting
[`conservative_rasterization_mode`] to
`VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT` or
`VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT` in
[`PipelineRasterizationConservativeStateCreateInfoEXT`].
The [`PipelineRasterizationConservativeStateCreateInfoEXT`] state is set
by adding this structure to the [`p_next`] chain of a
[`PipelineRasterizationStateCreateInfo`] structure when creating the
graphics pipeline.
Enabling these modes also affects line and point rasterization if the
implementation sets
[`PhysicalDeviceConservativeRasterizationPropertiesEXT::conservative_point_and_line_rasterization`]
to [`TRUE`].[`PipelineRasterizationConservativeStateCreateInfoEXT`] is defined as:
```c
// Provided by VK_EXT_conservative_rasterization
typedef struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
    VkStructureType                                           sType;
    const void*                                               pNext;
    VkPipelineRasterizationConservativeStateCreateFlagsEXT    flags;
    VkConservativeRasterizationModeEXT                        conservativeRasterizationMode;
    float                                                     extraPrimitiveOverestimationSize;
} VkPipelineRasterizationConservativeStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`conservative_rasterization_mode`] is the conservative rasterization mode to use.
- [`extra_primitive_overestimation_size`] is the extra size in pixels to increase the generating primitive during conservative rasterization at each of its edges in `X` and `Y` equally in screen space beyond the base overestimation specified in [`PhysicalDeviceConservativeRasterizationPropertiesEXT::primitive_overestimation_size`].

# Description
## Valid Usage
-  [`extra_primitive_overestimation_size`] **must**  be in the range of `0.0` to [`PhysicalDeviceConservativeRasterizationPropertiesEXT::max_extra_primitive_overestimation_size`] inclusive

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT`
-  [`flags`] **must**  be `0`
-  [`conservative_rasterization_mode`] **must**  be a valid [`ConservativeRasterizationModeEXT`] value

# Related
- [`VK_EXT_conservative_rasterization`]
- [`ConservativeRasterizationModeEXT`]
- [`PipelineRasterizationConservativeStateCreateFlagsEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        