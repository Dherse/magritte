[VK_EXT_ycbcr_2plane_444_formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_ycbcr_2plane_444_formats.html) - device extension

# Description
This extension adds some Y′C<sub>B</sub>C<sub>R</sub> formats that are in common use for video
encode and decode, but were not part of the
`[`khr_sampler_ycbcr_conversion`]` extension.

# Registered extension number
331

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_sampler_ycbcr_conversion`]`

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Tony Zlatinski [tzlatinski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_ycbcr_2plane_444_formats] @tzlatinski%0A<<Here describe the issue or question you have about the VK_EXT_ycbcr_2plane_444_formats extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`]

# New constants
- `VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME`
- `VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION`
- Extending [`Format`]:  - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT`  - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT`  - `VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT`  - `VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT`

# Version history
- Revision 1, 2020-03-08 (Piers Daniell)  - Initial draft

# Other information
* 2020-07-28
*   - Promoted to Vulkan 1.3 Core 
* No known IP claims.
*   - Piers Daniell, NVIDIA  - Ping Liu, Intel

# Related
- [`PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        