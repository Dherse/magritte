[VkImageTiling](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageTiling.html) - Specifies the tiling arrangement of data in an image

# C Specifications
Possible values of [`ImageCreateInfo::tiling`], specifying the
tiling arrangement of texel blocks in an image, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkImageTiling {
    VK_IMAGE_TILING_OPTIMAL = 0,
    VK_IMAGE_TILING_LINEAR = 1,
  // Provided by VK_EXT_image_drm_format_modifier
    VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT = 1000158000,
} VkImageTiling;
```

# Description
- [`OPTIMAL`] specifies optimal tiling (texels are laid out in an implementation-dependent arrangement, for more efficient memory access).
- [`LINEAR`] specifies linear tiling (texels are laid out in memory in row-major order, possibly with some padding on each row).
- [`DRM_FORMAT_MODIFIER_EXT`] indicates that the image’s tiling is defined by a [Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier). The modifier is specified at image creation with [`ImageDrmFormatModifierListCreateInfoEXT`] or [`ImageDrmFormatModifierExplicitCreateInfoEXT`], and  **can**  be queried with [`get_image_drm_format_modifier_properties_ext`].

# Related
- [`crate::vulkan1_0`]
- [`ImageCreateInfo`]
- [`PhysicalDeviceImageFormatInfo2`]
- [`PhysicalDeviceSparseImageFormatInfo2`]
- [`get_physical_device_external_image_format_properties_nv`]
- [`get_physical_device_image_format_properties`]
- [`get_physical_device_sparse_image_format_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        