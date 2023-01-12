[vkCmdCopyImageToBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyImageToBuffer.html) - Copy image data into a buffer

# C Specifications
To copy data from an image object to a buffer object, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdCopyImageToBuffer(
    VkCommandBuffer                             commandBuffer,
    VkImage                                     srcImage,
    VkImageLayout                               srcImageLayout,
    VkBuffer                                    dstBuffer,
    uint32_t                                    regionCount,
    const VkBufferImageCopy*                    pRegions);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`src_image`] is the source image.
- [`src_image_layout`] is the layout of the source image subresources for the copy.
- [`dst_buffer`] is the destination buffer.
- [`region_count`] is the number of regions to copy.
- [`p_regions`] is a pointer to an array of [`BufferImageCopy`] structures specifying the regions to copy.

# Description
Each region in [`p_regions`] is copied from the specified region of the
source image to the specified region of the destination buffer.If [`src_image`] has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), regions of each plane to be a source of a copy  **must** 
be specified separately using the [`p_regions`] member of the
[`BufferImageCopy`] structure.
In this case, the `aspectMask` of `imageSubresource` **must**  be
`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or
`VK_IMAGE_ASPECT_PLANE_2_BIT`.
For the purposes of [`cmd_copy_buffer_to_image`], each plane of a
multi-planar image is treated as having the format listed in
[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatible-planes) for the plane identified by the
`aspectMask` of the corresponding subresource.
This applies both to [`Format`] and to coordinates used in the copy,
which correspond to texels in the *plane* rather than how these texels map
to coordinates in the image as a whole.
## Valid Usage
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`src_image`] **must**  not be a protected image
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`dst_buffer`] **must**  not be a protected buffer
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not supported, [`dst_buffer`] **must**  not be an unprotected buffer

-    The image region specified by each element of [`p_regions`] **must**  be contained within the specified `imageSubresource` of [`src_image`]

-  [`dst_buffer`] **must**  be large enough to contain all buffer locations that are accessed according to [Buffer and Image Addressing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#copies-buffers-images-addressing), for each element of [`p_regions`]
-    The union of all source regions, and the union of all destination regions, specified by the elements of [`p_regions`],  **must**  not overlap in memory
-  [`src_image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage flag
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`src_image`] **must**  contain `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT`
-    If [`src_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`src_image`] **must**  have a sample count equal to `VK_SAMPLE_COUNT_1_BIT`
-  [`src_image_layout`] **must**  specify the layout of the image subresources of [`src_image`] specified in [`p_regions`] at the time this command is executed on a [`Device`]
-  [`src_image_layout`] **must**  be `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
-    The `imageSubresource.mipLevel` member of each element of [`p_regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `imageSubresource.baseArrayLayer` +  `imageSubresource.layerCount` of each element of [`p_regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`src_image`] was created
-    The `imageOffset` and `imageExtent` members of each element of [`p_regions`] **must**  respect the image transfer granularity requirements of [`command_buffer`]’s command pool’s queue family, as described in [`QueueFamilyProperties`]
-  [`src_image`] **must**  not have been created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
-    For each element of [`p_regions`], `imageOffset.x` and (`imageExtent.width` +  `imageOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `imageSubresource` of [`src_image`]
-    For each element of [`p_regions`], `imageOffset.y` and (imageExtent.height +  `imageOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `imageSubresource` of [`src_image`]

-    If [`src_image`] does not have either a depth/stencil or a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), then for each element of [`p_regions`], `bufferOffset` **must**  be a multiple of the format’s texel block size
-    If [`src_image`] has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), then for each element of [`p_regions`], `bufferOffset` **must**  be a multiple of the element size of the compatible format for the format and the `aspectMask` of the `imageSubresource` as defined in [[formats-compatible-planes]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-compatible-planes)
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`p_regions`], `imageOffset.y` **must**  be `0` and `imageExtent.height` **must**  be `1`
-    For each element of [`p_regions`], `imageOffset.z` and (`imageExtent.depth` +  `imageOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `imageSubresource` of [`src_image`]
-    If [`src_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`p_regions`], `imageOffset.z` **must**  be `0` and `imageExtent.depth` **must**  be `1`
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `bufferRowLength` **must**  be a multiple of the compressed texel block width
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `bufferImageHeight` **must**  be a multiple of the compressed texel block height
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], all members of `imageOffset` **must**  be a multiple of the corresponding dimensions of the compressed texel block
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `bufferOffset` **must**  be a multiple of the compressed texel block size in bytes
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `imageExtent.width` **must**  be a multiple of the compressed texel block width or (`imageExtent.width` +  `imageOffset.x`) **must**  equal the width of the specified `imageSubresource` of [`src_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `imageExtent.height` **must**  be a multiple of the compressed texel block height or (`imageExtent.height` +  `imageOffset.y`) **must**  equal the height of the specified `imageSubresource` of [`src_image`]
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `imageExtent.depth` **must**  be a multiple of the compressed texel block depth or (`imageExtent.depth` +  `imageOffset.z`) **must**  equal the depth of the specified `imageSubresource` of [`src_image`]
-    For each element of [`p_regions`], `imageSubresource.aspectMask` **must**  specify aspects present in [`src_image`]
-    If [`src_image`] has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), then for each element of [`p_regions`], `imageSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT` (with `VK_IMAGE_ASPECT_PLANE_2_BIT` valid only for image formats with three planes)
-    If [`src_image`] is of type `VK_IMAGE_TYPE_3D`, for each element of [`p_regions`], `imageSubresource.baseArrayLayer` **must**  be `0` and `imageSubresource.layerCount` **must**  be `1`
-    If [`src_image`] is not a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `bufferRowLength` multiplied by the texel block size of [`src_image`] **must**  be less than or equal to 2<sup>31</sup>-1
-    If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`p_regions`], `bufferRowLength` divided by the compressed texel block width and then multiplied by the texel block size of [`src_image`] **must**  be less than or equal to 2<sup>31</sup>-1
-    If the queue family used to create the [`CommandPool`] which [`command_buffer`] was allocated from does not support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`, the `bufferOffset` member of any element of [`p_regions`] **must**  be a multiple of `4`
-    If [`src_image`] has a depth/stencil format, the `bufferOffset` member of any element of [`p_regions`] **must**  be a multiple of `4`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`src_image`] **must**  be a valid [`Image`] handle
-  [`src_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`dst_buffer`] **must**  be a valid [`Buffer`] handle
-  [`p_regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`BufferImageCopy`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance
-  [`region_count`] **must**  be greater than `0`
-    Each of [`command_buffer`], [`dst_buffer`], and [`src_image`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`BufferImageCopy`]
- [`CommandBuffer`]
- [`Image`]
- [`ImageLayout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        