[VkCopyImageInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageInfo2.html) - Structure specifying parameters of an image copy command

# C Specifications
The [`CopyImageInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkCopyImageInfo2 {
    VkStructureType        sType;
    const void*            pNext;
    VkImage                srcImage;
    VkImageLayout          srcImageLayout;
    VkImage                dstImage;
    VkImageLayout          dstImageLayout;
    uint32_t               regionCount;
    const VkImageCopy2*    pRegions;
} VkCopyImageInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkCopyImageInfo2 VkCopyImageInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_image`] is the source image.
- [`src_image_layout`] is the current layout of the source image subresource.
- [`dst_image`] is the destination image.
- [`dst_image_layout`] is the current layout of the destination image subresource.
- [`region_count`] is the number of regions to copy.
- [`regions`] is a pointer to an array of [`ImageCopy2`] structures specifying the regions to copy.

# Description
## Valid Usage
-    The union of all source regions, and the union of all destination regions, specified by the elements of [`regions`],  **must**  not overlap in memory
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`src_image`] **must**  contain `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT`
-    If [`src_image`] is non-sparse then the image or *disjoint* plane to be copied  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`src_image_layout`] **must**  specify the layout of the image subresources of [`src_image`] specified in [`regions`] at the time this command is executed on a [`Device`]
-  [`src_image_layout`] **must**  be `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`] **must**  contain `VK_FORMAT_FEATURE_TRANSFER_DST_BIT`
-    If [`dst_image`] is non-sparse then the image or *disjoint* plane that is the destination of the copy  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_image_layout`] **must**  specify the layout of the image subresources of [`dst_image`] specified in [`regions`] at the time this command is executed on a [`Device`]
-  [`dst_image_layout`] **must**  be `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`, `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
-    If the [`Format`] of each of [`src_image`] and [`dst_image`] is not a [*multi-planar format*](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), the [`Format`] of each of [`src_image`] and [`dst_image`] **must**  be compatible, as defined [above](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#copies-images-format-compatibility)
-    In a copy to or from a plane of a [multi-planar image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), the [`Format`] of the image and plane  **must**  be compatible according to [the description of compatible planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-compatible-planes) for the plane being copied
-    The sample count of [`src_image`] and [`dst_image`] **must**  match
-    The `srcSubresource.mipLevel` member of each element of [`regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.mipLevel` member of each element of [`regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of [`regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of [`regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `srcOffset` and `extent` members of each element of [`regions`] **must**  respect the image transfer granularity requirements of `commandBuffer`’s command pool’s queue family, as described in [`QueueFamilyProperties`]
-    The `dstOffset` and `extent` members of each element of [`regions`] **must**  respect the image transfer granularity requirements of `commandBuffer`’s command pool’s queue family, as described in [`QueueFamilyProperties`]
-  [`dst_image`] and [`src_image`] **must**  not have been created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
-    If neither [`src_image`] nor [`dst_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`regions`], `srcSubresource.aspectMask` and `dstSubresource.aspectMask` **must**  match
-    If [`src_image`] has a [`Format`] with [two planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`regions`], `srcSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`
-    If [`src_image`] has a [`Format`] with [three planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`regions`], `srcSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If [`dst_image`] has a [`Format`] with [two planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`regions`], `dstSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`
-    If [`dst_image`] has a [`Format`] with [three planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) then for each element of [`regions`], `dstSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If [`src_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and the [`dst_image`] does not have a multi-planar image format, then for each element of [`regions`], `dstSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`dst_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and the [`src_image`] does not have a multi-planar image format, then for each element of [`regions`], `srcSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`src_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`regions`], `srcSubresource.baseArrayLayer` **must**  be `0` and `srcSubresource.layerCount` **must**  be `1`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`regions`], `dstSubresource.baseArrayLayer` **must**  be `0` and `dstSubresource.layerCount` **must**  be `1`
-    For each element of [`regions`], `srcSubresource.aspectMask` **must**  specify aspects present in [`src_image`]
-    For each element of [`regions`], `dstSubresource.aspectMask` **must**  specify aspects present in [`dst_image`]
-    For each element of [`regions`], `srcOffset.x` and (`extent.width` +  `srcOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `srcSubresource` of [`src_image`]
-    For each element of [`regions`], `srcOffset.y` and (`extent.height` +  `srcOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`], `srcOffset.y` **must**  be `0` and `extent.height` **must**  be `1`
-    For each element of [`regions`], `srcOffset.z` and (`extent.depth` +  `srcOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`], `srcOffset.z` **must**  be `0` and `extent.depth` **must**  be `1`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`], `dstOffset.z` **must**  be `0` and `extent.depth` **must**  be `1`
-    If [`src_image`] is of type `VK_IMAGE_TYPE_2D`, then for each element of [`regions`], `srcOffset.z` **must**  be `0`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_2D`, then for each element of [`regions`], `dstOffset.z` **must**  be `0`
-    If [`src_image`] and [`dst_image`] are both of type `VK_IMAGE_TYPE_2D`, then for each element of [`regions`], `extent.depth` **must**  be `1`
-    If [`src_image`] is of type `VK_IMAGE_TYPE_2D`, and [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`regions`], `extent.depth` **must**  equal `srcSubresource.layerCount`
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_2D`, and [`src_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`regions`], `extent.depth` **must**  equal `dstSubresource.layerCount`
-    For each element of [`regions`], `dstOffset.x` and (`extent.width` +  `dstOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `dstSubresource` of [`dst_image`]
-    For each element of [`regions`], `dstOffset.y` and (`extent.height` +  `dstOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`], `dstOffset.y` **must**  be `0` and `extent.height` **must**  be `1`
-    For each element of [`regions`], `dstOffset.z` and (`extent.depth` +  `dstOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `dstSubresource` of [`dst_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], all members of `srcOffset` **must**  be a multiple of the corresponding dimensions of the compressed texel block
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], `extent.width` **must**  be a multiple of the compressed texel block width or (`extent.width` +  `srcOffset.x`) **must**  equal the width of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], `extent.height` **must**  be a multiple of the compressed texel block height or (`extent.height` +  `srcOffset.y`) **must**  equal the height of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], `extent.depth` **must**  be a multiple of the compressed texel block depth or (`extent.depth` +  `srcOffset.z`) **must**  equal the depth of the specified `srcSubresource` of [`src_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], all members of `dstOffset` **must**  be a multiple of the corresponding dimensions of the compressed texel block
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], `extent.width` **must**  be a multiple of the compressed texel block width or (`extent.width` +  `dstOffset.x`) **must**  equal the width of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], `extent.height` **must**  be a multiple of the compressed texel block height or (`extent.height` +  `dstOffset.y`) **must**  equal the height of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), then for each element of [`regions`], `extent.depth` **must**  be a multiple of the compressed texel block depth or (`extent.depth` +  `dstOffset.z`) **must**  equal the depth of the specified `dstSubresource` of [`dst_image`]
-    If the `aspect` member of any element of `pRanges` includes any flag other than `VK_IMAGE_ASPECT_STENCIL_BIT` or [`src_image`] was not created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` **must**  have been included in the [`ImageCreateInfo::usage`] used to create [`src_image`]
-    If the `aspect` member of any element of `pRanges` includes any flag other than `VK_IMAGE_ASPECT_STENCIL_BIT` or [`dst_image`] was not created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_DST_BIT` **must**  have been included in the [`ImageCreateInfo::usage`] used to create [`dst_image`]
-    If the `aspect` member of any element of `pRanges` includes `VK_IMAGE_ASPECT_STENCIL_BIT`, and [`src_image`] was created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` **must**  have been included in the [`ImageStencilUsageCreateInfo::stencil_usage`] used to create [`src_image`]
-    If the `aspect` member of any element of `pRanges` includes `VK_IMAGE_ASPECT_STENCIL_BIT`, and [`dst_image`] was created with [separate stencil usage](), `VK_IMAGE_USAGE_TRANSFER_DST_BIT` **must**  have been included in the [`ImageStencilUsageCreateInfo::stencil_usage`] used to create [`dst_image`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2`
-  [`p_next`] **must**  be `NULL`
-  [`src_image`] **must**  be a valid [`Image`] handle
-  [`src_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`dst_image`] **must**  be a valid [`Image`] handle
-  [`dst_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`ImageCopy2`] structures
-  [`region_count`] **must**  be greater than `0`
-    Both of [`dst_image`], and [`src_image`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_KHR_copy_commands2`]
- [`crate::vulkan1_3`]
- [`Image`]
- [`ImageCopy2`]
- [`ImageLayout`]
- [`StructureType`]
- [`cmd_copy_image2`]
- [`cmd_copy_image2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        