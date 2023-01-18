[VkRenderPassCreateInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo2.html) - Structure specifying parameters of a newly created render pass

# C Specifications
The [`RenderPassCreateInfo2`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkRenderPassCreateInfo2 {
    VkStructureType                    sType;
    const void*                        pNext;
    VkRenderPassCreateFlags            flags;
    uint32_t                           attachmentCount;
    const VkAttachmentDescription2*    pAttachments;
    uint32_t                           subpassCount;
    const VkSubpassDescription2*       pSubpasses;
    uint32_t                           dependencyCount;
    const VkSubpassDependency2*        pDependencies;
    uint32_t                           correlatedViewMaskCount;
    const uint32_t*                    pCorrelatedViewMasks;
} VkRenderPassCreateInfo2;
```
or the equivalent
```c
// Provided by VK_KHR_create_renderpass2
typedef VkRenderPassCreateInfo2 VkRenderPassCreateInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`attachment_count`] is the number of attachments used by this render pass.
- [`attachments`] is a pointer to an array of [`attachment_count`][`AttachmentDescription2`] structures describing the attachments used by the render pass.
- [`subpass_count`] is the number of subpasses to create.
- [`subpasses`] is a pointer to an array of [`subpass_count`][`SubpassDescription2`] structures describing each subpass.
- [`dependency_count`] is the number of dependencies between pairs of subpasses.
- [`dependencies`] is a pointer to an array of [`dependency_count`][`SubpassDependency2`] structures describing dependencies between pairs of subpasses.
- [`correlated_view_mask_count`] is the number of correlation masks.
- [`correlated_view_masks`] is a pointer to an array of view masks indicating sets of views that  **may**  be more efficient to render concurrently.

# Description
Parameters defined by this structure with the same name as those in
[`RenderPassCreateInfo`] have the identical effect to those parameters;
the child structures are variants of those used in
[`RenderPassCreateInfo`] which add [`s_type`] and [`p_next`]
parameters, allowing them to be extended.If the [`SubpassDescription2::view_mask`] member of any element of
[`subpasses`] is not zero, *multiview* functionality is considered to be
enabled for this render pass.[`correlated_view_mask_count`] and [`correlated_view_masks`] have the same
effect as [`RenderPassMultiviewCreateInfo::correlation_mask_count`]
and [`RenderPassMultiviewCreateInfo::correlation_masks`],
respectively.
## Valid Usage
-    If any two subpasses operate on attachments with overlapping ranges of the same [`DeviceMemory`] object, and at least one subpass writes to that area of [`DeviceMemory`], a subpass dependency  **must**  be included (either directly or via some intermediate subpasses) between them
-    If the `attachment` member of any element of `pInputAttachments`, `pColorAttachments`, `pResolveAttachments` or `pDepthStencilAttachment`, or the attachment indexed by any element of `pPreserveAttachments` in any given element of [`subpasses`] is bound to a range of a [`DeviceMemory`] object that overlaps with any other attachment in any subpass (including the same subpass), the [`AttachmentDescription2`] structures describing them  **must**  include `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT` in [`flags`]
-    If the `attachment` member of any element of `pInputAttachments`, `pColorAttachments`, `pResolveAttachments` or `pDepthStencilAttachment`, or any element of `pPreserveAttachments` in any given element of [`subpasses`] is not [`ATTACHMENT_UNUSED`], then it  **must**  be less than [`attachment_count`]
-    If the pNext chain includes a [`RenderPassFragmentDensityMapCreateInfoEXT`] structure and the `fragmentDensityMapAttachment` member is not [`ATTACHMENT_UNUSED`], then `attachment` **must**  be less than [`attachment_count`]
-    If the [`subpasses`] pNext chain includes a [`SubpassDescriptionDepthStencilResolve`] structure and the `pDepthStencilResolveAttachment` member is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], then `attachment` **must**  be less than [`attachment_count`]
-    For any member of [`attachments`] with a `loadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment  **must**  not specify a `layout` equal to `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
-    For any member of [`attachments`] with a `stencilLoadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment  **must**  not specify a `layout` equal to `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, or `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`
-    For any element of [`dependencies`], if the `srcSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `srcStageMask` member of that dependency  **must**  be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the source subpass
-    For any element of [`dependencies`], if the `dstSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `dstStageMask` member of that dependency  **must**  be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the destination subpass
-    The set of bits included in any element of [`correlated_view_masks`] **must**  not overlap with the set of bits included in any other element of [`correlated_view_masks`]
-    If the [`SubpassDescription2::view_mask`] member of all elements of [`subpasses`] is `0`, [`correlated_view_mask_count`] **must**  be `0`
-    The [`SubpassDescription2::view_mask`] member of all elements of [`subpasses`] **must**  either all be `0`, or all not be `0`
-    If the [`SubpassDescription2::view_mask`] member of all elements of [`subpasses`] is `0`, the `dependencyFlags` member of any element of [`dependencies`] **must**  not include `VK_DEPENDENCY_VIEW_LOCAL_BIT`
-    For any element of [`dependencies`] where its `srcSubpass` member equals its `dstSubpass` member, if the `viewMask` member of the corresponding element of [`subpasses`] includes more than one bit, its `dependencyFlags` member  **must**  include `VK_DEPENDENCY_VIEW_LOCAL_BIT`
-    If the `attachment` member of any element of the `pInputAttachments` member of any element of [`subpasses`] is not [`ATTACHMENT_UNUSED`], the `aspectMask` member of that element of `pInputAttachments` **must**  only include aspects that are present in images of the format specified by the element of [`attachments`] specified by `attachment`
-    The `srcSubpass` member of each element of [`dependencies`] **must**  be less than [`subpass_count`]
-    The `dstSubpass` member of each element of [`dependencies`] **must**  be less than [`subpass_count`]
-    If any element of [`attachments`] is used as a fragment shading rate attachment in any subpass, it  **must**  not be used as any other attachment in the render pass
-    If [`flags`] includes `VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM`, an element of [`subpasses`] includes an instance of [`FragmentShadingRateAttachmentInfoKHR`] in its [`p_next`] chain, and the `pFragmentShadingRateAttachment` member of that structure is not equal to `NULL`, the `attachment` member of `pFragmentShadingRateAttachment` **must**  be [`ATTACHMENT_UNUSED`]
-    If any element of [`attachments`] is used as a fragment shading rate attachment in any subpass, it  **must**  have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
-    If the pipeline is being created with fragment shader state, and the [`VK_QCOM_render_pass_shader_resolve`] extension is enabled, and if subpass has any input attachments, and if the subpass description contains `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`, then the sample count of the input attachments  **must**  equal `rasterizationSamples`
-    If the pipeline is being created with fragment shader state, and the [`VK_QCOM_render_pass_shader_resolve`] extension is enabled, and if the subpass description contains `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`, then `sampleShadingEnable` **must**  be false
-    If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if `pResolveAttachments` is not `NULL`, then each resolve attachment  **must**  be [`ATTACHMENT_UNUSED`]
-    If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if `pDepthStencilResolveAttachment` is not `NULL`, then the depth/stencil resolve attachment  **must**  be [`ATTACHMENT_UNUSED`]
-    If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, then the subpass  **must**  be the last subpass in a subpass dependency chain

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`RenderPassFragmentDensityMapCreateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`RenderPassCreateFlagBits`] values
-    If [`attachment_count`] is not `0`, [`attachments`] **must**  be a valid pointer to an array of [`attachment_count`] valid [`AttachmentDescription2`] structures
-  [`subpasses`] **must**  be a valid pointer to an array of [`subpass_count`] valid [`SubpassDescription2`] structures
-    If [`dependency_count`] is not `0`, [`dependencies`] **must**  be a valid pointer to an array of [`dependency_count`] valid [`SubpassDependency2`] structures
-    If [`correlated_view_mask_count`] is not `0`, [`correlated_view_masks`] **must**  be a valid pointer to an array of [`correlated_view_mask_count`]`uint32_t` values
-  [`subpass_count`] **must**  be greater than `0`

# Related
- [`VK_KHR_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`AttachmentDescription2`]
- [`RenderPassCreateFlags`]
- [`StructureType`]
- [`SubpassDependency2`]
- [`SubpassDescription2`]
- [`create_render_pass2`]
- [`create_render_pass2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        