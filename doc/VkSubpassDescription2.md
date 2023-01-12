[VkSubpassDescription2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription2.html) - Structure specifying a subpass description

# C Specifications
The [`SubpassDescription2`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSubpassDescription2 {
    VkStructureType                  sType;
    const void*                      pNext;
    VkSubpassDescriptionFlags        flags;
    VkPipelineBindPoint              pipelineBindPoint;
    uint32_t                         viewMask;
    uint32_t                         inputAttachmentCount;
    const VkAttachmentReference2*    pInputAttachments;
    uint32_t                         colorAttachmentCount;
    const VkAttachmentReference2*    pColorAttachments;
    const VkAttachmentReference2*    pResolveAttachments;
    const VkAttachmentReference2*    pDepthStencilAttachment;
    uint32_t                         preserveAttachmentCount;
    const uint32_t*                  pPreserveAttachments;
} VkSubpassDescription2;
```
or the equivalent
```c
// Provided by VK_KHR_create_renderpass2
typedef VkSubpassDescription2 VkSubpassDescription2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`SubpassDescriptionFlagBits`] specifying usage of the subpass.
- [`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying the pipeline type supported for this subpass.
- [`view_mask`] is a bitfield of view indices describing which views rendering is broadcast to in this subpass, when multiview is enabled.
- [`input_attachment_count`] is the number of input attachments.
- [`input_attachments`] is a pointer to an array of [`AttachmentReference2`] structures defining the input attachments for this subpass and their layouts.
- [`color_attachment_count`] is the number of color attachments.
- [`color_attachments`] is a pointer to an array of [`color_attachment_count`][`AttachmentReference2`] structures defining the color attachments for this subpass and their layouts.
- [`resolve_attachments`] is `NULL` or a pointer to an array of [`color_attachment_count`][`AttachmentReference2`] structures defining the resolve attachments for this subpass and their layouts.
- [`depth_stencil_attachment`] is a pointer to a [`AttachmentReference2`] structure specifying the depth/stencil attachment for this subpass and its layout.
- [`preserve_attachment_count`] is the number of preserved attachments.
- [`preserve_attachments`] is a pointer to an array of [`preserve_attachment_count`] render pass attachment indices identifying attachments that are not used by this subpass, but whose contents  **must**  be preserved throughout the subpass.

# Description
Parameters defined by this structure with the same name as those in
[`SubpassDescription`] have the identical effect to those parameters.[`view_mask`] has the same effect for the described subpass as
[`RenderPassMultiviewCreateInfo::view_masks`] has on each
corresponding subpass.If a [`FragmentShadingRateAttachmentInfoKHR`] structure is included in
the [`p_next`] chain, `pFragmentShadingRateAttachment` is not `NULL`,
and its `attachment` member is not `VK_ATTACHMENT_UNUSED`, the
identified attachment defines a fragment shading rate attachment for that
subpass.
## Valid Usage
-  [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS` or `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`
-  [`color_attachment_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_color_attachments`]
-    If the first use of an attachment in this render pass is as an input attachment, and the attachment is not also used as a color or depth/stencil attachment in the same subpass, then `loadOp` **must**  not be `VK_ATTACHMENT_LOAD_OP_CLEAR`
-    If [`resolve_attachments`] is not `NULL`, for each resolve attachment that does not have the value `VK_ATTACHMENT_UNUSED`, the corresponding color attachment  **must**  not have the value `VK_ATTACHMENT_UNUSED`
-    If [`resolve_attachments`] is not `NULL`, for each resolve attachment that is not `VK_ATTACHMENT_UNUSED`, the corresponding color attachment  **must**  not have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    If [`resolve_attachments`] is not `NULL`, each resolve attachment that is not `VK_ATTACHMENT_UNUSED` **must**  have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    Any given element of [`resolve_attachments`] **must**  have the same [`Format`] as its corresponding color attachment
-    All attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have the same sample count
-    All attachments in [`input_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain at least `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    All attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    All attachments in [`resolve_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    If [`depth_stencil_attachment`] is not `NULL` and the attachment is not `VK_ATTACHMENT_UNUSED` then it  **must**  have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`input_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`resolve_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If the `[`amd_mixed_attachment_samples`]` extension is enabled, all attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have a sample count that is smaller than or equal to the sample count of [`depth_stencil_attachment`] if it is not `VK_ATTACHMENT_UNUSED`
-    If neither the `[`amd_mixed_attachment_samples`]` nor the `[`nv_framebuffer_mixed_samples`]` extensions are enabled, and if [`depth_stencil_attachment`] is not `VK_ATTACHMENT_UNUSED` and any attachments in [`color_attachments`] are not `VK_ATTACHMENT_UNUSED`, they  **must**  have the same sample count
-    Each element of [`preserve_attachments`] **must**  not be `VK_ATTACHMENT_UNUSED`
-    Any given element of [`preserve_attachments`] **must**  not also be an element of any other member of the subpass description
-    If any attachment is used by more than one [`AttachmentReference2`] member, then each use  **must**  use the same `layout`
-    Attachments  **must**  follow the [image layout requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#attachment-type-imagelayout) based on the type of attachment it is being used as
-    If [`flags`] includes `VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX`, it  **must**  also include `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX`
-    If the `attachment` member of any element of [`input_attachments`] is not `VK_ATTACHMENT_UNUSED`, then the `aspectMask` member  **must**  be a valid combination of [`ImageAspectFlagBits`]
-    If the `attachment` member of any element of [`input_attachments`] is not `VK_ATTACHMENT_UNUSED`, then the `aspectMask` member  **must**  not be `0`
-    If the `attachment` member of any element of [`input_attachments`] is not `VK_ATTACHMENT_UNUSED`, then the `aspectMask` member  **must**  not include `VK_IMAGE_ASPECT_METADATA_BIT`
-    If the `attachment` member of any element of [`input_attachments`] is not `VK_ATTACHMENT_UNUSED`, then the `aspectMask` member  **must**  not include `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index *i*
-    An attachment  **must**  not be used in both [`depth_stencil_attachment`] and [`color_attachments`]
-    If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature is not enabled, [`view_mask`] **must**  be `0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`FragmentShadingRateAttachmentInfoKHR`] or [`SubpassDescriptionDepthStencilResolve`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`SubpassDescriptionFlagBits`] values
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-    If [`input_attachment_count`] is not `0`, [`input_attachments`] **must**  be a valid pointer to an array of [`input_attachment_count`] valid [`AttachmentReference2`] structures
-    If [`color_attachment_count`] is not `0`, [`color_attachments`] **must**  be a valid pointer to an array of [`color_attachment_count`] valid [`AttachmentReference2`] structures
-    If [`color_attachment_count`] is not `0`, and [`resolve_attachments`] is not `NULL`, [`resolve_attachments`] **must**  be a valid pointer to an array of [`color_attachment_count`] valid [`AttachmentReference2`] structures
-    If [`depth_stencil_attachment`] is not `NULL`, [`depth_stencil_attachment`] **must**  be a valid pointer to a valid [`AttachmentReference2`] structure
-    If [`preserve_attachment_count`] is not `0`, [`preserve_attachments`] **must**  be a valid pointer to an array of [`preserve_attachment_count`]`uint32_t` values

# Related
- [`khr_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`AttachmentReference2`]
- [`PipelineBindPoint`]
- [`RenderPassCreateInfo2`]
- [`StructureType`]
- [VkSubpassDescriptionFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        