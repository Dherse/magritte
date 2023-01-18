[VkGraphicsShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html) - Structure specifying override parameters for each shader group

# C Specifications
The [`GraphicsShaderGroupCreateInfoNV`] structure provides the state
overrides for each shader group.
Each shader group behaves like a pipeline that was created from its state as
well as the remaining parent’s state.
It is defined as:
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkGraphicsShaderGroupCreateInfoNV {
    VkStructureType                                 sType;
    const void*                                     pNext;
    uint32_t                                        stageCount;
    const VkPipelineShaderStageCreateInfo*          pStages;
    const VkPipelineVertexInputStateCreateInfo*     pVertexInputState;
    const VkPipelineTessellationStateCreateInfo*    pTessellationState;
} VkGraphicsShaderGroupCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`stage_count`] is the number of entries in the [`stages`] array.
- [`stages`] is a pointer to an array [`PipelineShaderStageCreateInfo`] structures specifying the set of the shader stages to be included in this shader group.
- [`vertex_input_state`] is a pointer to a [`PipelineVertexInputStateCreateInfo`] structure.
- [`tessellation_state`] is a pointer to a [`PipelineTessellationStateCreateInfo`] structure, and is ignored if the shader group does not include a tessellation control shader stage and tessellation evaluation shader stage.

# Description
## Valid Usage
-    For [`stage_count`], the same restrictions as in [`GraphicsPipelineCreateInfo`]::[`stage_count`] apply
-    For [`stages`], the same restrictions as in [`GraphicsPipelineCreateInfo`]::[`stages`] apply
-    For [`vertex_input_state`], the same restrictions as in [`GraphicsPipelineCreateInfo`]::[`vertex_input_state`] apply
-    For [`tessellation_state`], the same restrictions as in [`GraphicsPipelineCreateInfo`]::[`tessellation_state`] apply

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`stages`] **must**  be a valid pointer to an array of [`stage_count`] valid [`PipelineShaderStageCreateInfo`] structures
-  [`stage_count`] **must**  be greater than `0`

# Related
- [`VK_NV_device_generated_commands`]
- [`GraphicsPipelineShaderGroupsCreateInfoNV`]
- [`PipelineShaderStageCreateInfo`]
- [`PipelineTessellationStateCreateInfo`]
- [`PipelineVertexInputStateCreateInfo`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        