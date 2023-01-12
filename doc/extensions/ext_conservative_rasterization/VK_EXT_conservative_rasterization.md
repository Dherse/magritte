[VK_EXT_conservative_rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_conservative_rasterization.html) - device extension

# Description
This extension adds a new rasterization mode called conservative
rasterization.
There are two modes of conservative rasterization; overestimation and
underestimation.When overestimation is enabled, if any part of the primitive, including its
edges, covers any part of the rectangular pixel area, including its sides,
then a fragment is generated with all coverage samples turned on.
This extension allows for some variation in implementations by accounting
for differences in overestimation, where the generating primitive size is
increased at each of its edges by some sub-pixel amount to further increase
conservative pixel coverage.
Implementations can allow the application to specify an extra overestimation
beyond the base overestimation the implementation already does.
It also allows implementations to either cull degenerate primitives or
rasterize them.When underestimation is enabled, fragments are only generated if the
rectangular pixel area is fully covered by the generating primitive.
If supported by the implementation, when a pixel rectangle is fully covered
the fragment shader input variable builtin called FullyCoveredEXT is set to
true.
The shader variable works in either overestimation or underestimation mode.Implementations can process degenerate triangles and lines by either
discarding them or generating conservative fragments for them.
Degenerate triangles are those that end up with zero area after the
rasterizer quantizes them to the fixed-point pixel grid.
Degenerate lines are those with zero length after quantization.

# Registered extension number
102

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_conservative_rasterization] @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_EXT_conservative_rasterization extension>>)

# New structures
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceConservativeRasterizationPropertiesEXT`] 
- Extending [`PipelineRasterizationStateCreateInfo`]:  - [`PipelineRasterizationConservativeStateCreateInfoEXT`]

# New enums
- [`ConservativeRasterizationModeEXT`]

# New bitmasks
- [`PipelineRasterizationConservativeStateCreateFlagsEXT`]

# New constants
- `VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME`
- `VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT`

# Version history
- Revision 1.1, 2020-09-06 (Piers Daniell)  - Add missing SPIR-V and GLSL dependencies. 
- Revision 1, 2017-08-28 (Piers Daniell)  - Internal revisions

# Other information
* 2020-06-09
*   - This extension requires [`SPV_EXT_fragment_fully_covered`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_fragment_fully_covered.html) if the [`PhysicalDeviceConservativeRasterizationPropertiesEXT::fully_covered_fragment_shader_input_variable`] feature is used.  - This extension requires [`SPV_KHR_post_depth_coverage`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_post_depth_coverage.html)if the [`PhysicalDeviceConservativeRasterizationPropertiesEXT::conservative_rasterization_post_depth_coverage`] feature is used.  - This extension provides API support for [`GL_NV_conservative_raster_underestimation`](https://www.khronos.org/registry/OpenGL/extensions/NV/NV_conservative_raster_underestimation.txt) if the [`PhysicalDeviceConservativeRasterizationPropertiesEXT::fully_covered_fragment_shader_input_variable`] feature is used. 
*   - Daniel Koch, NVIDIA  - Daniel Rakos, AMD  - Jeff Bolz, NVIDIA  - Slawomir Grajewski, Intel  - Stu Smith, Imagination Technologies

# Related
- [`ConservativeRasterizationModeEXT`]
- [`PhysicalDeviceConservativeRasterizationPropertiesEXT`]
- [`PipelineRasterizationConservativeStateCreateFlagsEXT`]
- [`PipelineRasterizationConservativeStateCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        