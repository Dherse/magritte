[VkPipelineRenderingCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRenderingCreateInfo.html) - Structure specifying attachment formats

# C Specifications
The [`PipelineRenderingCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPipelineRenderingCreateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           viewMask;
    uint32_t           colorAttachmentCount;
    const VkFormat*    pColorAttachmentFormats;
    VkFormat           depthAttachmentFormat;
    VkFormat           stencilAttachmentFormat;
} VkPipelineRenderingCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_dynamic_rendering
typedef VkPipelineRenderingCreateInfo VkPipelineRenderingCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`view_mask`] is the viewMask used for rendering.
- [`color_attachment_count`] is the number of entries in [`color_attachment_formats`]
- [`color_attachment_formats`] is a pointer to an array of [`Format`] values defining the format of color attachments used in this pipeline.
- [`depth_attachment_format`] is a [`Format`] value defining the format of the depth attachment used in this pipeline.
- [`stencil_attachment_format`] is a [`Format`] value defining the format of the stencil attachment used in this pipeline.

# Description
When a pipeline is created without a [`RenderPass`], if this structure
is present in the [`p_next`] chain of [`GraphicsPipelineCreateInfo`],
it specifies the view mask and format of attachments used for rendering.
If this structure is not specified, and the pipeline does not include a
[`RenderPass`], [`view_mask`] and [`color_attachment_count`] are `0`,
and [`depth_attachment_format`] and [`stencil_attachment_format`] are
`VK_FORMAT_UNDEFINED`.
If a graphics pipeline is created with a valid [`RenderPass`],
parameters of this structure are ignored.If [`depth_attachment_format`], [`stencil_attachment_format`], or any
element of [`color_attachment_formats`] is `VK_FORMAT_UNDEFINED`, it
indicates that the corresponding attachment is unused within the render
pass.
Valid formats indicate that an attachment  **can**  be used - but it is still
valid to set the attachment to `NULL` when beginning rendering.
## Valid Usage
-    If any element of [`color_attachment_formats`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that includes either `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format that includes a depth aspect
-    If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format that includes a stencil aspect
-    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED` and [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, [`depth_attachment_format`] **must**  equal [`stencil_attachment_format`]
-    If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature is not enabled, [`view_mask`] **must**  be `0`
-    The index of the most significant bit in [`view_mask`] **must**  be less than [`maxMultiviewViewCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxMultiviewViewCount)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO`
-    If [`color_attachment_count`] is not `0`, [`color_attachment_formats`] **must**  be a valid pointer to an array of [`color_attachment_count`] valid [`Format`] values
-  [`depth_attachment_format`] **must**  be a valid [`Format`] value
-  [`stencil_attachment_format`] **must**  be a valid [`Format`] value

# Related
- [`khr_dynamic_rendering`]
- [`crate::vulkan1_3`]
- [`Format`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        