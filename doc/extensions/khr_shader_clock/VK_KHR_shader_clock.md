[VK_KHR_shader_clock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_clock.html) - device extension

# Description
This extension advertises the SPIR-V `ShaderClockKHR` capability for
Vulkan, which allows a shader to query a real-time or monotonically
incrementing counter at the subgroup level or across the device level.
The two valid SPIR-V scopes for `OpReadClockKHR` are `Subgroup` and
[`Device`].When using GLSL source-based shading languages, the `clockRealtime*EXT`()
timing functions map to the `OpReadClockKHR` instruction with a scope of
[`Device`], and the `clock*ARB`() timing functions map to the
`OpReadClockKHR` instruction with a scope of `Subgroup`.

# Registered extension number
182

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Aaron Hagan [ahagan](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_clock] @ahagan%0A<<Here describe the issue or question you have about the VK_KHR_shader_clock extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceShaderClockFeaturesKHR`]

# New constants
- `VK_KHR_SHADER_CLOCK_EXTENSION_NAME`
- `VK_KHR_SHADER_CLOCK_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`

# Version history
- Revision 1, 2019-4-25 (Aaron Hagan)  - Initial revision

# Other information
* 2019-4-25
* No known IP claims.
*   - This extension requires [`SPV_KHR_shader_clock`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_shader_clock.html).  - This extension provides API support for [`ARB_shader_clock`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_clock.txt) and [`EXT_shader_realtime_clock`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shader_realtime_clock.txt) 
*   - Aaron Hagan, AMD  - Daniel Koch, NVIDIA

# Related
- [`PhysicalDeviceShaderClockFeaturesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        