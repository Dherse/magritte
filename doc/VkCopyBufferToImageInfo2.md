[VkCopyBufferToImageInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferToImageInfo2.html) - Structure specifying parameters of a buffer to image copy command

# C Specifications
The [`CopyBufferToImageInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkCopyBufferToImageInfo2 {
    VkStructureType              sType;
    const void*                  pNext;
    VkBuffer                     srcBuffer;
    VkImage                      dstImage;
    VkImageLayout                dstImageLayout;
    uint32_t                     regionCount;
    const VkBufferImageCopy2*    pRegions;
} VkCopyBufferToImageInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkCopyBufferToImageInfo2 VkCopyBufferToImageInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_buffer`] is the source buffer.
- [`dst_image`] is the destination image.
- [`dst_image_layout`] is the layout of the destination image subresources for the copy.
- [`region_count`] is the number of regions to copy.
- [`regions`] is a pointer to an array of [`BufferImageCopy2`] structures specifying the regions to copy.

# Description
## Valid Usage
-    If the image region specified by each element of [`regions`] does not contain [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, it  **must**  be a region that is contained within the specified `imageSubresource` of [`dst_image`]
-    If the image region specified by each element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, the rotated destination region as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-buffers-images-rotation-addressing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-buffers-images-rotation-addressing) **must**  be contained within [`dst_image`]
-    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`dst_image`] **must**  not be a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#blocked-image)
-    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`dst_image`] **must**  be of type `VK_IMAGE_TYPE_2D`
-    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`dst_image`] **must**  not have a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)

-  [`src_buffer`] **must**  be large enough to contain all buffer locations that are accessed according to [Buffer and Image Addressing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#copies-buffers-images-addressing), for each element of [`regions`]
-    The union of all source regions, and the union of all destination regions, specified by the elements of [`regions`],  **must**  not overlap in memory
-  [`src_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` usage flag
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`] **must**  contain `VK_FORMAT_FEATURE_TRANSFER_DST_BIT`
-    If [`src_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_image`] **must**  have a sample count equal to `VK_SAMPLE_COUNT_1_BIT`
-  [`dst_image_layout`] **must**  specify the layout of the image subresources of [`dst_image`] specified in [`regions`] at the time this command is executed on a [`Device`]
-  [`dst_image_layout`] **must**  be `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`, `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
-    The `imageSubresource.mipLevel` member of each element of [`regions`] **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `imageSubresource.baseArrayLayer` +  `imageSubresource.layerCount` of each element of [`regions`] **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`dst_image`] was created
-    The `imageOffset` and `imageExtent` members of each element of [`regions`] **must**  respect the image transfer granularity requirements of `commandBuffer`’s command pool’s queue family, as described in [`QueueFamilyProperties`]
-  [`dst_image`] **must**  not have been created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
-    If the queue family used to create the [`CommandPool`] which `commandBuffer` was allocated from does not support `VK_QUEUE_GRAPHICS_BIT`, for each element of [`regions`], the `aspectMask` member of `imageSubresource` **must**  not be `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT`
-    For each element of [`regions`] not containing [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, `imageOffset.x` and (`imageExtent.width` +  `imageOffset.x`) **must**  both be greater than or equal to `0` and less than or equal to the width of the specified `imageSubresource` of [`dst_image`]
-    For each element of [`regions`] not containing [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, `imageOffset.y` and (`imageExtent.height` +  `imageOffset.y`) **must**  both be greater than or equal to `0` and less than or equal to the height of the specified `imageSubresource` of [`dst_image`]

-    If [`dst_image`] does not have either a depth/stencil or a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), then for each element of [`regions`], `bufferOffset` **must**  be a multiple of the format’s texel block size
-    If [`dst_image`] has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), then for each element of [`regions`], `bufferOffset` **must**  be a multiple of the element size of the compatible format for the format and the `aspectMask` of the `imageSubresource` as defined in [[formats-compatible-planes]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-compatible-planes)
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`], `imageOffset.y` **must**  be `0` and `imageExtent.height` **must**  be `1`
-    For each element of [`regions`], `imageOffset.z` and (`imageExtent.depth` +  `imageOffset.z`) **must**  both be greater than or equal to `0` and less than or equal to the depth of the specified `imageSubresource` of [`dst_image`]
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of [`regions`], `imageOffset.z` **must**  be `0` and `imageExtent.depth` **must**  be `1`
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `bufferRowLength` **must**  be a multiple of the compressed texel block width
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `bufferImageHeight` **must**  be a multiple of the compressed texel block height
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], all members of `imageOffset` **must**  be a multiple of the corresponding dimensions of the compressed texel block
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `bufferOffset` **must**  be a multiple of the compressed texel block size in bytes
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `imageExtent.width` **must**  be a multiple of the compressed texel block width or (`imageExtent.width` +  `imageOffset.x`) **must**  equal the width of the specified `imageSubresource` of [`dst_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `imageExtent.height` **must**  be a multiple of the compressed texel block height or (`imageExtent.height` +  `imageOffset.y`) **must**  equal the height of the specified `imageSubresource` of [`dst_image`]
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `imageExtent.depth` **must**  be a multiple of the compressed texel block depth or (`imageExtent.depth` +  `imageOffset.z`) **must**  equal the depth of the specified `imageSubresource` of [`dst_image`]
-    For each element of [`regions`], `imageSubresource.aspectMask` **must**  specify aspects present in [`dst_image`]
-    If [`dst_image`] has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), then for each element of [`regions`], `imageSubresource.aspectMask` **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT` (with `VK_IMAGE_ASPECT_PLANE_2_BIT` valid only for image formats with three planes)
-    If [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, for each element of [`regions`], `imageSubresource.baseArrayLayer` **must**  be `0` and `imageSubresource.layerCount` **must**  be `1`
-    If [`dst_image`] is not a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `bufferRowLength` multiplied by the texel block size of [`dst_image`] **must**  be less than or equal to 2<sup>31</sup>-1
-    If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image), for each element of [`regions`], `bufferRowLength` divided by the compressed texel block width and then multiplied by the texel block size of [`dst_image`] **must**  be less than or equal to 2<sup>31</sup>-1
-    If the queue family used to create the [`CommandPool`] which `commandBuffer` was allocated from does not support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`, the `bufferOffset` member of any element of [`regions`] **must**  be a multiple of `4`
-    If [`dst_image`] has a depth/stencil format, the `bufferOffset` member of any element of [`regions`] **must**  be a multiple of `4`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2`
-  [`p_next`] **must**  be `NULL`
-  [`src_buffer`] **must**  be a valid [`Buffer`] handle
-  [`dst_image`] **must**  be a valid [`Image`] handle
-  [`dst_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`BufferImageCopy2`] structures
-  [`region_count`] **must**  be greater than `0`
-    Both of [`dst_image`], and [`src_buffer`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`khr_copy_commands2`]
- [`crate::vulkan1_3`]
- [`Buffer`]
- [`BufferImageCopy2`]
- [`Image`]
- [`ImageLayout`]
- [`StructureType`]
- [`cmd_copy_buffer_to_image2`]
- [`cmd_copy_buffer_to_image2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        