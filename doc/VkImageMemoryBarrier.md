[VkImageMemoryBarrier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier.html) - Structure specifying the parameters of an image memory barrier

# C Specifications
The [`ImageMemoryBarrier`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageMemoryBarrier {
    VkStructureType            sType;
    const void*                pNext;
    VkAccessFlags              srcAccessMask;
    VkAccessFlags              dstAccessMask;
    VkImageLayout              oldLayout;
    VkImageLayout              newLayout;
    uint32_t                   srcQueueFamilyIndex;
    uint32_t                   dstQueueFamilyIndex;
    VkImage                    image;
    VkImageSubresourceRange    subresourceRange;
} VkImageMemoryBarrier;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
- [`old_layout`] is the old layout in an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
- [`new_layout`] is the new layout in an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
- [`src_queue_family_index`] is the source queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
- [`dst_queue_family_index`] is the destination queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
- [`image`] is a handle to the image affected by this barrier.
- [`subresource_range`] describes the [image subresource range](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views) within [`image`] that is affected by this barrier.

# Description
The first [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to access to memory through the specified image subresource range,
via access types in the [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks)
specified by [`src_access_mask`].
If [`src_access_mask`] includes `VK_ACCESS_HOST_WRITE_BIT`, memory
writes performed by that access type are also made visible, as that access
type is not performed through a resource.The second [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) is
limited to access to memory through the specified image subresource range,
via access types in the [destination access
mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks) specified by [`dst_access_mask`].
If [`dst_access_mask`] includes `VK_ACCESS_HOST_WRITE_BIT` or
`VK_ACCESS_HOST_READ_BIT`, available memory writes are also made visible
to accesses of those types, as those access types are not performed through
a resource.If [`src_queue_family_index`] is not equal to [`dst_queue_family_index`], and
[`src_queue_family_index`] is equal to the current queue family, then the
memory barrier defines a [queue
family release operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release) for the specified image subresource range, and
the second access scope includes no access, as if [`dst_access_mask`] was
`0`.If [`dst_queue_family_index`] is not equal to [`src_queue_family_index`], and
[`dst_queue_family_index`] is equal to the current queue family, then the
memory barrier defines a [queue
family acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire) for the specified image subresource range, and
the first access scope includes no access, as if [`src_access_mask`] was
`0`.If the [`synchronization2` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) is not
enabled or [`old_layout`] is not equal to [`new_layout`],
[`old_layout`] and [`new_layout`] define an
[image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions) for
the specified image subresource range.If [`image`] has a multi-planar format and the image is *disjoint*, then
including `VK_IMAGE_ASPECT_COLOR_BIT` in the `aspectMask` member of
[`subresource_range`] is equivalent to including
`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, and
(for three-plane formats only) `VK_IMAGE_ASPECT_PLANE_2_BIT`.
## Valid Usage
-  `subresourceRange.baseMipLevel` **must**  be less than the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-    If `subresourceRange.levelCount` is not `VK_REMAINING_MIP_LEVELS`, `subresourceRange.baseMipLevel` +  `subresourceRange.levelCount` **must**  be less than or equal to the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
-  `subresourceRange.baseArrayLayer` **must**  be less than the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    If `subresourceRange.layerCount` is not `VK_REMAINING_ARRAY_LAYERS`, `subresourceRange.baseArrayLayer` +  `subresourceRange.layerCount` **must**  be less than or equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
-    If [`image`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_SAMPLED_BIT` or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), [`old_layout`] **must**  be `VK_IMAGE_LAYOUT_UNDEFINED` or the current layout of the image subresources affected by the barrier
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), [`new_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED` or `VK_IMAGE_LAYOUT_PREINITIALIZED`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with at least one of `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` set
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` then [`image`] **must**  have been created with at least one of `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` set
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL`, [`image`] **must**  have been created with `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` or `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL`, [`image`] **must**  have been created with at least one of `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions), and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR` then [`image`] **must**  have been created with `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` set
-    If [`image`] has a single-plane color format or is not *disjoint*, then the `aspectMask` member of [`subresource_range`] **must**  be `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`image`] has a multi-planar format and the image is *disjoint*, then the `aspectMask` member of [`subresource_range`] **must**  include either at least one of `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, and `VK_IMAGE_ASPECT_PLANE_2_BIT`; or  **must**  include `VK_IMAGE_ASPECT_COLOR_BIT`
-    If [`image`] has a multi-planar format with only two planes, then the `aspectMask` member of [`subresource_range`] **must**  not include `VK_IMAGE_ASPECT_PLANE_2_BIT`
-    If [`image`] has a depth/stencil format with both depth and stencil and the [separateDepthStencilLayouts](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled, then the `aspectMask` member of [`subresource_range`] **must**  include either or both `VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT`
-    If [`image`] has a depth/stencil format with both depth and stencil and the [separateDepthStencilLayouts](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is not enabled, then the `aspectMask` member of [`subresource_range`] **must**  include both `VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT`
-    If [`src_queue_family_index`] is not equal to [`dst_queue_family_index`], at least one  **must**  not be a special queue family reserved for external memory ownership transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
-    If [`image`] was created with a sharing mode of `VK_SHARING_MODE_CONCURRENT`, [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, and one of [`src_queue_family_index`] and [`dst_queue_family_index`] is one of the special queue family values reserved for external memory transfers, the other  **must**  be `VK_QUEUE_FAMILY_IGNORED`
-    If [`image`] was created with a sharing mode of `VK_SHARING_MODE_EXCLUSIVE`, and [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, [`src_queue_family_index`] and [`dst_queue_family_index`] **must**  both be valid queue families, or one of the special queue family values reserved for external memory transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
-    If the [`synchronization2` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) is not enabled, and [`image`] was created with a sharing mode of `VK_SHARING_MODE_CONCURRENT`, at least one of [`src_queue_family_index`] and [`dst_queue_family_index`] **must**  be `VK_QUEUE_FAMILY_IGNORED`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`SampleLocationsInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`old_layout`] **must**  be a valid [`ImageLayout`] value
-  [`new_layout`] **must**  be a valid [`ImageLayout`] value
-  [`image`] **must**  be a valid [`Image`] handle
-  [`subresource_range`] **must**  be a valid [`ImageSubresourceRange`] structure

# Related
- [`crate::vulkan1_0`]
- [VkAccessFlags]()
- [`Image`]
- [`ImageLayout`]
- [`ImageSubresourceRange`]
- [`StructureType`]
- [`cmd_pipeline_barrier`]
- [`cmd_wait_events`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        