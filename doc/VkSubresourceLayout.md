[VkSubresourceLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubresourceLayout.html) - Structure specifying subresource layout

# C Specifications
Information about the layout of the image subresource is returned in a
[`SubresourceLayout`] structure:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSubresourceLayout {
    VkDeviceSize    offset;
    VkDeviceSize    size;
    VkDeviceSize    rowPitch;
    VkDeviceSize    arrayPitch;
    VkDeviceSize    depthPitch;
} VkSubresourceLayout;
```

# Members
- [`offset`] is the byte offset from the start of the image or the plane where the image subresource begins.
- [`size`] is the size in bytes of the image subresource. [`size`] includes any extra memory that is required based on [`row_pitch`].
- [`row_pitch`] describes the number of bytes between each row of texels in an image.
- [`array_pitch`] describes the number of bytes between each array layer of an image.
- [`depth_pitch`] describes the number of bytes between each slice of 3D image.

# Description
If the image is [linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then [`row_pitch`],
[`array_pitch`] and [`depth_pitch`] describe the layout of the image
subresource in linear memory.
For uncompressed formats, [`row_pitch`] is the number of bytes between
texels with the same x coordinate in adjacent rows (y coordinates differ by
one).
[`array_pitch`] is the number of bytes between texels with the same x and y
coordinate in adjacent array layers of the image (array layer values differ
by one).
[`depth_pitch`] is the number of bytes between texels with the same x and y
coordinate in adjacent slices of a 3D image (z coordinates differ by one).
Expressed as an addressing formula, the starting byte of a texel in the
image subresource has address:
```c
// (x,y,z,layer) are in texel coordinates
address(x,y,z,layer) = layer*arrayPitch + z*depthPitch + y*rowPitch + x*elementSize + offset
```
For compressed formats, the [`row_pitch`] is the number of bytes between
compressed texel blocks in adjacent rows.
[`array_pitch`] is the number of bytes between compressed texel blocks in
adjacent array layers.
[`depth_pitch`] is the number of bytes between compressed texel blocks in
adjacent slices of a 3D image.
```c
// (x,y,z,layer) are in compressed texel block coordinates
address(x,y,z,layer) = layer*arrayPitch + z*depthPitch + y*rowPitch + x*compressedTexelBlockByteSize + offset;
```
The value of [`array_pitch`] is undefined for images that were not created
as arrays.
[`depth_pitch`] is defined only for 3D images.If the image has a
*single-plane*
color format
and its tiling is `VK_IMAGE_TILING_LINEAR`
, then the `aspectMask` member of [`ImageSubresource`] **must**  be
`VK_IMAGE_ASPECT_COLOR_BIT`.If the image has a depth/stencil format
and its tiling is `VK_IMAGE_TILING_LINEAR`
, then `aspectMask` **must**  be either `VK_IMAGE_ASPECT_DEPTH_BIT` or
`VK_IMAGE_ASPECT_STENCIL_BIT`.
On implementations that store depth and stencil aspects separately, querying
each of these image subresource layouts will return a different [`offset`]
and [`size`] representing the region of memory used for that aspect.
On implementations that store depth and stencil aspects interleaved, the
same [`offset`] and [`size`] are returned and represent the interleaved
memory allocation.If the image has a [multi-planar
format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
and its tiling is `VK_IMAGE_TILING_LINEAR`
, then the `aspectMask` member of [`ImageSubresource`] **must**  be
`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or
(for 3-plane formats only) `VK_IMAGE_ASPECT_PLANE_2_BIT`.
Querying each of these image subresource layouts will return a different
[`offset`] and [`size`] representing the region of memory used for that
plane.
If the image is *disjoint*, then the [`offset`] is relative to the base
address of the plane.
If the image is *non-disjoint*, then the [`offset`] is relative to the
base address of the image.If the image’s tiling is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then
the `aspectMask` member of [`ImageSubresource`] **must**  be one of
`VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT`, where the maximum allowed
plane index *i* is defined by the
[`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`]
associated with the image’s [`ImageCreateInfo::format`] and
[modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
The memory range used by the subresource is described by [`offset`] and
[`size`].
If the image is *disjoint*, then the [`offset`] is relative to the base
address of the *memory plane*.
If the image is *non-disjoint*, then the [`offset`] is relative to the
base address of the image.
If the image is [non-linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then
[`row_pitch`], [`array_pitch`], and [`depth_pitch`] have an
implementation-dependent meaning.

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [`ImageDrmFormatModifierExplicitCreateInfoEXT`]
- [`get_image_subresource_layout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        