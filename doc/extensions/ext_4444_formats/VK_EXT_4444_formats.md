[VK_EXT_4444_formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_4444_formats.html) - device extension

# Description
This extension defines the `VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT` and
`VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT` formats which are defined in other
current graphics APIs.This extension may be useful for building translation layers for those APIs
or for porting applications that use these formats without having to resort
to swizzles.When VK_EXT_custom_border_color is used, these formats are not subject to
the same restrictions for border color without format as with
VK_FORMAT_B4G4R4A4_UNORM_PACK16.

# Registered extension number
341

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Joshua Ashton [Joshua-Ashton](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_4444_formats] @Joshua-Ashton%0A<<Here describe the issue or question you have about the VK_EXT_4444_formats extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevice4444FormatsFeaturesEXT`]

# New constants
- [`EXT_4444_FORMATS_EXTENSION_NAME`]
- [`EXT_4444_FORMATS_SPEC_VERSION`]
- Extending [`Format`]:  - `VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT`  - `VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT`

# Version history
- Revision 1, 2020-07-04 (Joshua Ashton)  - Initial draft

# Other information
* 2020-07-28
*   - Promoted to Vulkan 1.3 Core 
* No known IP claims.
*   - Joshua Ashton, Valve  - Jason Ekstrand, Intel

# Related
- [`PhysicalDevice4444FormatsFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        