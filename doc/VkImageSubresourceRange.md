[VkImageSubresourceRange](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceRange.html) - Structure specifying an image subresource range

# C Specifications
The [`ImageSubresourceRange`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageSubresourceRange {
    VkImageAspectFlags    aspectMask;
    uint32_t              baseMipLevel;
    uint32_t              levelCount;
    uint32_t              baseArrayLayer;
    uint32_t              layerCount;
} VkImageSubresourceRange;
```

# Members
- [`aspect_mask`] is a bitmask of [`ImageAspectFlagBits`] specifying which aspect(s) of the image are included in the view.
- [`base_mip_level`] is the first mipmap level accessible to the view.
- [`level_count`] is the number of mipmap levels (starting from [`base_mip_level`]) accessible to the view.
- [`base_array_layer`] is the first array layer accessible to the view.
- [`layer_count`] is the number of array layers (starting from [`base_array_layer`]) accessible to the view.

# Description
The number of mipmap levels and array layers  **must**  be a subset of the image
subresources in the image.
If an application wants to use all mip levels or layers in an image after
the [`base_mip_level`] or [`base_array_layer`], it  **can**  set [`level_count`]
and [`layer_count`] to the special values [`REMAINING_MIP_LEVELS`] and
[`REMAINING_ARRAY_LAYERS`] without knowing the exact number of mip
levels or layers.For cube and cube array image views, the layers of the image view starting
at [`base_array_layer`] correspond to faces in the order +X, -X, +Y, -Y, +Z,
-Z.
For cube arrays, each set of six sequential layers is a single cube, so the
number of cube maps in a cube map array view is *[`layer_count`] / 6*, and
image array layer ([`base_array_layer`] +  i) is face index
(i mod 6) of cube *i / 6*.
If the number of layers in the view, whether set explicitly in
[`layer_count`] or implied by [`REMAINING_ARRAY_LAYERS`], is not a
multiple of 6, the last cube map in the array  **must**  not be accessed.[`aspect_mask`] **must**  be only `VK_IMAGE_ASPECT_COLOR_BIT`,
`VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT` if
`format` is a color, depth-only or stencil-only format,
respectively, except if `format` is a
[multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion).
If using a depth/stencil format with both depth and stencil components,
[`aspect_mask`] **must**  include at least one of
`VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT`, and
 **can**  include both.When the [`ImageSubresourceRange`] structure is used to select a subset
of the slices of a 3D image’s mip level in order to create a 2D or 2D array
image view of a 3D image created with
`VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT`, [`base_array_layer`] and
[`layer_count`] specify the first slice index and the number of slices to
include in the created image view.
Such an image view  **can**  be used as a framebuffer attachment that refers only
to the specified range of slices of the selected mip level.
However, any layout transitions performed on such an attachment view during
a render pass instance still apply to the entire subresource referenced
which includes all the slices of the selected mip level.When using an image view of a depth/stencil image to populate a descriptor
set (e.g. for sampling in the shader, or for use as an input attachment),
the [`aspect_mask`] **must**  only include one bit, which selects whether the
image view is used for depth reads (i.e. using a floating-point sampler or
input attachment in the shader) or stencil reads (i.e. using an unsigned
integer sampler or input attachment in the shader).
When an image view of a depth/stencil image is used as a depth/stencil
framebuffer attachment, the [`aspect_mask`] is ignored and both depth and
stencil image subresources are used.When creating a [`ImageView`], if [sampler
Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) is enabled in the sampler, the [`aspect_mask`] of a
`subresourceRange` used by the [`ImageView`] **must**  be
`VK_IMAGE_ASPECT_COLOR_BIT`.When creating a [`ImageView`], if sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is not
enabled in the sampler and the image `format` is
[multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), the image  **must** 
have been created with `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`, and the
[`aspect_mask`] of the [`ImageView`]’s `subresourceRange` **must**  be
`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or
`VK_IMAGE_ASPECT_PLANE_2_BIT`.
## Valid Usage
-    If [`level_count`] is not [`REMAINING_MIP_LEVELS`], it  **must**  be greater than `0`
-    If [`layer_count`] is not [`REMAINING_ARRAY_LAYERS`], it  **must**  be greater than `0`
-    If [`aspect_mask`] includes `VK_IMAGE_ASPECT_COLOR_BIT`, then it  **must**  not include any of `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
-  [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index *i*

## Valid Usage (Implicit)
-  [`aspect_mask`] **must**  be a valid combination of [`ImageAspectFlagBits`] values
-  [`aspect_mask`] **must**  not be `0`

# Related
- [`crate::vulkan1_0`]
- [`ImageAspectFlags`]
- [`ImageMemoryBarrier`]
- [`ImageMemoryBarrier2`]
- [`ImageViewCreateInfo`]
- [`cmd_clear_color_image`]
- [`cmd_clear_depth_stencil_image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        