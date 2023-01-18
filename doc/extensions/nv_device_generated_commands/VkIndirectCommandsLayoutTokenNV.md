[VkIndirectCommandsLayoutTokenNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html) - Struct specifying the details of an indirect command layout token

# C Specifications
The [`IndirectCommandsLayoutTokenNV`] structure specifies details to the
function arguments that need to be known at layout creation time:
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkIndirectCommandsLayoutTokenNV {
    VkStructureType                  sType;
    const void*                      pNext;
    VkIndirectCommandsTokenTypeNV    tokenType;
    uint32_t                         stream;
    uint32_t                         offset;
    uint32_t                         vertexBindingUnit;
    VkBool32                         vertexDynamicStride;
    VkPipelineLayout                 pushconstantPipelineLayout;
    VkShaderStageFlags               pushconstantShaderStageFlags;
    uint32_t                         pushconstantOffset;
    uint32_t                         pushconstantSize;
    VkIndirectStateFlagsNV           indirectStateFlags;
    uint32_t                         indexTypeCount;
    const VkIndexType*               pIndexTypes;
    const uint32_t*                  pIndexTypeValues;
} VkIndirectCommandsLayoutTokenNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`token_type`] specifies the token command type.
- [`stream`] is the index of the input stream containing the token argument data.
- [`offset`] is a relative starting offset within the input stream memory for the token argument data.
- [`vertex_binding_unit`] is used for the vertex buffer binding command.
- [`vertex_dynamic_stride`] sets if the vertex buffer stride is provided by the binding command rather than the current bound graphics pipeline state.
- [`pushconstant_pipeline_layout`] is the [`PipelineLayout`] used for the push constant command.
- [`pushconstant_shader_stage_flags`] are the shader stage flags used for the push constant command.
- [`pushconstant_offset`] is the offset used for the push constant command.
- [`pushconstant_size`] is the size used for the push constant command.
- [`indirect_state_flags`] are the active states for the state flag command.
- [`index_type_count`] is the optional size of the [`index_types`] and [`index_type_values`] array pairings. If not zero, it allows to register a custom `uint32_t` value to be treated as specific [`IndexType`].
- [`index_types`] is the used [`IndexType`] for the corresponding `uint32_t` value entry in [`index_type_values`].

# Description
## Valid Usage
-  [`stream`] **must**  be smaller than [`IndirectCommandsLayoutCreateInfoNV::stream_count`]
-  [`offset`] **must**  be less than or equal to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_token_offset`]
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV`, [`vertex_binding_unit`] **must**  stay within device supported limits for the appropriate commands
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, [`pushconstant_pipeline_layout`] **must**  be valid
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, [`pushconstant_offset`] **must**  be a multiple of `4`
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, [`pushconstant_size`] **must**  be a multiple of `4`
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, [`pushconstant_offset`] **must**  be less than [`PhysicalDeviceLimits::max_push_constants_size`]
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, [`pushconstant_size`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_push_constants_size`] minus [`pushconstant_offset`]
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, for each byte in the range specified by [`pushconstant_offset`] and [`pushconstant_size`] and for each shader stage in [`pushconstant_shader_stage_flags`], there  **must**  be a push constant range in [`pushconstant_pipeline_layout`] that includes that byte and that stage
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, for each byte in the range specified by [`pushconstant_offset`] and [`pushconstant_size`] and for each push constant range that overlaps that byte, [`pushconstant_shader_stage_flags`] **must**  include all stages in that push constant range’s [`PushConstantRange::stage_flags`]
-    If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV`, [`indirect_state_flags`] **must**  not be `0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`
-  [`p_next`] **must**  be `NULL`
-  [`token_type`] **must**  be a valid [`IndirectCommandsTokenTypeNV`] value
-    If [`pushconstant_pipeline_layout`] is not [`crate::Handle::null`], [`pushconstant_pipeline_layout`] **must**  be a valid [`PipelineLayout`] handle
-  [`pushconstant_shader_stage_flags`] **must**  be a valid combination of [`ShaderStageFlagBits`] values
-  [`indirect_state_flags`] **must**  be a valid combination of [`IndirectStateFlagBitsNV`] values
-    If [`index_type_count`] is not `0`, [`index_types`] **must**  be a valid pointer to an array of [`index_type_count`] valid [`IndexType`] values
-    If [`index_type_count`] is not `0`, [`index_type_values`] **must**  be a valid pointer to an array of [`index_type_count`]`uint32_t` values

# Related
- [`VK_NV_device_generated_commands`]
- [`Bool32`]
- [`IndexType`]
- [`IndirectCommandsLayoutCreateInfoNV`]
- [`IndirectCommandsTokenTypeNV`]
- [`IndirectStateFlagsNV`]
- [`PipelineLayout`]
- [`ShaderStageFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        