[VK_EXT_fragment_shader_interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_shader_interlock.html) - device extension

# Description
This extension adds support for the `FragmentShaderPixelInterlockEXT`,
`FragmentShaderSampleInterlockEXT`, and
`FragmentShaderShadingRateInterlockEXT` capabilities from the
`SPV_EXT_fragment_shader_interlock` extension to Vulkan.Enabling these capabilities provides a critical section for fragment shaders
to avoid overlapping pixels being processed at the same time, and certain
guarantees about the ordering of fragment shader invocations of fragments of
overlapping pixels.This extension can be useful for algorithms that need to access per-pixel
data structures via shader loads and stores.
Algorithms using this extension can access per-pixel data structures in
critical sections without other invocations accessing the same per-pixel
data.
Additionally, the ordering guarantees are useful for cases where the API
ordering of fragments is meaningful.
For example, applications may be able to execute programmable blending
operations in the fragment shader, where the destination buffer is read via
image loads and the final value is written via image stores.

# Registered extension number
252

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_fragment_shader_interlock] @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_EXT_fragment_shader_interlock extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`]

# New constants
- `VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME`
- `VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT`

# Version history
- Revision 1, 2019-05-24 (Piers Daniell)  - Internal revisions

# Other information
* 2019-05-02
*   - This extension requires [`SPV_EXT_fragment_shader_interlock`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_fragment_shader_interlock.html)  - This extension provides API support for [`GL_ARB_fragment_shader_interlock`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_fragment_shader_interlock.txt) 
*   - Daniel Koch, NVIDIA  - Graeme Leese, Broadcom  - Jan-Harald Fredriksen, Arm  - Jason Ekstrand, Intel  - Jeff Bolz, NVIDIA  - Ruihao Zhang, Qualcomm  - Slawomir Grajewski, Intel  - Spencer Fricke, Samsung

# Related
- [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        