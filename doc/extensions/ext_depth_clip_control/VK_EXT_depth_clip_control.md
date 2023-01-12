[VK_EXT_depth_clip_control](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_control.html) - device extension

# Description
This extension allows the application to use the OpenGL depth range in NDC,
i.e. with depth in range [-1, 1], as opposed to Vulkan’s default of
[0, 1].
The purpose of this extension is to allow efficient layering of OpenGL over
Vulkan, by avoiding emulation in the pre-rasterization shader stages.
This emulation, which effectively duplicates gl_Position but with a
different depth value, costs ALU and consumes shader output components that
the implementation may not have to spare to meet OpenGL minimum
requirements.

# Registered extension number
356

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Shahbaz Youssefi [syoussefi](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_depth_clip_control] @syoussefi%0A<<Here describe the issue or question you have about the VK_EXT_depth_clip_control extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceDepthClipControlFeaturesEXT`] 
- Extending [`PipelineViewportStateCreateInfo`]:  - [`PipelineViewportDepthClipControlCreateInfoEXT`]

# New constants
- `VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME`
- `VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT`

# Known issues & F.A.Q.
1) Should this extension include an origin control option to match
GL_LOWER_LEFT found in ARB_clip_control? **RESOLVED** : No.
The fix for porting over the origin is a simple y-axis flip.
The depth clip control is a much harder problem to solve than what this
extension is aimed to solve.
Adding an equivalent to GL_LOWER_LEFT would require more testing.2) Should this pipeline state be dynamic? **RESOLVED** : Yes.
The purpose of this extension is to emulate the OpenGL depth range, which is
expected to be globally fixed (in case of OpenGL ES) or very infrequently
changed (with `glClipControl` in OpenGL).3) Should the control provided in this extension be an enum that could be
extended in the future? **RESOLVED** : No.
It is highly unlikely that the depth range is changed to anything other than
[0, 1] in the future.
Should that happen a new extension will be required to extend such an enum,
and that extension might as well add a new struct to chain to
[`PipelineViewportStateCreateInfo::p_next`] instead.

# Version history
- Revision 0, 2020-10-01 (Spencer Fricke)  - Internal revisions 
- Revision 1, 2020-11-26 (Shahbaz Youssefi)  - Language fixes

# Other information
* 2021-11-09
*   - Spencer Fricke, Samsung Electronics  - Shahbaz Youssefi, Google  - Ralph Potter, Samsung Electronics

# Related
- [`PhysicalDeviceDepthClipControlFeaturesEXT`]
- [`PipelineViewportDepthClipControlCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        