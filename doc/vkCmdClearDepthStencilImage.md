[vkCmdClearDepthStencilImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearDepthStencilImage.html) - Fill regions of a combined depth/stencil image

# C Specifications
To clear one or more subranges of a depth/stencil image, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdClearDepthStencilImage(
    VkCommandBuffer                             commandBuffer,
    VkImage                                     image,
    VkImageLayout                               imageLayout,
    const VkClearDepthStencilValue*             pDepthStencil,
    uint32_t                                    rangeCount,
    const VkImageSubresourceRange*              pRanges);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`image`] is the image to be cleared.
- [`image_layout`] specifies the current layout of the image subresource ranges to be cleared, and  **must**  be `VK_IMAGE_LAYOUT_GENERAL` or `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`.
- [`p_depth_stencil`] is a pointer to a [`ClearDepthStencilValue`] structure containing the values that the depth and stencil image subresource ranges will be cleared to (see [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears-values](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears-values) below).
- [`range_count`] is the number of image subresource range structures in [`p_ranges`].
- [`p_ranges`] is a pointer to an array of [`ImageSubresourceRange`] structures describing a range of mipmap levels, array layers, and aspects to be cleared, as described in [Image Views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views).

# Description
## Valid Usage
-    The [format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-format-features) of [`image`] **must**  contain `VK_FORMAT_FEATURE_TRANSFER_DST_BIT`
-    If the `aspect` member of any element of [`p_ranges`] includes `VK_IMAGE_ASPECT_STENCIL_BIT`, and [`image`] was created with [`ImageStencilUsageCreateInfo`], `VK_IMAGE_USAGE_TRANSFER_DST_BIT` **must**  have been included in the [`ImageStencilUsageCreateInfo::stencil_usage`] used to create [`image`]
-    If the `aspect` member of any element of [`p_ranges`] includes `VK_IMAGE_ASPECT_STENCIL_BIT`, and [`image`] was not created with [`ImageStencilUsageCreateInfo`], `VK_IMAGE_USAGE_TRANSFER_DST_BIT` **must**  have been included in the [`ImageCreateInfo::usage`] used to create [`image`]
-    If the `aspect` member of any element of [`p_ranges`] includes `VK_IMAGE_ASPECT_DEPTH_BIT`, `VK_IMAGE_USAGE_TRANSFER_DST_BIT` **must**  have been included in the [`ImageCreateInfo::usage`] used to create [`image`]
-    If [`image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`image_layout`] **must**  specify the layout of the image subresource ranges of [`image`] specified in [`p_ranges`] at the time this command is executed on a [`Device`]
-  [`image_layout`] **must**  be either of `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
-    The [`ImageSubresourceRange::aspect_mask`] member of each element of the [`p_ranges`] array  **must**  not include bits other than `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT`
-    If the [`image`]’s format does not have a stencil component, then the [`ImageSubresourceRange::aspect_mask`] member of each element of the [`p_ranges`] array  **must**  not include the `VK_IMAGE_ASPECT_STENCIL_BIT` bit
-    If the [`image`]’s format does not have a depth component, then the [`ImageSubresourceRange::aspect_mask`] member of each element of the [`p_ranges`] array  **must**  not include the `VK_IMAGE_ASPECT_DEPTH_BIT` bit
-    The [`ImageSubresourceRange::base_mip_level`] members of the elements of the [`p_ranges`] array  **must**  each be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    For each [`ImageSubresourceRange`] element of [`p_ranges`], if the `levelCount` member is not `VK_REMAINING_MIP_LEVELS`, then `baseMipLevel` +  `levelCount` **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    The [`ImageSubresourceRange::base_array_layer`] members of the elements of the [`p_ranges`] array  **must**  each be less than the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    For each [`ImageSubresourceRange`] element of [`p_ranges`], if the `layerCount` member is not `VK_REMAINING_ARRAY_LAYERS`, then `baseArrayLayer` +  `layerCount` **must**  be less than the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-  [`image`] **must**  have a depth/stencil format
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, [`image`] **must**  not be a protected image
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, [`image`] **must**  not be an unprotected image

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`image`] **must**  be a valid [`Image`] handle
-  [`image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`p_depth_stencil`] **must**  be a valid pointer to a valid [`ClearDepthStencilValue`] structure
-  [`p_ranges`] **must**  be a valid pointer to an array of [`range_count`] valid [`ImageSubresourceRange`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called outside of a render pass instance
-  [`range_count`] **must**  be greater than `0`
-    Both of [`command_buffer`], and [`image`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`ClearDepthStencilValue`]
- [`CommandBuffer`]
- [`Image`]
- [`ImageLayout`]
- [`ImageSubresourceRange`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        