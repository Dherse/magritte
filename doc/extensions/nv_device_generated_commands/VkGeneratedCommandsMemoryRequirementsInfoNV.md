[VkGeneratedCommandsMemoryRequirementsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html) - Structure specifying parameters for the reservation of preprocess buffer space

# C Specifications
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkGeneratedCommandsMemoryRequirementsInfoNV {
    VkStructureType               sType;
    const void*                   pNext;
    VkPipelineBindPoint           pipelineBindPoint;
    VkPipeline                    pipeline;
    VkIndirectCommandsLayoutNV    indirectCommandsLayout;
    uint32_t                      maxSequencesCount;
} VkGeneratedCommandsMemoryRequirementsInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`pipeline_bind_point`] is the [`PipelineBindPoint`] of the [`pipeline`] that this buffer memory is intended to be used with during the execution.
- [`pipeline`] is the [`Pipeline`] that this buffer memory is intended to be used with during the execution.
- [`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`] that this buffer memory is intended to be used with.
- [`max_sequences_count`] is the maximum number of sequences that this buffer memory in combination with the other state provided  **can**  be used with.

# Description
## Valid Usage
-  [`max_sequences_count`] **must**  be less or equal to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_sequence_count`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`indirect_commands_layout`] **must**  be a valid [`IndirectCommandsLayoutNV`] handle
-    Both of [`indirect_commands_layout`], and [`pipeline`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`nv_device_generated_commands`]
- [`IndirectCommandsLayoutNV`]
- [`Pipeline`]
- [`PipelineBindPoint`]
- [`StructureType`]
- [`get_generated_commands_memory_requirements_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        