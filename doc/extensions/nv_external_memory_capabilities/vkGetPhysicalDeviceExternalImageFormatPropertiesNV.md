[vkGetPhysicalDeviceExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html) - Determine image capabilities compatible with external memory handle types

# C Specifications
To determine the image capabilities compatible with an external memory
handle type, call:
```c
// Provided by VK_NV_external_memory_capabilities
VkResult vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
    VkPhysicalDevice                            physicalDevice,
    VkFormat                                    format,
    VkImageType                                 type,
    VkImageTiling                               tiling,
    VkImageUsageFlags                           usage,
    VkImageCreateFlags                          flags,
    VkExternalMemoryHandleTypeFlagsNV           externalHandleType,
    VkExternalImageFormatPropertiesNV*          pExternalImageFormatProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the image capabilities
- [`format`] is the image format, corresponding to [`ImageCreateInfo`]::[`format`].
- [`type_`] is the image type, corresponding to [`ImageCreateInfo::image_type`].
- [`tiling`] is the image tiling, corresponding to [`ImageCreateInfo`]::[`tiling`].
- [`usage`] is the intended usage of the image, corresponding to [`ImageCreateInfo`]::[`usage`].
- [`flags`] is a bitmask describing additional parameters of the image, corresponding to [`ImageCreateInfo`]::[`flags`].
- [`external_handle_type`] is either one of the bits from [`ExternalMemoryHandleTypeFlagBitsNV`], or 0.
- [`p_external_image_format_properties`] is a pointer to a [`ExternalImageFormatPropertiesNV`] structure in which capabilities are returned.

# Description
If [`external_handle_type`] is 0,
`pExternalImageFormatProperties->imageFormatProperties` will return the
same values as a call to [`get_physical_device_image_format_properties`], and
the other members of [`p_external_image_format_properties`] will all be 0.
Otherwise, they are filled in as described for
[`ExternalImageFormatPropertiesNV`].
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`format`] **must**  be a valid [`Format`] value
-  [`type_`] **must**  be a valid [`ImageType`] value
-  [`tiling`] **must**  be a valid [`ImageTiling`] value
-  [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-  [`flags`] **must**  be a valid combination of [`ImageCreateFlagBits`] values
-  [`external_handle_type`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`] values
-  [`p_external_image_format_properties`] **must**  be a valid pointer to a [`ExternalImageFormatPropertiesNV`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_FORMAT_NOT_SUPPORTED`

# Related
- [`nv_external_memory_capabilities`]
- [`ExternalImageFormatPropertiesNV`]
- [VkExternalMemoryHandleTypeFlagsNV]()
- [`Format`]
- [VkImageCreateFlags]()
- [`ImageTiling`]
- [`ImageType`]
- [VkImageUsageFlags]()
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        