[VkGeneratedCommandsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsInfoNV.html) - Structure specifying parameters for the generation of commands

# C Specifications
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkGeneratedCommandsInfoNV {
    VkStructureType                      sType;
    const void*                          pNext;
    VkPipelineBindPoint                  pipelineBindPoint;
    VkPipeline                           pipeline;
    VkIndirectCommandsLayoutNV           indirectCommandsLayout;
    uint32_t                             streamCount;
    const VkIndirectCommandsStreamNV*    pStreams;
    uint32_t                             sequencesCount;
    VkBuffer                             preprocessBuffer;
    VkDeviceSize                         preprocessOffset;
    VkDeviceSize                         preprocessSize;
    VkBuffer                             sequencesCountBuffer;
    VkDeviceSize                         sequencesCountOffset;
    VkBuffer                             sequencesIndexBuffer;
    VkDeviceSize                         sequencesIndexOffset;
} VkGeneratedCommandsInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`pipeline_bind_point`] is the [`PipelineBindPoint`] used for the [`pipeline`].
- [`pipeline`] is the [`Pipeline`] used in the generation and execution process.
- [`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`] that provides the command sequence to generate.
- [`stream_count`] defines the number of input streams
- [`streams`] is a pointer to an array of [`stream_count`][`IndirectCommandsStreamNV`] structures providing the input data for the tokens used in [`indirect_commands_layout`].
- [`sequences_count`] is the maximum number of sequences to reserve. If [`sequences_count_buffer`] is [`crate::Handle::null`], this is also the actual number of sequences generated.
- [`preprocess_buffer`] is the [`Buffer`] that is used for preprocessing the input data for execution. If this structure is used with [`cmd_execute_generated_commands_nv`] with its `isPreprocessed` set to `VK_TRUE`, then the preprocessing step is skipped and data is only read from this buffer.
- [`preprocess_offset`] is the byte offset into [`preprocess_buffer`] where the preprocessed data is stored.
- [`preprocess_size`] is the maximum byte size within the [`preprocess_buffer`] after the [`preprocess_offset`] that is available for preprocessing.
- [`sequences_count_buffer`] is a [`Buffer`] in which the actual number of sequences is provided as single `uint32_t` value.
- [`sequences_count_offset`] is the byte offset into [`sequences_count_buffer`] where the count value is stored.
- [`sequences_index_buffer`] is a [`Buffer`] that encodes the used sequence indices as `uint32_t` array.
- [`sequences_index_offset`] is the byte offset into [`sequences_index_buffer`] where the index values start.

# Description
## Valid Usage
-    The provided [`pipeline`] **must**  match the pipeline bound at execution time
-    If the [`indirect_commands_layout`] uses a token of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV`, then the [`pipeline`] **must**  have been created with multiple shader groups
-    If the [`indirect_commands_layout`] uses a token of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV`, then the [`pipeline`] **must**  have been created with `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV` set in [`GraphicsPipelineCreateInfo::flags`]
-    If the [`indirect_commands_layout`] uses a token of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, then the [`pipeline`]`s [`PipelineLayout`] **must**  match the [`IndirectCommandsLayoutTokenNV::pushconstant_pipeline_layout`]
-  [`stream_count`] **must**  match the [`indirect_commands_layout`]’s [`stream_count`]
-  [`sequences_count`] **must**  be less or equal to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_sequence_count`] and [`GeneratedCommandsMemoryRequirementsInfoNV::max_sequences_count`] that was used to determine the [`preprocess_size`]
-  [`preprocess_buffer`] **must**  have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set in its usage flag
-  [`preprocess_offset`] **must**  be aligned to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::min_indirect_commands_buffer_offset_alignment`]
-    If [`preprocess_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`preprocess_size`] **must**  be at least equal to the memory requirement`s size returned by [`get_generated_commands_memory_requirements_nv`] using the matching inputs ([`indirect_commands_layout`], …​) as within this structure
-  [`sequences_count_buffer`] **can**  be set if the actual used count of sequences is sourced from the provided buffer. In that case the [`sequences_count`] serves as upper bound
-    If [`sequences_count_buffer`] is not [`crate::Handle::null`], its usage flag  **must**  have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
-    If [`sequences_count_buffer`] is not [`crate::Handle::null`], [`sequences_count_offset`] **must**  be aligned to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::min_sequences_count_buffer_offset_alignment`]
-    If [`sequences_count_buffer`] is not [`crate::Handle::null`] and is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If [`indirect_commands_layout`]’s `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV` is set, [`sequences_index_buffer`] **must**  be set otherwise it  **must**  be [`crate::Handle::null`]
-    If [`sequences_index_buffer`] is not [`crate::Handle::null`], its usage flag  **must**  have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
-    If [`sequences_index_buffer`] is not [`crate::Handle::null`], [`sequences_index_offset`] **must**  be aligned to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::min_sequences_index_buffer_offset_alignment`]
-    If [`sequences_index_buffer`] is not [`crate::Handle::null`] and is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`indirect_commands_layout`] **must**  be a valid [`IndirectCommandsLayoutNV`] handle
-  [`streams`] **must**  be a valid pointer to an array of [`stream_count`] valid [`IndirectCommandsStreamNV`] structures
-  [`preprocess_buffer`] **must**  be a valid [`Buffer`] handle
-    If [`sequences_count_buffer`] is not [`crate::Handle::null`], [`sequences_count_buffer`] **must**  be a valid [`Buffer`] handle
-    If [`sequences_index_buffer`] is not [`crate::Handle::null`], [`sequences_index_buffer`] **must**  be a valid [`Buffer`] handle
-  [`stream_count`] **must**  be greater than `0`
-    Each of [`indirect_commands_layout`], [`pipeline`], [`preprocess_buffer`], [`sequences_count_buffer`], and [`sequences_index_buffer`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`nv_device_generated_commands`]
- [`Buffer`]
- [`DeviceSize`]
- [`IndirectCommandsLayoutNV`]
- [`IndirectCommandsStreamNV`]
- [`Pipeline`]
- [`PipelineBindPoint`]
- [`StructureType`]
- [`cmd_execute_generated_commands_nv`]
- [`cmd_preprocess_generated_commands_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        