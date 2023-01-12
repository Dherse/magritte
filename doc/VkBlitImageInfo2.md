[VkBlitImageInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlitImageInfo2.html) - Structure specifying parameters of blit image command

# C Specifications
The [`BlitImageInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkBlitImageInfo2 {
    VkStructureType        sType;
    const void*            pNext;
    VkImage                srcImage;
    VkImageLayout          srcImageLayout;
    VkImage                dstImage;
    VkImageLayout          dstImageLayout;
    uint32_t               regionCount;
    const VkImageBlit2*    pRegions;
    VkFilter               filter;
} VkBlitImageInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkBlitImageInfo2 VkBlitImageInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_image`] is the source image.
- [`src_image_layout`] is the layout of the source image subresources for the blit.
- [`dst_image`] is the destination image.
- [`dst_image_layout`] is the layout of the destination image subresources for the blit.
- [`region_count`] is the number of regions to blit.
- [`regions`] is a pointer to an array of [`ImageBlit2`] structures specifying the regions to blit.
- [`filter`] is a [`Filter`] specifying the filter to apply if the blits require scaling.

# Description
## Valid Usage
-    The source region specified by each element of [`regions`] **must**  be a region that is contained within [`src_image`]
-    The destination region specified by each element of [`regions`] **must**  be a region that is contained within [`dst_image`]
-    The union of all destination regions, specified by the elements of [`regions`],  **must**  not overlap in memory with any texel that  **may**  be sampled during the blit operation
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`src_image`] **must**  contain `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
-  [`src_image`] **must**  not use a [format that requires a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
-  [`src_image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage flag
-    If [`src_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`src_image_layout`] **must**  specify the layout of the image subresources of [`src_image`] specified in [`regions`] at the time this command is executed on a [`Device`]
-  [`src_image_layout`] **must**  be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`, `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`] **must**  contain `VK_FORMAT_FEATURE_BLIT_DST_BIT`
-  [`dst_image`] **must**  not use a [format that requires a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
-  [`dst_image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_image_layout`] **must**  specify the layout of the image subresources of [`dst_image`] specified in [`regions`] at the time this command is executed on a [`Device`]
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
-    The `srcSubresource.mipLevel` member of each element of [`regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.mipLevel` member of each element of [`regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of [`regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of [`regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-  [`dst_image`] and [`src_image`] **must**  not have been created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
-    If either [`src_image`] or [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`regions`], `srcSubresource.baseArrayLayer` and `dstSubresource.baseArrayLayer` **must**  each be `0`, and `srcSubresource.layerCount` and `dstSubresource.layerCount` **must**  each be `1`
-    For each element of [`regions`], `srcSubresource.aspectMask` **must**  specify aspects present in [`src_image`]
-    For each element of [`regions`], `dstSubresource.aspectMask` **must**  specify aspects present in [`dst_image`]
-    For each element of [`regions`], `srcOffsets`[0].x and `srcOffsets`[1].x  **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `srcSubresource` of [`src_image`]
-    For each element of [`regions`], `srcOffsets`[0].y and `srcOffsets`[1].y  **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`], `srcOffsets`[0].y  **must**  be `0` and `srcOffsets`[1].y  **must**  be `1`
-    For each element of [`regions`], `srcOffsets`[0].z and `srcOffsets`[1].z  **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`regions`], `srcOffsets`[0].z  **must**  be `0` and `srcOffsets`[1].z  **must**  be `1`
-    For each element of [`regions`], `dstOffsets`[0].x and `dstOffsets`[1].x  **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `dstSubresource` of [`dst_image`]
-    For each element of [`regions`], `dstOffsets`[0].y and `dstOffsets`[1].y  **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`], `dstOffsets`[0].y  **must**  be `0` and `dstOffsets`[1].y  **must**  be `1`
-    For each element of [`regions`], `dstOffsets`[0].z and `dstOffsets`[1].z  **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`regions`], `dstOffsets`[0].z  **must**  be `0` and `dstOffsets`[1].z  **must**  be `1`
-    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`src_image`] and [`dst_image`] **must**  not be block-compressed images
-    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`src_image`] **must**  be of type `VK_IMAGE_TYPE_2D`
-    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`src_image`] **must**  not have a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2`
-  [`p_next`] **must**  be `NULL`
-  [`src_image`] **must**  be a valid [`Image`] handle
-  [`src_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`dst_image`] **must**  be a valid [`Image`] handle
-  [`dst_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`ImageBlit2`] structures
-  [`filter`] **must**  be a valid [`Filter`] value
-  [`region_count`] **must**  be greater than `0`
-    Both of [`dst_image`], and [`src_image`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`khr_copy_commands2`]
- [`crate::vulkan1_3`]
- [`Filter`]
- [`Image`]
- [`ImageBlit2`]
- [`ImageLayout`]
- [`StructureType`]
- [`cmd_blit_image2`]
- [`cmd_blit_image2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        