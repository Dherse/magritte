[VkDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html) - Structure specifying properties of a format when combined with a DRM format modifier

# C Specifications
The [`DrmFormatModifierPropertiesEXT`] structure describes properties of
a [`Format`] when that format is combined with a
[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
These properties, like those of [`FormatProperties2`], are independent
of any particular image.The [`DrmFormatModifierPropertiesEXT`] structure is defined as:
```c
// Provided by VK_EXT_image_drm_format_modifier
typedef struct VkDrmFormatModifierPropertiesEXT {
    uint64_t                drmFormatModifier;
    uint32_t                drmFormatModifierPlaneCount;
    VkFormatFeatureFlags    drmFormatModifierTilingFeatures;
} VkDrmFormatModifierPropertiesEXT;
```

# Members
- [`drm_format_modifier`] is a *Linux DRM format modifier*.
- [`drm_format_modifier_plane_count`] is the number of *memory planes* in any image created with `format` and [`drm_format_modifier`]. An image’s *memory planecount* is distinct from its *format planecount*, as explained below.
- [`drm_format_modifier_tiling_features`] is a bitmask of [`FormatFeatureFlagBits`] that are supported by any image created with `format` and [`drm_format_modifier`].

# Description
The returned [`drm_format_modifier_tiling_features`] **must**  contain at least
one bit.The implementation  **must**  not return `DRM_FORMAT_MOD_INVALID` in
[`drm_format_modifier`].An image’s *memory planecount* (as returned by
[`drm_format_modifier_plane_count`]) is distinct from its *format planecount*
(in the sense of [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
Y′C<sub>B</sub>C<sub>R</sub> formats).
In [`ImageAspectFlags`], each
`VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` represents a *memory plane*
and each `VK_IMAGE_ASPECT_PLANE*_i_*BIT` a *format plane*.An image’s set of *format planes* is an ordered partition of the image’s
 **content**  into separable groups of format components.
The ordered partition is encoded in the name of each [`Format`].
For example, `VK_FORMAT_G8_B8R8_2PLANE_420_UNORM` contains two *format
planes*; the first plane contains the green component and the second plane
contains the blue component and red component.
If the format name does not contain `PLANE`, then the format contains a
single plane; for example, `VK_FORMAT_R8G8B8A8_UNORM`.
Some commands, such as [`cmd_copy_buffer_to_image`], do not operate on all
format components in the image, but instead operate only on the *format
planes* explicitly chosen by the application and operate on each *format
plane* independently.An image’s set of *memory planes* is an ordered partition of the image’s
 **memory**  rather than the image’s  **content** .
Each *memory plane* is a contiguous range of memory.
The union of an image’s *memory planes* is not necessarily contiguous.If an image is [linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the partition is
the same for *memory planes* and for *format planes*.
Therefore, if the returned [`drm_format_modifier`] is
`DRM_FORMAT_MOD_LINEAR`, then [`drm_format_modifier_plane_count`] **must** 
equal the *format planecount*, and [`drm_format_modifier_tiling_features`] **must**  be identical to the
[`FormatProperties2`]`::linearTilingFeatures` returned in the same
`pNext` chain.If an image is [non-linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the partition
of the image’s  **memory**  into *memory planes* is implementation-specific and
 **may**  be unrelated to the partition of the image’s  **content**  into *format
planes*.
For example, consider an image whose `format` is
`VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM`, `tiling` is
`VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, whose [`drm_format_modifier`]
is not `DRM_FORMAT_MOD_LINEAR`, and `flags` lacks
`VK_IMAGE_CREATE_DISJOINT_BIT`.
The image has 3 *format planes*, and commands such
[`cmd_copy_buffer_to_image`] act on each *format plane* independently as if
the data of each *format plane* were separable from the data of the other
planes.
In a straightforward implementation, the implementation  **may**  store the
image’s content in 3 adjacent *memory planes* where each *memory plane*
corresponds exactly to a *format plane*.
However, the implementation  **may**  also store the image’s content in a single
*memory plane* where all format components are combined using an
implementation-private block-compressed format; or the implementation  **may** 
store the image’s content in a collection of 7 adjacent *memory planes*
using an implementation-private sharding technique.
Because the image is non-linear and non-disjoint, the implementation has
much freedom when choosing the image’s placement in memory.The *memory planecount* applies to function parameters and structures only
when the API specifies an explicit requirement on
[`drm_format_modifier_plane_count`].
In all other cases, the *memory planecount* is ignored.

# Related
- [`VK_EXT_image_drm_format_modifier`]
- [`DrmFormatModifierPropertiesListEXT`]
- [`FormatFeatureFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        