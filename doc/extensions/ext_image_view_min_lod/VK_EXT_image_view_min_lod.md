[VK_EXT_image_view_min_lod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_view_min_lod.html) - device extension

# Description
This extension allows applications to clamp the minimum LOD value during
[Image Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and
[Integer Texel Coordinate
Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations) with a given [`ImageView`] by
[`ImageViewMinLodCreateInfoEXT::min_lod`].This extension may be useful to restrict a [`ImageView`] to only mips
which have been uploaded, and the use of fractional `minLod` can be
useful for smoothly introducing new mip levels when using linear mipmap
filtering.

# Registered extension number
392

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Joshua Ashton [Joshua-Ashton](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_image_view_min_lod] @Joshua-Ashton%0A<<Here describe the issue or question you have about the VK_EXT_image_view_min_lod extension>>)

# New structures
- Extending [`ImageViewCreateInfo`]:  - [`ImageViewMinLodCreateInfoEXT`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceImageViewMinLodFeaturesEXT`]

# New constants
- [`EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME`]
- [`EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`

# Version history
- Revision 1, 2021-07-06 (Joshua Ashton)  - Initial version

# Other information
* 2021-11-09
* No known IP claims.
*   - Joshua Ashton, Valve  - Hans-Kristian Arntzen, Valve  - Samuel Iglesias Gonsalvez, Igalia  - Tobias Hector, AMD  - Jason Ekstrand, Intel  - Tom Olson, ARM

# Related
- [`ImageViewMinLodCreateInfoEXT`]
- [`PhysicalDeviceImageViewMinLodFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        