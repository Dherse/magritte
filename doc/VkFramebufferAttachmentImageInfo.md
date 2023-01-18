[VkFramebufferAttachmentImageInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfo.html) - Structure specifying parameters of an image that will be used with a framebuffer

# C Specifications
The [`FramebufferAttachmentImageInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkFramebufferAttachmentImageInfo {
    VkStructureType       sType;
    const void*           pNext;
    VkImageCreateFlags    flags;
    VkImageUsageFlags     usage;
    uint32_t              width;
    uint32_t              height;
    uint32_t              layerCount;
    uint32_t              viewFormatCount;
    const VkFormat*       pViewFormats;
} VkFramebufferAttachmentImageInfo;
```
or the equivalent
```c
// Provided by VK_KHR_imageless_framebuffer
typedef VkFramebufferAttachmentImageInfo VkFramebufferAttachmentImageInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`ImageCreateFlagBits`], matching the value of [`ImageCreateInfo`]::[`flags`] used to create an image that will be used with this framebuffer.
- [`usage`] is a bitmask of [`ImageUsageFlagBits`], matching the value of [`ImageCreateInfo`]::[`usage`] used to create an image used with this framebuffer.
- [`width`] is the width of the image view used for rendering.
- [`height`] is the height of the image view used for rendering.
- [`layer_count`] is the number of array layers of the image view used for rendering.
- [`view_format_count`] is the number of entries in the [`view_formats`] array, matching the value of [`ImageFormatListCreateInfo`]::[`view_format_count`] used to create an image used with this framebuffer.
- [`view_formats`] is a pointer to an array of [`Format`] values specifying all of the formats which  **can**  be used when creating views of the image, matching the value of [`ImageFormatListCreateInfo`]::[`view_formats`] used to create an image used with this framebuffer.

# Description
Images that  **can**  be used with the framebuffer when beginning a render pass,
as specified by [`RenderPassAttachmentBeginInfo`],  **must**  be created with
parameters that are identical to those specified here.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`ImageCreateFlagBits`] values
-  [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
-  [`usage`] **must**  not be `0`
-    If [`view_format_count`] is not `0`, [`view_formats`] **must**  be a valid pointer to an array of [`view_format_count`] valid [`Format`] values

# Related
- [`VK_KHR_imageless_framebuffer`]
- [`crate::vulkan1_2`]
- [`Format`]
- [`FramebufferAttachmentsCreateInfo`]
- [`ImageCreateFlags`]
- [`ImageUsageFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        