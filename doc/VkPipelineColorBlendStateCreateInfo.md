[VkPipelineColorBlendStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline color blend state

# C Specifications
The [`PipelineColorBlendStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineColorBlendStateCreateInfo {
    VkStructureType                               sType;
    const void*                                   pNext;
    VkPipelineColorBlendStateCreateFlags          flags;
    VkBool32                                      logicOpEnable;
    VkLogicOp                                     logicOp;
    uint32_t                                      attachmentCount;
    const VkPipelineColorBlendAttachmentState*    pAttachments;
    float                                         blendConstants[4];
} VkPipelineColorBlendStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`PipelineColorBlendStateCreateFlagBits`] specifying additional color blending information.
- [`logic_op_enable`] controls whether to apply [Logical Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-logicop).
- [`logic_op`] selects which logical operation to apply.
- [`attachment_count`] is the number of [`PipelineColorBlendAttachmentState`] elements in [`attachments`].
- [`attachments`] is a pointer to an array of [`PipelineColorBlendAttachmentState`] structures defining blend state for each color attachment.
- [`blend_constants`] is a pointer to an array of four values used as the R, G, B, and A components of the blend constant that are used in blending, depending on the [blend factor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendfactors).

# Description
## Valid Usage
-    If the [independent blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-independentBlend) feature is not enabled, all elements of [`attachments`] **must**  be identical
-    If the [logic operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-logicOp) feature is not enabled, [`logic_op_enable`] **must**  be `VK_FALSE`
-    If [`logic_op_enable`] is `VK_TRUE`, [`logic_op`] **must**  be a valid [`LogicOp`] value
-    If the [`rasterizationOrderColorAttachmentAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rasterizationOrderColorAttachmentAccess) feature is not enabled, [`flags`] **must**  not include `VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`PipelineColorBlendAdvancedStateCreateInfoEXT`] or [`PipelineColorWriteCreateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`PipelineColorBlendStateCreateFlagBits`] values
-    If [`attachment_count`] is not `0`, [`attachments`] **must**  be a valid pointer to an array of [`attachment_count`] valid [`PipelineColorBlendAttachmentState`] structures

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`GraphicsPipelineCreateInfo`]
- [`LogicOp`]
- [`PipelineColorBlendAttachmentState`]
- [VkPipelineColorBlendStateCreateFlags]()
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        