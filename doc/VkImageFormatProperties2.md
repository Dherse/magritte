[VkImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties2.html) - Structure specifying an image format properties

# C Specifications
The [`ImageFormatProperties2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkImageFormatProperties2 {
    VkStructureType            sType;
    void*                      pNext;
    VkImageFormatProperties    imageFormatProperties;
} VkImageFormatProperties2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkImageFormatProperties2 VkImageFormatProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure. The [`p_next`] chain of [`ImageFormatProperties2`] is used to allow the specification of additional capabilities to be returned from [`get_physical_device_image_format_properties2`].
- [`image_format_properties`] is a [`ImageFormatProperties`] structure in which capabilities are returned.

# Description
If the combination of parameters to
[`get_physical_device_image_format_properties2`] is not supported by the
implementation for use in [`create_image`], then all members of
[`image_format_properties`] will be filled with zero.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`AndroidHardwareBufferUsageANDROID`], [`ExternalImageFormatProperties`], [`FilterCubicImageViewImageFormatPropertiesEXT`], [`SamplerYcbcrConversionImageFormatProperties`], or [`TextureLodGatherFormatPropertiesAMD`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`crate::vulkan1_1`]
- [`ImageFormatProperties`]
- [`StructureType`]
- [`get_physical_device_image_format_properties2`]
- [`get_physical_device_image_format_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        