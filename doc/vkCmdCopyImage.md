[vkCmdCopyImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImage.html) - Copy data between images

# C Specifications
To copy data between image objects, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdCopyImage(
    VkCommandBuffer                             commandBuffer,
    VkImage                                     srcImage,
    VkImageLayout                               srcImageLayout,
    VkImage                                     dstImage,
    VkImageLayout                               dstImageLayout,
    uint32_t                                    regionCount,
    const VkImageCopy*                          pRegions);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`src_image`] is the source image.
- [`src_image_layout`] is the current layout of the source image subresource.
- [`dst_image`] is the destination image.
- [`dst_image_layout`] is the current layout of the destination image subresource.
- [`region_count`] is the number of regions to copy.
- [`p_regions`] is a pointer to an array of [`ImageCopy`] structures specifying the regions to copy.

# Description
Each region in [`p_regions`] is copied from the source image to the same
region of the destination image.
[`src_image`] and [`dst_image`] **can**  be the same image or alias the same
memory.The formats of [`src_image`] and [`dst_image`] **must**  be compatible.
Formats are compatible if they share the same class, as shown in the
[Compatible Formats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility) table.
Depth/stencil formats  **must**  match exactly.If either [`src_image`] or [`dst_image`] has a
[*multi-planar* format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
regions of each plane to be copied  **must**  be specified separately using the
`srcSubresource` and `dstSubresource` members of the
[`ImageCopy`] structure.
In this case, the `aspectMask` of the `srcSubresource` or
`dstSubresource` that refers to the multi-planar image  **must**  be
`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or
`VK_IMAGE_ASPECT_PLANE_2_BIT`.
For the purposes of [`cmd_copy_image`], each plane of a multi-planar image
is treated as having the format listed in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes) for
the plane identified by the `aspectMask` of the corresponding
subresource.
This applies both to [`Format`] and to coordinates used in the copy,
which correspond to texels in the *plane* rather than how these texels map
to coordinates in the image as a whole.[`cmd_copy_image`] allows copying between *size-compatible* compressed and
uncompressed internal formats.
Formats are size-compatible if the texel block size of the uncompressed
format is equal to the texel block size of the compressed format.
Such a copy does not perform on-the-fly compression or decompression.
When copying from an uncompressed format to a compressed format, each texel
of uncompressed data of the source image is copied as a raw value to the
corresponding compressed texel block of the destination image.
When copying from a compressed format to an uncompressed format, each
compressed texel block of the source image is copied as a raw value to the
corresponding texel of uncompressed data in the destination image.
Thus, for example, it is legal to copy between a 128-bit uncompressed format
and a compressed format which has a 128-bit sized compressed texel block
representing 4×4 texels (using 8 bits per texel), or between a 64-bit
uncompressed format and a compressed format which has a 64-bit sized
compressed texel block representing 4×4 texels (using 4 bits per
texel).When copying between compressed and uncompressed formats the `extent`
members represent the texel dimensions of the source image and not the
destination.
When copying from a compressed image to an uncompressed image the image
texel dimensions written to the uncompressed image will be source extent
divided by the compressed texel block dimensions.
When copying from an uncompressed image to a compressed image the image
texel dimensions written to the compressed image will be the source extent
multiplied by the compressed texel block dimensions.
In both cases the number of bytes read and the number of bytes written will
be identical.Copying to or from block-compressed images is typically done in multiples of
the compressed texel block size.
For this reason the `extent` **must**  be a multiple of the compressed texel
block dimension.
There is one exception to this rule which is  **required**  to handle compressed
images created with dimensions that are not a multiple of the compressed
texel block dimensions: if the [`src_image`] is compressed, then:
- If `extent.width` is not a multiple of the compressed texel block width, then (`extent.width` +  `srcOffset.x`) **must**  equal the image subresource width.
- If `extent.height` is not a multiple of the compressed texel block height, then (`extent.height` +  `srcOffset.y`) **must**  equal the image subresource height.
- If `extent.depth` is not a multiple of the compressed texel block depth, then (`extent.depth` +  `srcOffset.z`) **must**  equal the image subresource depth.
Similarly, if the [`dst_image`] is compressed, then:
- If `extent.width` is not a multiple of the compressed texel block width, then (`extent.width` +  `dstOffset.x`) **must**  equal the image subresource width.
- If `extent.height` is not a multiple of the compressed texel block height, then (`extent.height` +  `dstOffset.y`) **must**  equal the image subresource height.
- If `extent.depth` is not a multiple of the compressed texel block depth, then (`extent.depth` +  `dstOffset.z`) **must**  equal the image subresource depth.
This allows the last compressed texel block of the image in each
non-multiple dimension to be included as a source or destination of the
copy.“`_422`” image formats that are not
[*multi-planar*](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) are treated as
having a 2×1 compressed texel block for the purposes of these rules.[`cmd_copy_image`] **can**  be used to copy image data between multisample
images, but both images  **must**  have the same number of samples.
## Valid Usage
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`src_image`] **must**  not be a protected image
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`dst_image`] **must**  not be a protected image
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not supported, [`dst_image`] **must**  not be an unprotected image

-    The union of all source regions, and the union of all destination regions, specified by the elements of [`p_regions`],  **must**  not overlap in memory
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`src_image`] **must**  contain `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT`
-    If [`src_image`] is non-sparse then the image or *disjoint* plane to be copied  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`src_image_layout`] **must**  specify the layout of the image subresources of [`src_image`] specified in [`p_regions`] at the time this command is executed on a [`Device`]
-  [`src_image_layout`] **must**  be `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`] **must**  contain `VK_FORMAT_FEATURE_TRANSFER_DST_BIT`
-    If [`dst_image`] is non-sparse then the image or *disjoint* plane that is the destination of the copy  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_image_layout`] **must**  specify the layout of the image subresources of [`dst_image`] specified in [`p_regions`] at the time this command is executed on a [`Device`]
-  [`dst_image_layout`] **must**  be `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`, `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
-    If the [`Format`] of each of [`src_image`] and [`dst_image`] is not a [*multi-planar format*](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), the [`Format`] of each of [`src_image`] and [`dst_image`] **must**  be compatible, as defined [above](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#copies-images-format-compatibility)
-    In a copy to or from a plane of a [multi-planar image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), the [`Format`] of the image and plane  **must**  be compatible according to [the description of compatible planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-compatible-planes) for the plane being copied
-    The sample count of [`src_image`] and [`dst_image`] **must**  match
-    The `srcSubresource.mipLevel` member of each element of [`p_regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.mipLevel` member of each element of [`p_regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of [`p_regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of [`p_regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `srcOffset` and `extent` members of each element of [`p_regions`] **must**  respect the image transfer granularity requirements of [`command_buffer`]’s command pool’s queue family, as described in [`QueueFamilyProperties`]
-    The `dstOffset` and `extent` members of each element of [`p_regions`] **must**  respect the image transfer granularity requirements of [`command_buffer`]’s command pool’s queue family, as described in [`QueueFamilyProperties`]
-  [`dst_image`] and [`src_image`] **must**  not have been created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
-    If neither [`src_image`] nor [`dst_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`p_regions`], `srcSubresource.aspectMask` and `dstSubresource.aspectMask` **must**  match
-    If [`src_image`] has a [`Format`] with [two planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`p_regions`], `srcSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`
-    If [`src_image`] has a [`Format`] with [three planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`p_regions`], `srcSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If [`dst_image`] has a [`Format`] with [two planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`p_regions`], `dstSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`
-    If [`dst_image`] has a [`Format`] with [three planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`p_regions`], `dstSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If [`src_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and the [`dst_image`] does not have a multi-planar image format, then for each element of [`p_regions`], `dstSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`dst_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and the [`src_image`] does not have a multi-planar image format, then for each element of [`p_regions`], `srcSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`src_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`p_regions`], `srcSubresource.baseArrayLayer` **must**  be `0` and `srcSubresource.layerCount` **must**  be `1`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`p_regions`], `dstSubresource.baseArrayLayer` **must**  be `0` and `dstSubresource.layerCount` **must**  be `1`
-    For each element of [`p_regions`], `srcSubresource.aspectMask` **must**  specify aspects present in [`src_image`]
-    For each element of [`p_regions`], `dstSubresource.aspectMask` **must**  specify aspects present in [`dst_image`]
-    For each element of [`p_regions`], `srcOffset.x` and (`extent.width` +  `srcOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `srcSubresource` of [`src_image`]
-    For each element of [`p_regions`], `srcOffset.y` and (`extent.height` +  `srcOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `srcOffset.y` **must**  be `0` and `extent.height` **must**  be `1`
-    For each element of [`p_regions`], `srcOffset.z` and (`extent.depth` +  `srcOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `srcOffset.z` **must**  be `0` and `extent.depth` **must**  be `1`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `dstOffset.z` **must**  be `0` and `extent.depth` **must**  be `1`
-    If [`src_image`] is of type `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `srcOffset.z` **must**  be `0`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `dstOffset.z` **must**  be `0`
-    If [`src_image`] and [`dst_image`] are both of type `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `extent.depth` **must**  be `1`
-    If [`src_image`] is of type `VK_IMAGE_TYPE_2D`, and [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`p_regions`], `extent.depth` **must**  equal `srcSubresource.layerCount`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_2D`, and [`src_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`p_regions`], `extent.depth` **must**  equal `dstSubresource.layerCount`
-    For each element of [`p_regions`], `dstOffset.x` and (`extent.width` +  `dstOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `dstSubresource` of [`dst_image`]
-    For each element of [`p_regions`], `dstOffset.y` and (`extent.height` +  `dstOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `dstOffset.y` **must**  be `0` and `extent.height` **must**  be `1`
-    For each element of [`p_regions`], `dstOffset.z` and (`extent.depth` +  `dstOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `dstSubresource` of [`dst_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], all members of `srcOffset` **must**  be a multiple of the corresponding dimensions of the compressed texel block
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], `extent.width` **must**  be a multiple of the compressed texel block width or (`extent.width` +  `srcOffset.x`) **must**  equal the width of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], `extent.height` **must**  be a multiple of the compressed texel block height or (`extent.height` +  `srcOffset.y`) **must**  equal the height of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], `extent.depth` **must**  be a multiple of the compressed texel block depth or (`extent.depth` +  `srcOffset.z`) **must**  equal the depth of the specified `srcSubresource` of [`src_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], all members of `dstOffset` **must**  be a multiple of the corresponding dimensions of the compressed texel block
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], `extent.width` **must**  be a multiple of the compressed texel block width or (`extent.width` +  `dstOffset.x`) **must**  equal the width of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], `extent.height` **must**  be a multiple of the compressed texel block height or (`extent.height` +  `dstOffset.y`) **must**  equal the height of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`p_regions`], `extent.depth` **must**  be a multiple of the compressed texel block depth or (`extent.depth` +  `dstOffset.z`) **must**  equal the depth of the specified `dstSubresource` of [`dst_image`]
-    If the `aspect` member of any element of `pRanges` includes any flag other than `VK_IMAGE_ASPECT_STENCIL_BIT` or [`src_image`] was not created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` **must**  have been included in the [`ImageCreateInfo::usage`] used to create [`src_image`]
-    If the `aspect` member of any element of `pRanges` includes any flag other than `VK_IMAGE_ASPECT_STENCIL_BIT` or [`dst_image`] was not created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_DST_BIT` **must**  have been included in the [`ImageCreateInfo::usage`] used to create [`dst_image`]
-    If the `aspect` member of any element of `pRanges` includes `VK_IMAGE_ASPECT_STENCIL_BIT`, and [`src_image`] was created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` **must**  have been included in the [`ImageStencilUsageCreateInfo::stencil_usage`] used to create [`src_image`]
-    If the `aspect` member of any element of `pRanges` includes `VK_IMAGE_ASPECT_STENCIL_BIT`, and [`dst_image`] was created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_DST_BIT` **must**  have been included in the [`ImageStencilUsageCreateInfo::stencil_usage`] used to create [`dst_image`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`src_image`] **must**  be a valid [`Image`] handle
-  [`src_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`dst_image`] **must**  be a valid [`Image`] handle
-  [`dst_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`p_regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`ImageCopy`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance
-  [`region_count`] **must**  be greater than `0`
-    Each of [`command_buffer`], [`dst_image`], and [`src_image`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`Image`]
- [`ImageCopy`]
- [`ImageLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        