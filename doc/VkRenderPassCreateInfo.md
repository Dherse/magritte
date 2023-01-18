[VkRenderPassCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo.html) - Structure specifying parameters of a newly created render pass

# C Specifications
The [`RenderPassCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkRenderPassCreateInfo {
    VkStructureType                   sType;
    const void*                       pNext;
    VkRenderPassCreateFlags           flags;
    uint32_t                          attachmentCount;
    const VkAttachmentDescription*    pAttachments;
    uint32_t                          subpassCount;
    const VkSubpassDescription*       pSubpasses;
    uint32_t                          dependencyCount;
    const VkSubpassDependency*        pDependencies;
} VkRenderPassCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`RenderPassCreateFlagBits`]
- [`attachment_count`] is the number of attachments used by this render pass.
- [`attachments`] is a pointer to an array of [`attachment_count`][`AttachmentDescription`] structures describing the attachments used by the render pass.
- [`subpass_count`] is the number of subpasses to create.
- [`subpasses`] is a pointer to an array of [`subpass_count`][`SubpassDescription`] structures describing each subpass.
- [`dependency_count`] is the number of memory dependencies between pairs of subpasses.
- [`dependencies`] is a pointer to an array of [`dependency_count`][`SubpassDependency`] structures describing dependencies between pairs of subpasses.

# Description
## Valid Usage
-    If the `attachment` member of any element of `pInputAttachments`, `pColorAttachments`, `pResolveAttachments` or `pDepthStencilAttachment`, or any element of `pPreserveAttachments` in any element of [`subpasses`] is not [`ATTACHMENT_UNUSED`], then it  **must**  be less than [`attachment_count`]
-    If the pNext chain includes a [`RenderPassFragmentDensityMapCreateInfoEXT`] structure and the `fragmentDensityMapAttachment` member is not [`ATTACHMENT_UNUSED`], then `attachment` **must**  be less than [`attachment_count`]
-    For any member of [`attachments`] with a `loadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment  **must**  not specify a `layout` equal to `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`
-    For any member of [`attachments`] with a `stencilLoadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment  **must**  not specify a `layout` equal to `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`
-    For any member of [`attachments`] with a `loadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment  **must**  not specify a `layout` equal to `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
-    For any member of [`attachments`] with a `stencilLoadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment  **must**  not specify a `layout` equal to `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`
-    If the [`p_next`] chain includes a [`RenderPassInputAttachmentAspectCreateInfo`] structure, the `subpass` member of each element of its `pAspectReferences` member  **must**  be less than [`subpass_count`]
-    If the [`p_next`] chain includes a [`RenderPassInputAttachmentAspectCreateInfo`] structure, the `inputAttachmentIndex` member of each element of its `pAspectReferences` member  **must**  be less than the value of `inputAttachmentCount` in the element of [`subpasses`] identified by its `subpass` member
-    If the [`p_next`] chain includes a [`RenderPassInputAttachmentAspectCreateInfo`] structure, for any element of the `pInputAttachments` member of any element of [`subpasses`] where the `attachment` member is not [`ATTACHMENT_UNUSED`], the `aspectMask` member of the corresponding element of [`RenderPassInputAttachmentAspectCreateInfo::aspect_references`] **must**  only include aspects that are present in images of the format specified by the element of [`attachments`] at `attachment`
-    If the [`p_next`] chain includes a [`RenderPassMultiviewCreateInfo`] structure, and its [`subpass_count`] member is not zero, that member  **must**  be equal to the value of [`subpass_count`]
-    If the [`p_next`] chain includes a [`RenderPassMultiviewCreateInfo`] structure, if its [`dependency_count`] member is not zero, it  **must**  be equal to [`dependency_count`]
-    If the [`p_next`] chain includes a [`RenderPassMultiviewCreateInfo`] structure, for each non-zero element of `pViewOffsets`, the `srcSubpass` and `dstSubpass` members of [`dependencies`] at the same index  **must**  not be equal
-    If the [`p_next`] chain includes a [`RenderPassMultiviewCreateInfo`] structure, for any element of [`dependencies`] with a `dependencyFlags` member that does not include `VK_DEPENDENCY_VIEW_LOCAL_BIT`, the corresponding element of the `pViewOffsets` member of that [`RenderPassMultiviewCreateInfo`] instance  **must**  be `0`
-    If the [`p_next`] chain includes a [`RenderPassMultiviewCreateInfo`] structure, elements of its `pViewMasks` member  **must**  either all be `0`, or all not be `0`
-    If the [`p_next`] chain includes a [`RenderPassMultiviewCreateInfo`] structure, and each element of its `pViewMasks` member is `0`, the `dependencyFlags` member of each element of [`dependencies`] **must**  not include `VK_DEPENDENCY_VIEW_LOCAL_BIT`
-    If the [`p_next`] chain includes a [`RenderPassMultiviewCreateInfo`] structure, and each element of its `pViewMasks` member is `0`, its `correlationMaskCount` member  **must**  be `0`
-    For any element of [`dependencies`], if the `srcSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `srcStageMask` member of that dependency  **must**  be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the source subpass
-    For any element of [`dependencies`], if the `dstSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `dstStageMask` member of that dependency  **must**  be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the destination subpass
-    The `srcSubpass` member of each element of [`dependencies`] **must**  be less than [`subpass_count`]
-    The `dstSubpass` member of each element of [`dependencies`] **must**  be less than [`subpass_count`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`RenderPassFragmentDensityMapCreateInfoEXT`], [`RenderPassInputAttachmentAspectCreateInfo`], or [`RenderPassMultiviewCreateInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`RenderPassCreateFlagBits`] values
-    If [`attachment_count`] is not `0`, [`attachments`] **must**  be a valid pointer to an array of [`attachment_count`] valid [`AttachmentDescription`] structures
-  [`subpasses`] **must**  be a valid pointer to an array of [`subpass_count`] valid [`SubpassDescription`] structures
-    If [`dependency_count`] is not `0`, [`dependencies`] **must**  be a valid pointer to an array of [`dependency_count`] valid [`SubpassDependency`] structures
-  [`subpass_count`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_0`]
- [`AttachmentDescription`]
- [`RenderPassCreateFlags`]
- [`StructureType`]
- [`SubpassDependency`]
- [`SubpassDescription`]
- [`create_render_pass`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        