[VK_KHR_format_feature_flags2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_format_feature_flags2.html) - device extension

# Description
This extension adds a new [`FormatFeatureFlagBits2KHR`] 64bits format
feature flag type to extend the existing [`FormatFeatureFlagBits`] which
is limited to 31 flags.
At the time of this writing 29 bits of [`FormatFeatureFlagBits`] are
already used.Because [`FormatProperties2`] is already defined to extend the Vulkan
1.0 [`get_physical_device_format_properties`] entry point, this extension
defines a new [`FormatProperties3KHR`] to extend the
[`FormatProperties`].On top of replicating all the bits from [`FormatFeatureFlagBits`],
[`FormatFeatureFlagBits2KHR`] adds the following bits :
- `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR` and `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR` indicate that an implementation supports respectively reading and writing a given [`Format`] through storage operations without specifying the format in the shader.
- `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR` indicates that an implementation supports depth comparison performed by `OpImage*Dref*` instructions on a given [`Format`]. Previously the result of executing a `OpImage*Dref*` instruction on an image view, where the `format` was not one of the depth/stencil formats with a depth component, was undefined. This bit clarifies on which formats such instructions can be used.

# Registered extension number
361

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Lionel Landwerlin [llandwerlin](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_format_feature_flags2] @llandwerlin%0A<<Here describe the issue or question you have about the VK_KHR_format_feature_flags2 extension>>)

# New structures
- Extending [`FormatProperties2`]:  - [`FormatProperties3KHR`]

# New enums
- [`FormatFeatureFlagBits2KHR`]

# New bitmasks
- [`FormatFeatureFlags2KHR`]

# New constants
- [`KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME`]
- [`KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3_KHR`

# Version history
- Revision 1, 2020-07-21 (Lionel Landwerlin)  - Initial draft

# Other information
* 2021-07-01
*   - Promoted to Vulkan 1.3 Core 
* No known IP claims.
*   - Lionel Landwerlin, Intel  - Jason Ekstrand, Intel  - Tobias Hector, AMD  - Spencer Fricke, Samsung Electronics  - Graeme Leese, Broadcom  - Jan-Harald Fredriksen, ARM

# Related
- [`FormatFeatureFlagBits2KHR`]
- [`FormatFeatureFlags2KHR`]
- [`FormatProperties3KHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        