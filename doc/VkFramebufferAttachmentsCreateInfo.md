[VkFramebufferAttachmentsCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html) - Structure specifying parameters of images that will be used with a framebuffer

# C Specifications
The [`FramebufferAttachmentsCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkFramebufferAttachmentsCreateInfo {
    VkStructureType                            sType;
    const void*                                pNext;
    uint32_t                                   attachmentImageInfoCount;
    const VkFramebufferAttachmentImageInfo*    pAttachmentImageInfos;
} VkFramebufferAttachmentsCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_imageless_framebuffer
typedef VkFramebufferAttachmentsCreateInfo VkFramebufferAttachmentsCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attachment_image_info_count`] is the number of attachments being described.
- [`attachment_image_infos`] is a pointer to an array of [`FramebufferAttachmentImageInfo`] structures, each structure describing a number of parameters of the corresponding attachment in a render pass instance.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`
-    If [`attachment_image_info_count`] is not `0`, [`attachment_image_infos`] **must**  be a valid pointer to an array of [`attachment_image_info_count`] valid [`FramebufferAttachmentImageInfo`] structures

# Related
- [`crate::vulkan1_2`]
- [`FramebufferAttachmentImageInfo`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        