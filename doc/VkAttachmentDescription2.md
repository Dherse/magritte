[VkAttachmentDescription2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription2.html) - Structure specifying an attachment description

# C Specifications
The [`AttachmentDescription2`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkAttachmentDescription2 {
    VkStructureType                 sType;
    const void*                     pNext;
    VkAttachmentDescriptionFlags    flags;
    VkFormat                        format;
    VkSampleCountFlagBits           samples;
    VkAttachmentLoadOp              loadOp;
    VkAttachmentStoreOp             storeOp;
    VkAttachmentLoadOp              stencilLoadOp;
    VkAttachmentStoreOp             stencilStoreOp;
    VkImageLayout                   initialLayout;
    VkImageLayout                   finalLayout;
} VkAttachmentDescription2;
```
or the equivalent
```c
// Provided by VK_KHR_create_renderpass2
typedef VkAttachmentDescription2 VkAttachmentDescription2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`AttachmentDescriptionFlagBits`] specifying additional properties of the attachment.
- [`format`] is a [`Format`] value specifying the format of the image that will be used for the attachment.
- [`samples`] is a [`SampleCountFlagBits`] value specifying the number of samples of the image.
- [`load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of color and depth components of the attachment are treated at the beginning of the subpass where it is first used.
- [`store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of color and depth components of the attachment are treated at the end of the subpass where it is last used.
- [`stencil_load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of stencil components of the attachment are treated at the beginning of the subpass where it is first used.
- [`stencil_store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of stencil components of the attachment are treated at the end of the last subpass where it is used.
- [`initial_layout`] is the layout the attachment image subresource will be in when a render pass instance begins.
- [`final_layout`] is the layout the attachment image subresource will be transitioned to when a render pass instance ends.

# Description
Parameters defined by this structure with the same name as those in
[`AttachmentDescription`] have the identical effect to those parameters.If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled, and [`format`] is
a depth/stencil format, [`initial_layout`] and [`final_layout`] **can**  be
set to a layout that only specifies the layout of the depth aspect.If the [`p_next`] chain includes a
[`AttachmentDescriptionStencilLayout`] structure, then the
`stencilInitialLayout` and `stencilFinalLayout` members specify the
initial and final layouts of the stencil aspect of a depth/stencil format,
and [`initial_layout`] and [`final_layout`] only apply to the depth
aspect.
For depth-only formats, the [`AttachmentDescriptionStencilLayout`]
structure is ignored.
For stencil-only formats, the initial and final layouts of the stencil
aspect are taken from the [`AttachmentDescriptionStencilLayout`]
structure if present, or [`initial_layout`] and [`final_layout`] if not
present.If [`format`] is a depth/stencil format, and either [`initial_layout`] or
[`final_layout`] does not specify a layout for the stencil aspect, then the
application  **must**  specify the initial and final layouts of the stencil
aspect by including a [`AttachmentDescriptionStencilLayout`] structure
in the [`p_next`] chain.
## Valid Usage
-  [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED` or `VK_IMAGE_LAYOUT_PREINITIALIZED`
-    If [`format`] is a color format, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
-    If [`format`] is a depth/stencil format, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
-    If [`format`] is a color format, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
-    If [`format`] is a depth/stencil format, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
-    If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is not enabled, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is not enabled, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a color format, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a color format, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes both depth and stencil aspects, and [`initial_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the [`p_next`] chain  **must**  include a [`AttachmentDescriptionStencilLayout`] structure
-    If [`format`] is a depth/stencil format which includes both depth and stencil aspects, and [`final_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the [`p_next`] chain  **must**  include a [`AttachmentDescriptionStencilLayout`] structure
-    If [`format`] is a depth/stencil format which includes only the depth aspect, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes only the depth aspect, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes only the stencil aspect, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes only the stencil aspect, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
-    If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled and [`format`] is a depth/stencil format that includes a depth aspect and the [`p_next`] chain includes a [`AttachmentDescriptionStencilLayout`] structure, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled and [`format`] is a depth/stencil format that includes a depth aspect and the [`p_next`] chain includes a [`AttachmentDescriptionStencilLayout`] structure, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`AttachmentDescriptionStencilLayout`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`AttachmentDescriptionFlagBits`] values
-  [`format`] **must**  be a valid [`Format`] value
-  [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
-  [`load_op`] **must**  be a valid [`AttachmentLoadOp`] value
-  [`store_op`] **must**  be a valid [`AttachmentStoreOp`] value
-  [`stencil_load_op`] **must**  be a valid [`AttachmentLoadOp`] value
-  [`stencil_store_op`] **must**  be a valid [`AttachmentStoreOp`] value
-  [`initial_layout`] **must**  be a valid [`ImageLayout`] value
-  [`final_layout`] **must**  be a valid [`ImageLayout`] value

# Related
- [`VK_KHR_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`AttachmentDescriptionFlags`]
- [`AttachmentLoadOp`]
- [`AttachmentStoreOp`]
- [`Format`]
- [`ImageLayout`]
- [`RenderPassCreateInfo2`]
- [`SampleCountFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        