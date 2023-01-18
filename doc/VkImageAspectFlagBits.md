[VkImageAspectFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageAspectFlagBits.html) - Bitmask specifying which aspects of an image are included in a view

# C Specifications
Bits which  **can**  be set in an aspect mask to specify aspects of an image for
purposes such as identifying a subresource, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkImageAspectFlagBits {
    VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001,
    VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002,
    VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004,
    VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_ASPECT_PLANE_0_BIT = 0x00000010,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_ASPECT_PLANE_1_BIT = 0x00000020,
  // Provided by VK_VERSION_1_1
    VK_IMAGE_ASPECT_PLANE_2_BIT = 0x00000040,
  // Provided by VK_VERSION_1_3
    VK_IMAGE_ASPECT_NONE = 0,
  // Provided by VK_EXT_image_drm_format_modifier
    VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT = 0x00000080,
  // Provided by VK_EXT_image_drm_format_modifier
    VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT = 0x00000100,
  // Provided by VK_EXT_image_drm_format_modifier
    VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT = 0x00000200,
  // Provided by VK_EXT_image_drm_format_modifier
    VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT = 0x00000400,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_IMAGE_ASPECT_PLANE_0_BIT_KHR = VK_IMAGE_ASPECT_PLANE_0_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_IMAGE_ASPECT_PLANE_1_BIT_KHR = VK_IMAGE_ASPECT_PLANE_1_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_IMAGE_ASPECT_PLANE_2_BIT_KHR = VK_IMAGE_ASPECT_PLANE_2_BIT,
  // Provided by VK_KHR_maintenance4
    VK_IMAGE_ASPECT_NONE_KHR = VK_IMAGE_ASPECT_NONE,
} VkImageAspectFlagBits;
```

# Description
- [`NONE`] specifies no image aspect, or the image aspect is not applicable.
- [`COLOR`] specifies the color aspect.
- [`DEPTH`] specifies the depth aspect.
- [`STENCIL`] specifies the stencil aspect.
- [`METADATA`] specifies the metadata aspect, used for [sparse resource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory) operations.
- [`PLANE0`] specifies plane 0 of a *multi-planar* image format.
- [`PLANE1`] specifies plane 1 of a *multi-planar* image format.
- [`PLANE2`] specifies plane 2 of a *multi-planar* image format.
- [`MEMORY_PLANE0_EXT`] specifies *memory plane* 0.
- [`MEMORY_PLANE1_EXT`] specifies *memory plane* 1.
- [`MEMORY_PLANE2_EXT`] specifies *memory plane* 2.
- [`MEMORY_PLANE3_EXT`] specifies *memory plane* 3.

# Related
- [`crate::vulkan1_0`]
- [`BindImagePlaneMemoryInfo`]
- [`DeviceImageMemoryRequirements`]
- [`ImageAspectFlags`]
- [`ImagePlaneMemoryRequirementsInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        