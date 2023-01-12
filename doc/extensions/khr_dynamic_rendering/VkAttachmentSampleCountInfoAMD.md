[VkAttachmentSampleCountInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleCountInfoAMD.html) - Structure specifying command buffer inheritance info for dynamic render pass instances

# C Specifications
The
[`AttachmentSampleCountInfoAMD`]
or
[`AttachmentSampleCountInfoNV`]
structure is defined as:
```c
// Provided by VK_KHR_dynamic_rendering with VK_AMD_mixed_attachment_samples
typedef struct VkAttachmentSampleCountInfoAMD {
    VkStructureType                 sType;
    const void*                     pNext;
    uint32_t                        colorAttachmentCount;
    const VkSampleCountFlagBits*    pColorAttachmentSamples;
    VkSampleCountFlagBits           depthStencilAttachmentSamples;
} VkAttachmentSampleCountInfoAMD;
```
or the equivalent
```c
// Provided by VK_KHR_dynamic_rendering with VK_NV_framebuffer_mixed_samples
typedef VkAttachmentSampleCountInfoAMD VkAttachmentSampleCountInfoNV;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`color_attachment_count`] is the number of color attachments specified in a render pass instance.
- [`color_attachment_samples`] is a pointer to an array of [`SampleCountFlagBits`] values defining the sample count of color attachments.
- [`depth_stencil_attachment_samples`] is a [`SampleCountFlagBits`] value defining the sample count of a depth/stencil attachment.

# Description
If [`CommandBufferInheritanceInfo::render_pass`] is
[`crate::Handle::null`], `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`
is specified in [`CommandBufferBeginInfo::flags`], and the
[`p_next`] chain of [`CommandBufferInheritanceInfo`] includes
[`AttachmentSampleCountInfoAMD`], then this structure defines the sample
counts of each attachment within the render pass instance.
If [`AttachmentSampleCountInfoAMD`] is not included, the value of
[`CommandBufferInheritanceRenderingInfo::rasterization_samples`] is
used as the sample count for each attachment.
If [`CommandBufferInheritanceInfo::render_pass`] is not
[`crate::Handle::null`], or
`VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` is not specified in
[`CommandBufferBeginInfo::flags`], parameters of this structure
are ignored.[`AttachmentSampleCountInfoAMD`] **can**  also be included in the
[`p_next`] chain of [`GraphicsPipelineCreateInfo`].
When a graphics pipeline is created without a [`RenderPass`], if this
structure is present in the [`p_next`] chain of
[`GraphicsPipelineCreateInfo`], it specifies the sample count of
attachments used for rendering.
If this structure is not specified, and the pipeline does not include a
[`RenderPass`], the value of
[`PipelineMultisampleStateCreateInfo::rasterization_samples`] is
used as the sample count for each attachment.
If a graphics pipeline is created with a valid [`RenderPass`],
parameters of this structure are ignored.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD`
-    If [`color_attachment_count`] is not `0`, [`color_attachment_samples`] **must**  be a valid pointer to an array of [`color_attachment_count`] valid [`SampleCountFlagBits`] values
-    If [`depth_stencil_attachment_samples`] is not `0`, [`depth_stencil_attachment_samples`] **must**  be a valid [`SampleCountFlagBits`] value

# Related
- [`amd_mixed_attachment_samples`]
- [`khr_dynamic_rendering`]
- [`nv_framebuffer_mixed_samples`]
- [`SampleCountFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        