[VK_EXT_rgba10x6_formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_rgba10x6_formats.html) - device extension

# Description
This extension enables the
`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16` format to be used without
a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) enabled.

# Registered extension number
345

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_sampler_ycbcr_conversion`]`

# Contacts
- Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_rgba10x6_formats] @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the VK_EXT_rgba10x6_formats extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceRgba10x6FormatsFeaturesEXT`]

# New constants
- [`EXT_RGBA10X6_FORMATS_EXTENSION_NAME`]
- [`EXT_RGBA10X6_FORMATS_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT`

# Known issues & F.A.Q.
1) Should we reuse the existing format enumeration or introduce a new one? **RESOLVED** : We reuse an existing format enumeration,
`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`, that was previously
exclusively used for YCbCr and therefore had a set of limitations related to
that usage.
The alternative was to introduce a new format token with exactly the same
bit representation as the existing token, but without the limitations.2) Should we only introduce
`VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16` or also 1-3 component
variations? **RESOLVED** : Only the 4-component format is introduced because the 1- and 2-
component variations are already not exclusive to YCbCr, and the 3-component
variation is not a good match for hardware capabilities.

# Version history
- Revision 1, 2021-09-29 (Jan-Harald Fredriksen)  - Initial EXT version

# Other information
* 2021-09-29
* No known IP claims.
*   - Jan-Harald Fredriksen, Arm  - Graeme Leese, Broadcom  - Spencer Fricke, Samsung

# Related
- [`PhysicalDeviceRgba10x6FormatsFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        