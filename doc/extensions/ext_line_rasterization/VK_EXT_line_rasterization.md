[VK_EXT_line_rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_line_rasterization.html) - device extension

# Description
This extension adds some line rasterization features that are commonly used
in CAD applications and supported in other APIs like OpenGL.
Bresenham-style line rasterization is supported, smooth rectangular lines
(coverage to alpha) are supported, and stippled lines are supported for all
three line rasterization modes.

# Registered extension number
260

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_line_rasterization] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_EXT_line_rasterization extension>>)

# New commands
- [`cmd_set_line_stipple_ext`]

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceLineRasterizationFeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceLineRasterizationPropertiesEXT`] 
- Extending [`PipelineRasterizationStateCreateInfo`]:  - [`PipelineRasterizationLineStateCreateInfoEXT`]

# New enums
- [`LineRasterizationModeEXT`]

# New constants
- `VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME`
- `VK_EXT_LINE_RASTERIZATION_SPEC_VERSION`
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_LINE_STIPPLE_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT`

# Known issues & F.A.Q.
```c
(1) Do we need to support Bresenham-style and smooth lines with more than
one rasterization sample? i.e. the equivalent of glDisable(GL_MULTISAMPLE)
in OpenGL when the framebuffer has more than one sample?
```

```c
RESOLVED: Yes.
For simplicity, Bresenham line rasterization carries forward a few
restrictions from OpenGL, such as not supporting per-sample shading, alpha
to coverage, or alpha to one.
```

# Version history
- Revision 1, 2019-05-09 (Jeff Bolz)  - Initial draft

# Other information
* 2019-05-09
* No known IP claims.
*   - Jeff Bolz, NVIDIA  - Allen Jensen, NVIDIA  - Jason Ekstrand, Intel

# Related
- [`LineRasterizationModeEXT`]
- [`PhysicalDeviceLineRasterizationFeaturesEXT`]
- [`PhysicalDeviceLineRasterizationPropertiesEXT`]
- [`PipelineRasterizationLineStateCreateInfoEXT`]
- [`cmd_set_line_stipple_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        