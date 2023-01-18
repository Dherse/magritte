[VkRenderPassAttachmentBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfo.html) - Structure specifying images to be used as framebuffer attachments

# C Specifications
The [`RenderPassAttachmentBeginInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkRenderPassAttachmentBeginInfo {
    VkStructureType       sType;
    const void*           pNext;
    uint32_t              attachmentCount;
    const VkImageView*    pAttachments;
} VkRenderPassAttachmentBeginInfo;
```
or the equivalent
```c
// Provided by VK_KHR_imageless_framebuffer
typedef VkRenderPassAttachmentBeginInfo VkRenderPassAttachmentBeginInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attachment_count`] is the number of attachments.
- [`attachments`] is a pointer to an array of [`ImageView`] handles, each of which will be used as the corresponding attachment in the render pass instance.

# Description
## Valid Usage
-    Each element of [`attachments`] **must**  only specify a single mip level
-    Each element of [`attachments`] **must**  have been created with the identity swizzle
-    Each element of [`attachments`] **must**  have been created with [`ImageViewCreateInfo::view_type`] not equal to `VK_IMAGE_VIEW_TYPE_3D`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`
-    If [`attachment_count`] is not `0`, [`attachments`] **must**  be a valid pointer to an array of [`attachment_count`] valid [`ImageView`] handles

# Related
- [`VK_KHR_imageless_framebuffer`]
- [`crate::vulkan1_2`]
- [`ImageView`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        