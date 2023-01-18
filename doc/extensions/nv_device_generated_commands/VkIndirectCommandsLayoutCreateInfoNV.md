[VkIndirectCommandsLayoutCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html) - Structure specifying the parameters of a newly created indirect commands layout object

# C Specifications
The [`IndirectCommandsLayoutCreateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_device_generated_commands
typedef struct VkIndirectCommandsLayoutCreateInfoNV {
    VkStructureType                           sType;
    const void*                               pNext;
    VkIndirectCommandsLayoutUsageFlagsNV      flags;
    VkPipelineBindPoint                       pipelineBindPoint;
    uint32_t                                  tokenCount;
    const VkIndirectCommandsLayoutTokenNV*    pTokens;
    uint32_t                                  streamCount;
    const uint32_t*                           pStreamStrides;
} VkIndirectCommandsLayoutCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`pipeline_bind_point`] is the [`PipelineBindPoint`] that this layout targets.
- [`flags`] is a bitmask of [`IndirectCommandsLayoutUsageFlagBitsNV`] specifying usage hints of this layout.
- [`token_count`] is the length of the individual command sequence.
- [`tokens`] is an array describing each command token in detail. See [`IndirectCommandsTokenTypeNV`] and [`IndirectCommandsLayoutTokenNV`] below for details.
- [`stream_count`] is the number of streams used to provide the token inputs.
- [`stream_strides`] is an array defining the byte stride for each input stream.

# Description
The following code illustrates some of the flags:
```c
void cmdProcessAllSequences(cmd, pipeline, indirectCommandsLayout, pIndirectCommandsTokens, sequencesCount, indexbuffer, indexbufferOffset)
{
  for (s = 0; s < sequencesCount; s++)
  {
    sUsed = s;

    if (indirectCommandsLayout.flags & VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV) {
      sUsed = indexbuffer.load_uint32( sUsed * sizeof(uint32_t) + indexbufferOffset);
    }

    if (indirectCommandsLayout.flags & VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV) {
      sUsed = incoherent_implementation_dependent_permutation[ sUsed ];
    }

    cmdProcessSequence( cmd, pipeline, indirectCommandsLayout, pIndirectCommandsTokens, sUsed );
  }
}
```

## Valid Usage
-    The [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS`
-  [`token_count`] **must**  be greater than `0` and less than or equal to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_token_count`]
-    If [`tokens`] contains an entry of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV` it  **must**  be the first element of the array and there  **must**  be only a single element of such token type
-    If [`tokens`] contains an entry of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV` there  **must**  be only a single element of such token type
-    All state tokens in [`tokens`] **must**  occur prior work provoking tokens (`VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV`, `VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV`, `VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV`)
-    The content of [`tokens`] **must**  include one single work provoking token that is compatible with the [`pipeline_bind_point`]
-  [`stream_count`] **must**  be greater than `0` and less or equal to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_stream_count`]
-    each element of [`stream_strides`] **must**  be greater than `0`and less than or equal to [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_stream_stride`]. Furthermore the alignment of each token input  **must**  be ensured

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`IndirectCommandsLayoutUsageFlagBitsNV`] values
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-  [`tokens`] **must**  be a valid pointer to an array of [`token_count`] valid [`IndirectCommandsLayoutTokenNV`] structures
-  [`stream_strides`] **must**  be a valid pointer to an array of [`stream_count`]`uint32_t` values
-  [`token_count`] **must**  be greater than `0`
-  [`stream_count`] **must**  be greater than `0`

# Related
- [`VK_NV_device_generated_commands`]
- [`IndirectCommandsLayoutTokenNV`]
- [`IndirectCommandsLayoutUsageFlagsNV`]
- [`PipelineBindPoint`]
- [`StructureType`]
- [`create_indirect_commands_layout_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        