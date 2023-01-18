[VK_EXT_discard_rectangles](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_discard_rectangles.html) - device extension

# Description
This extension provides additional orthogonally aligned “discard
rectangles” specified in framebuffer-space coordinates that restrict
rasterization of all points, lines and triangles.From zero to an implementation-dependent limit (specified by
`maxDiscardRectangles`) number of discard rectangles can be operational
at once.
When one or more discard rectangles are active, rasterized fragments can
either survive if the fragment is within any of the operational discard
rectangles (`VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT` mode) or be
rejected if the fragment is within any of the operational discard rectangles
(`VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT` mode).These discard rectangles operate orthogonally to the existing scissor test
functionality.
The discard rectangles can be different for each physical device in a device
group by specifying the device mask and setting discard rectangle dynamic
state.

# Registered extension number
100

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_discard_rectangles] @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_EXT_discard_rectangles extension>>)

# New commands
- [`cmd_set_discard_rectangle_ext`]

# New structures
- Extending [`GraphicsPipelineCreateInfo`]:  - [`PipelineDiscardRectangleStateCreateInfoEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceDiscardRectanglePropertiesEXT`]

# New enums
- [`DiscardRectangleModeEXT`]

# New bitmasks
- [`PipelineDiscardRectangleStateCreateFlagsEXT`]

# New constants
- [`EXT_DISCARD_RECTANGLES_EXTENSION_NAME`]
- [`EXT_DISCARD_RECTANGLES_SPEC_VERSION`]
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT`

# Version history
- Revision 1, 2016-12-22 (Piers Daniell)  - Internal revisions

# Other information
* 2016-12-22
*   - Interacts with `[`VK_KHR_device_group`]`  - Interacts with Vulkan 1.1 
*   - Daniel Koch, NVIDIA  - Jeff Bolz, NVIDIA

# Related
- [`DiscardRectangleModeEXT`]
- [`PhysicalDeviceDiscardRectanglePropertiesEXT`]
- [`PipelineDiscardRectangleStateCreateFlagsEXT`]
- [`PipelineDiscardRectangleStateCreateInfoEXT`]
- [`cmd_set_discard_rectangle_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        