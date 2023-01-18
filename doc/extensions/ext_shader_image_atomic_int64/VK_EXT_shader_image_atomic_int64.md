[VK_EXT_shader_image_atomic_int64](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_image_atomic_int64.html) - device extension

# Description
This extension extends existing 64-bit integer atomic support to enable
these operations on images as well.When working with large 2- or 3-dimensional data sets (e.g. rasterization or
screen-space effects), image accesses are generally more efficient than
equivalent buffer accesses.
This extension allows applications relying on 64-bit integer atomics in this
manner to quickly improve performance with only relatively minor code
changes.64-bit integer atomic support is guaranteed for optimally tiled images with
the `VK_FORMAT_R64_UINT` and `VK_FORMAT_R64_SINT` formats.

# Registered extension number
235

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_image_atomic_int64] @tobski%0A<<Here describe the issue or question you have about the VK_EXT_shader_image_atomic_int64 extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceShaderImageAtomicInt64FeaturesEXT`]

# New constants
- [`EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME`]
- [`EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT`

# Version history
- Revision 1, 2020-07-14 (Tobias Hector)  - Initial draft

# Other information
* 2020-07-14
* No known IP claims.
*   - This extension requires [`SPV_EXT_shader_image_int64`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_image_int64.html)  - This extension provides API support for [`GLSL_EXT_shader_image_int64`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_shader_image_int64.txt) 
*   - Matthaeus Chajdas, AMD  - Graham Wihlidal, Epic Games  - Tobias Hector, AMD  - Jeff Bolz, Nvidia  - Jason Ekstrand, Intel

# Related
- [`PhysicalDeviceShaderImageAtomicInt64FeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        