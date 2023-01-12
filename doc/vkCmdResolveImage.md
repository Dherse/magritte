[vkCmdResolveImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage.html) - Resolve regions of an image

# C Specifications
To resolve a multisample color image to a non-multisample color image, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdResolveImage(
    VkCommandBuffer                             commandBuffer,
    VkImage                                     srcImage,
    VkImageLayout                               srcImageLayout,
    VkImage                                     dstImage,
    VkImageLayout                               dstImageLayout,
    uint32_t                                    regionCount,
    const VkImageResolve*                       pRegions);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`src_image`] is the source image.
- [`src_image_layout`] is the layout of the source image subresources for the resolve.
- [`dst_image`] is the destination image.
- [`dst_image_layout`] is the layout of the destination image subresources for the resolve.
- [`region_count`] is the number of regions to resolve.
- [`p_regions`] is a pointer to an array of [`ImageResolve`] structures specifying the regions to resolve.

# Description
During the resolve the samples corresponding to each pixel location in the
source are converted to a single sample before being written to the
destination.
If the source formats are floating-point or normalized types, the sample
values for each pixel are resolved in an implementation-dependent manner.
If the source formats are integer types, a single sample’s value is selected
for each pixel.`srcOffset` and `dstOffset` select the initial `x`, `y`, and
`z` offsets in texels of the sub-regions of the source and destination
image data.
`extent` is the size in texels of the source image to resolve in
`width`, `height` and `depth`.
Each element of [`p_regions`] **must**  be a region that is contained within
its corresponding image.Resolves are done layer by layer starting with `baseArrayLayer` member
of `srcSubresource` for the source and `dstSubresource` for the
destination.
`layerCount` layers are resolved to the destination image.
## Valid Usage
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`src_image`] **must**  not be a protected image
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`dst_image`] **must**  not be a protected image
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not supported, [`dst_image`] **must**  not be an unprotected image

-    The union of all source regions, and the union of all destination regions, specified by the elements of [`p_regions`],  **must**  not overlap in memory
-    If [`src_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`src_image`] **must**  have a sample count equal to any valid sample count value other than `VK_SAMPLE_COUNT_1_BIT`
-    If [`dst_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_image`] **must**  have a sample count equal to `VK_SAMPLE_COUNT_1_BIT`
-  [`src_image_layout`] **must**  specify the layout of the image subresources of [`src_image`] specified in [`p_regions`] at the time this command is executed on a [`Device`]
-  [`src_image_layout`] **must**  be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`, `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
-  [`dst_image_layout`] **must**  specify the layout of the image subresources of [`dst_image`] specified in [`p_regions`] at the time this command is executed on a [`Device`]
-  [`dst_image_layout`] **must**  be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`, `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`] **must**  contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    If the [`linearColorAttachment`]() feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, the [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`] **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-  [`src_image`] and [`dst_image`] **must**  have been created with the same image format
-    The `srcSubresource.mipLevel` member of each element of [`p_regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.mipLevel` member of each element of [`p_regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of [`p_regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of [`p_regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-  [`dst_image`] and [`src_image`] **must**  not have been created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
-    If either [`src_image`] or [`dst_image`] are of type `VK_IMAGE_TYPE_3D`, then for each element of [`p_regions`], `srcSubresource.baseArrayLayer` **must**  be `0` and `srcSubresource.layerCount` **must**  be `1`
-    If either [`src_image`] or [`dst_image`] are of type `VK_IMAGE_TYPE_3D`, then for each element of [`p_regions`], `dstSubresource.baseArrayLayer` **must**  be `0` and `dstSubresource.layerCount` **must**  be `1`
-    For each element of [`p_regions`], `srcOffset.x` and (`extent.width` +  `srcOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `srcSubresource` of [`src_image`]
-    For each element of [`p_regions`], `srcOffset.y` and (`extent.height` +  `srcOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `srcOffset.y` **must**  be `0` and `extent.height` **must**  be `1`
-    For each element of [`p_regions`], `srcOffset.z` and (`extent.depth` +  `srcOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `srcSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `srcOffset.z` **must**  be `0` and `extent.depth` **must**  be `1`
-    For each element of [`p_regions`], `dstOffset.x` and (`extent.width` +  `dstOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `dstSubresource` of [`dst_image`]
-    For each element of [`p_regions`], `dstOffset.y` and (`extent.height` +  `dstOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `dstOffset.y` **must**  be `0` and `extent.height` **must**  be `1`
-    For each element of [`p_regions`], `dstOffset.z` and (`extent.depth` +  `dstOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `dstSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `dstOffset.z` **must**  be `0` and `extent.depth` **must**  be `1`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`src_image`] **must**  be a valid [`Image`] handle
-  [`src_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`dst_image`] **must**  be a valid [`Image`] handle
-  [`dst_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`p_regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`ImageResolve`] structures
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
- [`Image`]
- [`ImageLayout`]
- [`ImageResolve`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        