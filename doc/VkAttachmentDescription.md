[VkAttachmentDescription](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription.html) - Structure specifying an attachment description

# C Specifications
The [`AttachmentDescription`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkAttachmentDescription {
    VkAttachmentDescriptionFlags    flags;
    VkFormat                        format;
    VkSampleCountFlagBits           samples;
    VkAttachmentLoadOp              loadOp;
    VkAttachmentStoreOp             storeOp;
    VkAttachmentLoadOp              stencilLoadOp;
    VkAttachmentStoreOp             stencilStoreOp;
    VkImageLayout                   initialLayout;
    VkImageLayout                   finalLayout;
} VkAttachmentDescription;
```

# Members
- [`flags`] is a bitmask of [`AttachmentDescriptionFlagBits`] specifying additional properties of the attachment.
- [`format`] is a [`Format`] value specifying the format of the image view that will be used for the attachment.
- [`samples`] is a [`SampleCountFlagBits`] value specifying the number of samples of the image.
- [`load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of color and depth components of the attachment are treated at the beginning of the subpass where it is first used.
- [`store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of color and depth components of the attachment are treated at the end of the subpass where it is last used.
- [`stencil_load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of stencil components of the attachment are treated at the beginning of the subpass where it is first used.
- [`stencil_store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of stencil components of the attachment are treated at the end of the last subpass where it is used.
- [`initial_layout`] is the layout the attachment image subresource will be in when a render pass instance begins.
- [`final_layout`] is the layout the attachment image subresource will be transitioned to when a render pass instance ends.

# Description
If the attachment uses a color format, then [`load_op`] and [`store_op`]
are used, and [`stencil_load_op`] and [`stencil_store_op`] are ignored.
If the format has depth and/or stencil components, [`load_op`] and
[`store_op`] apply only to the depth data, while [`stencil_load_op`] and
[`stencil_store_op`] define how the stencil data is handled.
[`load_op`] and [`stencil_load_op`] define the *load operations* that
execute as part of the first subpass that uses the attachment.
[`store_op`] and [`stencil_store_op`] define the *store operations* that
execute as part of the last subpass that uses the attachment.The load operation for each sample in an attachment happens-before any
recorded command which accesses the sample in the first subpass where the
attachment is used.
Load operations for attachments with a depth/stencil format execute in the
`VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT` pipeline stage.
Load operations for attachments with a color format execute in the
`VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.The store operation for each sample in an attachment happens-after any
recorded command which accesses the sample in the last subpass where the
attachment is used.
Store operations for attachments with a depth/stencil format execute in the
`VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT` pipeline stage.
Store operations for attachments with a color format execute in the
`VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.If an attachment is not used by any subpass, [`load_op`], [`store_op`],
[`stencil_store_op`], and [`stencil_load_op`] will be ignored for that
attachment, and no load or store ops will be performed.
However, any transition specified by [`initial_layout`] and
[`final_layout`] will still be executed.The load and store operations apply on the first and last use of each view
in the render pass, respectively.
If a view index of an attachment is not included in the view mask in any
subpass that uses it, then the load and store operations are ignored, and
the attachment’s memory contents will not be modified by execution of a
render pass instance.During a render pass instance, input/color attachments with color formats
that have a component size of 8, 16, or 32 bits  **must**  be represented in the
attachment’s format throughout the instance.
Attachments with other floating- or fixed-point color formats, or with depth
components  **may**  be represented in a format with a precision higher than the
attachment format, but  **must**  be represented with the same range.
When such a component is loaded via the [`load_op`], it will be converted
into an implementation-dependent format used by the render pass.
Such components  **must**  be converted from the render pass format, to the
format of the attachment, before they are resolved or stored at the end of a
render pass instance via [`store_op`].
Conversions occur as described in [Numeric
Representation and Computation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-numerics) and [Fixed-Point
Data Conversions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-fixedconv).If [`flags`] includes `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT`, then
the attachment is treated as if it shares physical memory with another
attachment in the same render pass.
This information limits the ability of the implementation to reorder certain
operations (like layout transitions and the [`load_op`]) such that it is
not improperly reordered against other uses of the same physical memory via
a different attachment.
This is described in more detail below.If a render pass uses multiple attachments that alias the same device
memory, those attachments  **must**  each include the
`VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT` bit in their attachment
description flags.
Attachments aliasing the same memory occurs in multiple ways:
- Multiple attachments being assigned the same image view as part of framebuffer creation.
- Attachments using distinct image views that correspond to the same image subresource of an image.
- Attachments using views of distinct image subresources which are bound to overlapping memory ranges.
Multiple attachments that alias the same memory  **must**  not be used in a
single subpass.
A given attachment index  **must**  not be used multiple times in a single
subpass, with one exception: two subpass attachments  **can**  use the same
attachment index if at least one use is as an input attachment and neither
use is as a resolve or preserve attachment.
In other words, the same view  **can**  be used simultaneously as an input and
color or depth/stencil attachment, but  **must**  not be used as multiple color
or depth/stencil attachments nor as resolve or preserve attachments.
The precise set of valid scenarios is described in more detail
[below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop).If a set of attachments alias each other, then all except the first to be
used in the render pass  **must**  use an [`initial_layout`] of
`VK_IMAGE_LAYOUT_UNDEFINED`, since the earlier uses of the other aliases
make their contents undefined.
Once an alias has been used and a different alias has been used after it,
the first alias  **must**  not be used in any later subpasses.
However, an application  **can**  assign the same image view to multiple aliasing
attachment indices, which allows that image view to be used multiple times
even if other aliases are used in between.
## Valid Usage
-  [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED` or `VK_IMAGE_LAYOUT_PREINITIALIZED`
-    If [`format`] is a color format, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a color format, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
-    If [`format`] is a depth/stencil format, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
-    If [`format`] is a color format, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a color format, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
-    If [`format`] is a depth/stencil format, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
-    If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is not enabled, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is not enabled, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a color format, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a color format, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes both depth and stencil aspects, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes both depth and stencil aspects, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes only the depth aspect, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes only the depth aspect, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes only the stencil aspect, [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
-    If [`format`] is a depth/stencil format which includes only the stencil aspect, [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`

## Valid Usage (Implicit)
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
- [`crate::vulkan1_0`]
- [VkAttachmentDescriptionFlags]()
- [`AttachmentLoadOp`]
- [`AttachmentStoreOp`]
- [`Format`]
- [`ImageLayout`]
- [`RenderPassCreateInfo`]
- [`SampleCountFlagBits`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        