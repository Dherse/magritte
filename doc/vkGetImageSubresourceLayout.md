[vkGetImageSubresourceLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout.html) - Retrieve information about an image subresource

# C Specifications
To query the memory layout of an image subresource, call:
```c
// Provided by VK_VERSION_1_0
void vkGetImageSubresourceLayout(
    VkDevice                                    device,
    VkImage                                     image,
    const VkImageSubresource*                   pSubresource,
    VkSubresourceLayout*                        pLayout);
```

# Parameters
- [`device`] is the logical device that owns the image.
- [`image`] is the image whose layout is being queried.
- [`p_subresource`] is a pointer to a [`ImageSubresource`] structure selecting a specific image for the image subresource.
- [`p_layout`] is a pointer to a [`SubresourceLayout`] structure in which the layout is returned.

# Description
If the image is [linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the
returned layout is valid for [host access](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-hostaccess).If the image’s
tiling is `VK_IMAGE_TILING_LINEAR` and its
format is a [multi-planar
format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), then [`get_image_subresource_layout`] describes one
*format plane*
of the image.
If the image’s tiling is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then
[`get_image_subresource_layout`] describes one *memory plane* of the image.
If the image’s tiling is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` and
the image is [non-linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the returned
layout has an implementation-dependent meaning; the vendor of the image’s
[DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier) **may**  provide
documentation that explains how to interpret the returned layout.[`get_image_subresource_layout`] is invariant for the lifetime of a single
image.
However, the subresource layout of images in Android hardware buffer
external memory is not known until the image has been bound to memory, so
applications  **must**  not call [`get_image_subresource_layout`] for such an
image before it has been bound.
## Valid Usage
-  [`image`] **must**  have been created with `tiling` equal to `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`
-    The `aspectMask` member of [`p_subresource`] **must**  only have a single bit set
-    The `mipLevel` member of [`p_subresource`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    The `arrayLayer` member of [`p_subresource`] **must**  be less than the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    If `format` is a color format, the `aspectMask` member of [`p_subresource`] **must**  be `VK_IMAGE_ASPECT_COLOR_BIT`
-    If `format` has a depth component, the `aspectMask` member of [`p_subresource`] **must**  contain `VK_IMAGE_ASPECT_DEPTH_BIT`
-    If `format` has a stencil component, the `aspectMask` member of [`p_subresource`] **must**  contain `VK_IMAGE_ASPECT_STENCIL_BIT`
-    If `format` does not contain a stencil or depth component, the `aspectMask` member of [`p_subresource`] **must**  not contain `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT`
-    If the `tiling` of the [`image`] is `VK_IMAGE_TILING_LINEAR` and its `format` is a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) with two planes, the `aspectMask` member of [`p_subresource`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`
-    If the `tiling` of the [`image`] is `VK_IMAGE_TILING_LINEAR` and its `format` is a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) with three planes, the `aspectMask` member of [`p_subresource`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If [`image`] was created with the `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` external memory handle type, then [`image`] **must**  be bound to memory
-    If the `tiling` of the [`image`] is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then the `aspectMask` member of [`p_subresource`] **must**  be `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` and the index *i* **must**  be less than the [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`image`] **must**  be a valid [`Image`] handle
-  [`p_subresource`] **must**  be a valid pointer to a valid [`ImageSubresource`] structure
-  [`p_layout`] **must**  be a valid pointer to a [`SubresourceLayout`] structure
-  [`image`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`Image`]
- [`ImageSubresource`]
- [`SubresourceLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        