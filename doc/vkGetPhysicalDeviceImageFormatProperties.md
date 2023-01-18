[vkGetPhysicalDeviceImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html) - Lists physical device's image format capabilities

# C Specifications
To query additional capabilities specific to image types, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkGetPhysicalDeviceImageFormatProperties(
    VkPhysicalDevice                            physicalDevice,
    VkFormat                                    format,
    VkImageType                                 type,
    VkImageTiling                               tiling,
    VkImageUsageFlags                           usage,
    VkImageCreateFlags                          flags,
    VkImageFormatProperties*                    pImageFormatProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the image capabilities.
- [`format`] is a [`Format`] value specifying the image format, corresponding to [`ImageCreateInfo`]::[`format`].
- [`type_`] is a [`ImageType`] value specifying the image type, corresponding to [`ImageCreateInfo::image_type`].
- [`tiling`] is a [`ImageTiling`] value specifying the image tiling, corresponding to [`ImageCreateInfo`]::[`tiling`].
- [`usage`] is a bitmask of [`ImageUsageFlagBits`] specifying the intended usage of the image, corresponding to [`ImageCreateInfo`]::[`usage`].
- [`flags`] is a bitmask of [`ImageCreateFlagBits`] specifying additional parameters of the image, corresponding to [`ImageCreateInfo`]::[`flags`].
- [`p_image_format_properties`] is a pointer to a [`ImageFormatProperties`] structure in which capabilities are returned.

# Description
The [`format`], [`type_`], [`tiling`], [`usage`], and [`flags`]
parameters correspond to parameters that would be consumed by
[`create_image`] (as members of [`ImageCreateInfo`]).If [`format`] is not a supported image format, or if the combination of
[`format`], [`type_`], [`tiling`], [`usage`], and [`flags`] is not
supported for images, then [`get_physical_device_image_format_properties`]
returns `VK_ERROR_FORMAT_NOT_SUPPORTED`.The limitations on an image format that are reported by
[`get_physical_device_image_format_properties`] have the following property:
if `usage1` and `usage2` of type [`ImageUsageFlags`] are such that
the bits set in `usage1` are a subset of the bits set in `usage2`, and
`flags1` and `flags2` of type [`ImageCreateFlags`] are such that
the bits set in `flags1` are a subset of the bits set in `flags2`,
then the limitations for `usage1` and `flags1` **must**  be no more strict
than the limitations for `usage2` and `flags2`, for all values of
[`format`], [`type_`], and [`tiling`].
## Valid Usage
-  [`tiling`] **must**  not be `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`. (Use [`get_physical_device_image_format_properties2`] instead)

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`format`] **must**  be a valid [`Format`] value
-  [`type_`] **must**  be a valid [`ImageType`] value
-  [`tiling`] **must**  be a valid [`ImageTiling`] value
-  [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-  [`flags`] **must**  be a valid combination of [`ImageCreateFlagBits`] values
-  [`p_image_format_properties`] **must**  be a valid pointer to a [`ImageFormatProperties`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`

# Related
- [`crate::vulkan1_0`]
- [`Format`]
- [`ImageCreateFlags`]
- [`ImageFormatProperties`]
- [`ImageTiling`]
- [`ImageType`]
- [`ImageUsageFlags`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        