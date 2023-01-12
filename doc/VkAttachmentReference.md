[VkAttachmentReference](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference.html) - Structure specifying an attachment reference

# C Specifications
The [`AttachmentReference`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkAttachmentReference {
    uint32_t         attachment;
    VkImageLayout    layout;
} VkAttachmentReference;
```

# Members
- [`attachment`] is either an integer value identifying an attachment at the corresponding index in [`RenderPassCreateInfo::attachments`], or `VK_ATTACHMENT_UNUSED` to signify that this attachment is not used.
- [`layout`] is a [`ImageLayout`] value specifying the layout the attachment uses during the subpass.

# Description
## Valid Usage
-    If [`attachment`] is not `VK_ATTACHMENT_UNUSED`, [`layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED`, `VK_IMAGE_LAYOUT_PREINITIALIZED`, `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`, `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`

## Valid Usage (Implicit)
-  [`layout`] **must**  be a valid [`ImageLayout`] value

# Related
- [`crate::vulkan1_0`]
- [`ImageLayout`]
- [`RenderPassFragmentDensityMapCreateInfoEXT`]
- [`SubpassDescription`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        