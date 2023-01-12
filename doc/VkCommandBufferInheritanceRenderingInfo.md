[VkCommandBufferInheritanceRenderingInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderingInfo.html) - Structure specifying command buffer inheritance info for dynamic render pass instances

# C Specifications
The [`CommandBufferInheritanceRenderingInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkCommandBufferInheritanceRenderingInfo {
    VkStructureType          sType;
    const void*              pNext;
    VkRenderingFlags         flags;
    uint32_t                 viewMask;
    uint32_t                 colorAttachmentCount;
    const VkFormat*          pColorAttachmentFormats;
    VkFormat                 depthAttachmentFormat;
    VkFormat                 stencilAttachmentFormat;
    VkSampleCountFlagBits    rasterizationSamples;
} VkCommandBufferInheritanceRenderingInfo;
```
or the equivalent
```c
// Provided by VK_KHR_dynamic_rendering
typedef VkCommandBufferInheritanceRenderingInfo VkCommandBufferInheritanceRenderingInfoKHR;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`flags`] is a bitmask of [`RenderingFlagBits`] used by the render pass instance.
- [`view_mask`] is the view mask used for rendering.
- [`color_attachment_count`] is the number of color attachments specified in the render pass instance.
- [`color_attachment_formats`] is a pointer to an array of [`Format`] values defining the format of color attachments.
- [`depth_attachment_format`] is a [`Format`] value defining the format of the depth attachment.
- [`stencil_attachment_format`] is a [`Format`] value defining the format of the stencil attachment.
- [`rasterization_samples`] is a [`SampleCountFlagBits`] specifying the number of samples used in rasterization.

# Description
If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
[`CommandBufferInheritanceRenderingInfo`] structure, then that structure
controls parameters of dynamic render pass instances that the
[`CommandBuffer`] **can**  be executed within.
If [`CommandBufferInheritanceInfo::render_pass`] is not
[`crate::Handle::null`], or
`VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` is not specified in
[`CommandBufferBeginInfo`]::[`flags`], parameters of this structure
are ignored.If [`color_attachment_count`] is `0` and the
[`variableMultisampleRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-variableMultisampleRate) feature
is enabled, [`rasterization_samples`] is ignored.If [`depth_attachment_format`], [`stencil_attachment_format`], or any
element of [`color_attachment_formats`] is `VK_FORMAT_UNDEFINED`, it
indicates that the corresponding attachment is unused within the render
pass.
## Valid Usage
-    If [`color_attachment_count`] is not `0`, [`rasterization_samples`] **must**  be a valid [`SampleCountFlagBits`] value
-    If the [`variableMultisampleRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-variableMultisampleRate) feature is not enabled, [`rasterization_samples`] **must**  be a valid [`SampleCountFlagBits`] value
-    If any element of [`color_attachment_formats`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format that includes a depth aspect
-    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    When rendering to a [Linear Color attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary), if any element of [`color_attachment_formats`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format that includes a stencil aspect
-    If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it  **must**  be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED` and [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, [`depth_attachment_format`] **must**  equal [`stencil_attachment_format`]
-    If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature is not enabled, [`view_mask`] **must**  be `0`
-    The index of the most significant bit in [`view_mask`] **must**  be less than [`maxMultiviewViewCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxMultiviewViewCount)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO`
-  [`flags`] **must**  be a valid combination of [`RenderingFlagBits`] values
-    If [`color_attachment_count`] is not `0`, [`color_attachment_formats`] **must**  be a valid pointer to an array of [`color_attachment_count`] valid [`Format`] values
-  [`depth_attachment_format`] **must**  be a valid [`Format`] value
-  [`stencil_attachment_format`] **must**  be a valid [`Format`] value
-    If [`rasterization_samples`] is not `0`, [`rasterization_samples`] **must**  be a valid [`SampleCountFlagBits`] value

# Related
- [`khr_dynamic_rendering`]
- [`crate::vulkan1_3`]
- [`Format`]
- [VkRenderingFlags]()
- [`SampleCountFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        