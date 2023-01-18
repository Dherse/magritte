[VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html) - Structure describing push descriptor limits that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`] structure is
defined as:
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           maxGraphicsShaderGroupCount;
    uint32_t           maxIndirectSequenceCount;
    uint32_t           maxIndirectCommandsTokenCount;
    uint32_t           maxIndirectCommandsStreamCount;
    uint32_t           maxIndirectCommandsTokenOffset;
    uint32_t           maxIndirectCommandsStreamStride;
    uint32_t           minSequencesCountBufferOffsetAlignment;
    uint32_t           minSequencesIndexBufferOffsetAlignment;
    uint32_t           minIndirectCommandsBufferOffsetAlignment;
} VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_graphics_shader_group_count`] is the maximum number of shader groups in [`GraphicsPipelineShaderGroupsCreateInfoNV`].
- [`max_indirect_sequence_count`] is the maximum number of sequences in [`GeneratedCommandsInfoNV`] and in [`GeneratedCommandsMemoryRequirementsInfoNV`].
- [`max_indirect_commands_token_count`] is the maximum number of tokens in [`IndirectCommandsLayoutCreateInfoNV`].
- [`max_indirect_commands_stream_count`] is the maximum number of streams in [`IndirectCommandsLayoutCreateInfoNV`].
- [`max_indirect_commands_token_offset`] is the maximum offset in [`IndirectCommandsLayoutTokenNV`].
- [`max_indirect_commands_stream_stride`] is the maximum stream stride in [`IndirectCommandsLayoutCreateInfoNV`].
- [`min_sequences_count_buffer_offset_alignment`] is the minimum alignment for memory addresses which  **can**  be used in [`GeneratedCommandsInfoNV`].
- [`min_sequences_index_buffer_offset_alignment`] is the minimum alignment for memory addresses which  **can**  be used in [`GeneratedCommandsInfoNV`].
- [`min_indirect_commands_buffer_offset_alignment`] is the minimum alignment for memory addresses used in [`IndirectCommandsStreamNV`], and as preprocess buffer in [`GeneratedCommandsInfoNV`].

# Description
If the [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`

# Related
- [`VK_NV_device_generated_commands`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        