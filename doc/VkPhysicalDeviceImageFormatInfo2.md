[VkPhysicalDeviceImageFormatInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html) - Structure specifying image creation parameters

# C Specifications
The [`PhysicalDeviceImageFormatInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDeviceImageFormatInfo2 {
    VkStructureType       sType;
    const void*           pNext;
    VkFormat              format;
    VkImageType           type;
    VkImageTiling         tiling;
    VkImageUsageFlags     usage;
    VkImageCreateFlags    flags;
} VkPhysicalDeviceImageFormatInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkPhysicalDeviceImageFormatInfo2 VkPhysicalDeviceImageFormatInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure. The [`p_next`] chain of [`PhysicalDeviceImageFormatInfo2`] is used to provide additional image parameters to [`get_physical_device_image_format_properties2`].
- [`format`] is a [`Format`] value indicating the image format, corresponding to [`ImageCreateInfo`]::[`format`].
- [`type_`] is a [`ImageType`] value indicating the image type, corresponding to [`ImageCreateInfo::image_type`].
- [`tiling`] is a [`ImageTiling`] value indicating the image tiling, corresponding to [`ImageCreateInfo`]::[`tiling`].
- [`usage`] is a bitmask of [`ImageUsageFlagBits`] indicating the intended usage of the image, corresponding to [`ImageCreateInfo`]::[`usage`].
- [`flags`] is a bitmask of [`ImageCreateFlagBits`] indicating additional parameters of the image, corresponding to [`ImageCreateInfo`]::[`flags`].

# Description
The members of [`PhysicalDeviceImageFormatInfo2`] correspond to the
arguments to [`get_physical_device_image_format_properties`], with
[`s_type`] and [`p_next`] added for extensibility.
## Valid Usage
-  [`tiling`] **must**  be `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` if and only if the [`p_next`] chain includes [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
-    If [`tiling`] is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` and [`flags`] contains `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`, then the [`p_next`] chain  **must**  include a [`ImageFormatListCreateInfo`] structure with non-zero `viewFormatCount`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`ImageFormatListCreateInfo`], [`ImageStencilUsageCreateInfo`], [`PhysicalDeviceExternalImageFormatInfo`], [`PhysicalDeviceImageDrmFormatModifierInfoEXT`], or [`PhysicalDeviceImageViewImageFormatInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`format`] **must**  be a valid [`Format`] value
-  [`type_`] **must**  be a valid [`ImageType`] value
-  [`tiling`] **must**  be a valid [`ImageTiling`] value
-  [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-  [`flags`] **must**  be a valid combination of [`ImageCreateFlagBits`] values

# Related
- [`crate::vulkan1_1`]
- [`Format`]
- [`ImageCreateFlags`]
- [`ImageTiling`]
- [`ImageType`]
- [`ImageUsageFlags`]
- [`StructureType`]
- [`get_physical_device_image_format_properties2`]
- [`get_physical_device_image_format_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        