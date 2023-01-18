[VkAttachmentReference2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference2.html) - Structure specifying an attachment reference

# C Specifications
The [`AttachmentReference2`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkAttachmentReference2 {
    VkStructureType       sType;
    const void*           pNext;
    uint32_t              attachment;
    VkImageLayout         layout;
    VkImageAspectFlags    aspectMask;
} VkAttachmentReference2;
```
or the equivalent
```c
// Provided by VK_KHR_create_renderpass2
typedef VkAttachmentReference2 VkAttachmentReference2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`attachment`] is either an integer value identifying an attachment at the corresponding index in [`RenderPassCreateInfo2::attachments`], or [`ATTACHMENT_UNUSED`] to signify that this attachment is not used.
- [`layout`] is a [`ImageLayout`] value specifying the layout the attachment uses during the subpass.
- [`aspect_mask`] is a mask of which aspect(s)  **can**  be accessed within the specified subpass as an input attachment.

# Description
Parameters defined by this structure with the same name as those in
[`AttachmentReference`] have the identical effect to those parameters.[`aspect_mask`] is ignored when this structure is used to describe anything
other than an input attachment reference.If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled, and [`attachment`]
has a depth/stencil format, [`layout`] **can**  be set to a layout that only
specifies the layout of the depth aspect.If [`layout`] only specifies the layout of the depth aspect of the
attachment, the layout of the stencil aspect is specified by the
`stencilLayout` member of a [`AttachmentReferenceStencilLayout`]
structure included in the [`p_next`] chain.
Otherwise, [`layout`] describes the layout for all relevant image aspects.
## Valid Usage
-    If [`attachment`] is not [`ATTACHMENT_UNUSED`], [`layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED`, `VK_IMAGE_LAYOUT_PREINITIALIZED`, or `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
-    If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is not enabled, and [`attachment`] is not [`ATTACHMENT_UNUSED`], [`layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
-    If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is a color format, [`layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is a depth/stencil format which includes both depth and stencil aspects, and [`layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the [`p_next`] chain  **must**  include a [`AttachmentReferenceStencilLayout`] structure
-    If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is a depth/stencil format which includes only the depth aspect, [`layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is a depth/stencil format which includes only the stencil aspect, [`layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`AttachmentReferenceStencilLayout`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`layout`] **must**  be a valid [`ImageLayout`] value

# Related
- [`VK_KHR_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`FragmentShadingRateAttachmentInfoKHR`]
- [`ImageAspectFlags`]
- [`ImageLayout`]
- [`StructureType`]
- [`SubpassDescription2`]
- [`SubpassDescriptionDepthStencilResolve`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        