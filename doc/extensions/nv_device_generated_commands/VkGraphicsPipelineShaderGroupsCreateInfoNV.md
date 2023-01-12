[VkGraphicsPipelineShaderGroupsCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html) - Structure specifying parameters of a newly created multi shader group pipeline

# C Specifications
The [`GraphicsPipelineShaderGroupsCreateInfoNV`] structure is defined
as:
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
    VkStructureType                             sType;
    const void*                                 pNext;
    uint32_t                                    groupCount;
    const VkGraphicsShaderGroupCreateInfoNV*    pGroups;
    uint32_t                                    pipelineCount;
    const VkPipeline*                           pPipelines;
} VkGraphicsPipelineShaderGroupsCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`group_count`] is the number of elements in the [`groups`] array.
- [`groups`] is a pointer to an array of [`GraphicsShaderGroupCreateInfoNV`] structures specifying which state of the original [`GraphicsPipelineCreateInfo`] each shader group overrides.
- [`pipeline_count`] is the number of elements in the [`pipelines`] array.
- [`pipelines`] is a pointer to an array of graphics [`Pipeline`] structures which are referenced within the created pipeline, including all their shader groups.

# Description
When referencing shader groups by index, groups defined in the referenced
pipelines are treated as if they were defined as additional entries in
[`groups`].
They are appended in the order they appear in the [`pipelines`] array and
in the [`groups`] array when those pipelines were defined.The application  **must**  maintain the lifetime of all such referenced pipelines
based on the pipelines that make use of them.
## Valid Usage
-  [`group_count`] **must**  be at least `1` and as maximum [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_graphics_shader_group_count`]
-    The sum of [`group_count`] including those groups added from referenced [`pipelines`] **must**  also be as maximum [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_graphics_shader_group_count`]
-    The state of the first element of [`groups`] **must**  match its equivalent within the parent’s [`GraphicsPipelineCreateInfo`]
-    Each element of [`groups`] **must**  in combination with the rest of the pipeline state yield a valid state configuration
-    All elements of [`groups`] **must**  use the same shader stage combinations unless any mesh shader stage is used, then either combination of task and mesh or just mesh shader is valid
-    Mesh and regular primitive shading stages cannot be mixed across [`groups`]
-    Each element of [`pipelines`] **must**  have been created with identical state to the pipeline currently created except the state that can be overridden by [`GraphicsShaderGroupCreateInfoNV`]
-    The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`
-  [`groups`] **must**  be a valid pointer to an array of [`group_count`] valid [`GraphicsShaderGroupCreateInfoNV`] structures
-    If [`pipeline_count`] is not `0`, [`pipelines`] **must**  be a valid pointer to an array of [`pipeline_count`] valid [`Pipeline`] handles
-  [`group_count`] **must**  be greater than `0`

# Related
- [`nv_device_generated_commands`]
- [`GraphicsShaderGroupCreateInfoNV`]
- [`Pipeline`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        