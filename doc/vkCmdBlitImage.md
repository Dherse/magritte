[vkCmdBlitImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBlitImage.html) - Copy regions of an image, potentially performing format conversion,

# C Specifications
To copy regions of a source image into a destination image, potentially
performing format conversion, arbitrary scaling, and filtering, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdBlitImage(
    VkCommandBuffer                             commandBuffer,
    VkImage                                     srcImage,
    VkImageLayout                               srcImageLayout,
    VkImage                                     dstImage,
    VkImageLayout                               dstImageLayout,
    uint32_t                                    regionCount,
    const VkImageBlit*                          pRegions,
    VkFilter                                    filter);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`src_image`] is the source image.
- [`src_image_layout`] is the layout of the source image subresources for the blit.
- [`dst_image`] is the destination image.
- [`dst_image_layout`] is the layout of the destination image subresources for the blit.
- [`region_count`] is the number of regions to blit.
- [`p_regions`] is a pointer to an array of [`ImageBlit`] structures specifying the regions to blit.
- [`filter`] is a [`Filter`] specifying the filter to apply if the blits require scaling.

# Description
[`cmd_blit_image`] **must**  not be used for multisampled source or
destination images.
Use [`cmd_resolve_image`] for this purpose.As the sizes of the source and destination extents  **can**  differ in any
dimension, texels in the source extent are scaled and filtered to the
destination extent.
Scaling occurs via the following operations:
- For each destination texel, the integer coordinate of that texel is converted to an unnormalized texture coordinate, using the effective inverse of the equations described in [unnormalized to integer conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer):  * u<sub>base</sub> = i +  ½  * v<sub>base</sub> = j +  ½  * w<sub>base</sub> = k +  ½ 
- These base coordinates are then offset by the first destination offset:  * u<sub>offset</sub> = u<sub>base</sub> - x<sub>dst0</sub>  * v<sub>offset</sub> = v<sub>base</sub> - y<sub>dst0</sub>  * w<sub>offset</sub> = w<sub>base</sub> - z<sub>dst0</sub>  * a<sub>offset</sub> = a - `baseArrayCount`<sub>dst</sub> 
- The scale is determined from the source and destination regions, and applied to the offset coordinates:  * scale<sub>u</sub> = (x<sub>src1</sub> - x<sub>src0</sub>) / (x<sub>dst1</sub> - x<sub>dst0</sub>)  * scale<sub>v</sub> = (y<sub>src1</sub> - y<sub>src0</sub>) / (y<sub>dst1</sub> - y<sub>dst0</sub>)  * scale<sub>w</sub> = (z<sub>src1</sub> - z<sub>src0</sub>) / (z<sub>dst1</sub> - z<sub>dst0</sub>)  * u<sub>scaled</sub> = u<sub>offset</sub> × scale<sub>u</sub>  * v<sub>scaled</sub> = v<sub>offset</sub> × scale<sub>v</sub>  * w<sub>scaled</sub> = w<sub>offset</sub> × scale<sub>w</sub> 
- Finally the source offset is added to the scaled coordinates, to determine the final unnormalized coordinates used to sample from [`src_image`]:  * u = u<sub>scaled</sub> +  x<sub>src0</sub>  * v = v<sub>scaled</sub> +  y<sub>src0</sub>  * w = w<sub>scaled</sub> +  z<sub>src0</sub>  * q = `mipLevel`  * a = a<sub>offset</sub> +  `baseArrayCount`<sub>src</sub> 
These coordinates are used to sample from the source image, as described in
[Image Operations chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures), with the filter mode equal to that
of [`filter`], a mipmap mode of `VK_SAMPLER_MIPMAP_MODE_NEAREST` and
an address mode of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`.
Implementations  **must**  clamp at the edge of the source image, and  **may** 
additionally clamp to the edge of the source region.Blits are done layer by layer starting with the `baseArrayLayer` member
of `srcSubresource` for the source and `dstSubresource` for the
destination.
`layerCount` layers are blitted to the destination image.When blitting 3D textures, slices in the destination region bounded by
`dstOffsets`[0].z and `dstOffsets`[1].z are sampled from slices in
the source region bounded by `srcOffsets`[0].z and
`srcOffsets`[1].z.
If the [`filter`] parameter is `VK_FILTER_LINEAR` then the value
sampled from the source image is taken by doing linear filtering using the
interpolated  **z**  coordinate represented by  **w**  in the previous equations.
If the [`filter`] parameter is `VK_FILTER_NEAREST` then the value
sampled from the source image is taken from the single nearest slice, with
an implementation-dependent arithmetic rounding mode.The following filtering and conversion rules apply:
- Integer formats  **can**  only be converted to other integer formats with the same signedness.
- No format conversion is supported between depth/stencil images. The formats  **must**  match.
- Format conversions on unorm, snorm, scaled and packed float formats of the copied aspect of the image are performed by first converting the pixels to float values.
- For sRGB source formats, nonlinear RGB values are converted to linear representation prior to filtering.
- After filtering, the float values are first clamped and then cast to the destination image format. In case of sRGB destination format, linear RGB values are converted to nonlinear representation before writing the pixel to the image.
Signed and unsigned integers are converted by first clamping to the
representable range of the destination format, then casting the value.
## Valid Usage
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`src_image`] **must**  not be a protected image
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`dst_image`] **must**  not be a protected image
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not supported, [`dst_image`] **must**  not be an unprotected image

-    The source region specified by each element of [`p_regions`] **must**  be a region that is contained within [`src_image`]
-    The destination region specified by each element of [`p_regions`] **must**  be a region that is contained within [`dst_image`]
-    The union of all destination regions, specified by the elements of [`p_regions`],  **must**  not overlap in memory with any texel that  **may**  be sampled during the blit operation
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`src_image`] **must**  contain `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
-  [`src_image`] **must**  not use a [format that requires a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
-  [`src_image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage flag
-    If [`src_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`src_image_layout`] **must**  specify the layout of the image subresources of [`src_image`] specified in [`p_regions`] at the time this command is executed on a [`Device`]
-  [`src_image_layout`] **must**  be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`, `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`] **must**  contain `VK_FORMAT_FEATURE_BLIT_DST_BIT`
-  [`dst_image`] **must**  not use a [format that requires a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
-  [`dst_image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_image_layout`] **must**  specify the layout of the image subresources of [`dst_image`] specified in [`p_regions`] at the time this command is executed on a [`Device`]
-  [`dst_image_layout`] **must**  be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`, `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
-    If either of [`src_image`] or [`dst_image`] was created with a signed integer [`Format`], the other  **must**  also have been created with a signed integer [`Format`]
-    If either of [`src_image`] or [`dst_image`] was created with an unsigned integer [`Format`], the other  **must**  also have been created with an unsigned integer [`Format`]
-    If either of [`src_image`] or [`dst_image`] was created with a depth/stencil format, the other  **must**  have exactly the same format
-    If [`src_image`] was created with a depth/stencil format, [`filter`] **must**  be `VK_FILTER_NEAREST`
-  [`src_image`] **must**  have been created with a `samples` value of `VK_SAMPLE_COUNT_1_BIT`
-  [`dst_image`] **must**  have been created with a `samples` value of `VK_SAMPLE_COUNT_1_BIT`
-    If [`filter`] is `VK_FILTER_LINEAR`, then the [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`src_image`] **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
-    If [`filter`] is `VK_FILTER_CUBIC_EXT`, then the [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`src_image`] **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
-    If [`filter`] is `VK_FILTER_CUBIC_EXT`, [`src_image`] **must**  be of type `VK_IMAGE_TYPE_2D`
-    The `srcSubresource.mipLevel` member of each element of [`p_regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.mipLevel` member of each element of [`p_regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of [`p_regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of [`p_regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-  [`dst_image`] and [`src_image`] **must**  not have been created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
-    If either [`src_image`] or [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`p_regions`], `srcSubresource.baseArrayLayer` and `dstSubresource.baseArrayLayer` **must**  each be `0`, and `srcSubresource.layerCount` and `dstSubresource.layerCount` **must**  each be `1`
-    For each element of [`p_regions`], `srcSubresource.aspectMask` **must**  specify aspects present in [`src_image`]
-    For each element of [`p_regions`], `dstSubresource.aspectMask` **must**  specify aspects present in [`dst_image`]
-    For each element of [`p_regions`], `srcOffsets`[0].x and `srcOffsets`[1].x  **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `srcSubresource` of [`src_image`]
-    For each element of [`p_regions`], `srcOffsets`[0].y and `srcOffsets`[1].y  **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `srcOffsets`[0].y  **must**  be `0` and `srcOffsets`[1].y  **must**  be `1`
-    For each element of [`p_regions`], `srcOffsets`[0].z and `srcOffsets`[1].z  **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `srcOffsets`[0].z  **must**  be `0` and `srcOffsets`[1].z  **must**  be `1`
-    For each element of [`p_regions`], `dstOffsets`[0].x and `dstOffsets`[1].x  **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `dstSubresource` of [`dst_image`]
-    For each element of [`p_regions`], `dstOffsets`[0].y and `dstOffsets`[1].y  **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `dstOffsets`[0].y  **must**  be `0` and `dstOffsets`[1].y  **must**  be `1`
-    For each element of [`p_regions`], `dstOffsets`[0].z and `dstOffsets`[1].z  **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `dstOffsets`[0].z  **must**  be `0` and `dstOffsets`[1].z  **must**  be `1`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`src_image`] **must**  be a valid [`Image`] handle
-  [`src_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`dst_image`] **must**  be a valid [`Image`] handle
-  [`dst_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`p_regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`ImageBlit`] structures
-  [`filter`] **must**  be a valid [`Filter`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
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
- [`Filter`]
- [`Image`]
- [`ImageBlit`]
- [`ImageLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        