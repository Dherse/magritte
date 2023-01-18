[vkCmdPushConstants](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPushConstants.html) - Update the values of push constants

# C Specifications
To update push constants, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdPushConstants(
    VkCommandBuffer                             commandBuffer,
    VkPipelineLayout                            layout,
    VkShaderStageFlags                          stageFlags,
    uint32_t                                    offset,
    uint32_t                                    size,
    const void*                                 pValues);
```

# Parameters
- [`command_buffer`] is the command buffer in which the push constant update will be recorded.
- [`layout`] is the pipeline layout used to program the push constant updates.
- [`stage_flags`] is a bitmask of [`ShaderStageFlagBits`] specifying the shader stages that will use the push constants in the updated range.
- [`offset`] is the start offset of the push constant range to update, in units of bytes.
- [`size`] is the size of the push constant range to update, in units of bytes.
- [`p_values`] is a pointer to an array of [`size`] bytes containing the new push constant values.

# Description
When a command buffer begins recording, all push constant values are
undefined.
Reads of undefined push constant values by the executing shader return
undefined values.Push constant values  **can**  be updated incrementally, causing shader stages in
[`stage_flags`] to read the new data from [`p_values`] for push constants
modified by this command, while still reading the previous data for push
constants not modified by this command.
When a [bound pipeline command](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-bindpoint-commands) is issued,
the bound pipeline’s layout  **must**  be compatible with the layouts used to set
the values of all push constants in the pipeline layout’s push constant
ranges, as described in [Pipeline Layout
Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-compatibility).
Binding a pipeline with a layout that is not compatible with the push
constant layout does not disturb the push constant values.
## Valid Usage
-    For each byte in the range specified by [`offset`] and [`size`] and for each shader stage in [`stage_flags`], there  **must**  be a push constant range in [`layout`] that includes that byte and that stage
-    For each byte in the range specified by [`offset`] and [`size`] and for each push constant range that overlaps that byte, [`stage_flags`] **must**  include all stages in that push constant range’s [`PushConstantRange`]::[`stage_flags`]
-  [`offset`] **must**  be a multiple of `4`
-  [`size`] **must**  be a multiple of `4`
-  [`offset`] **must**  be less than [`PhysicalDeviceLimits::max_push_constants_size`]
-  [`size`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_push_constants_size`] minus [`offset`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`layout`] **must**  be a valid [`PipelineLayout`] handle
-  [`stage_flags`] **must**  be a valid combination of [`ShaderStageFlagBits`] values
-  [`stage_flags`] **must**  not be `0`
-  [`p_values`] **must**  be a valid pointer to an array of [`size`] bytes
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-  [`size`] **must**  be greater than `0`
-    Both of [`command_buffer`], and [`layout`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`PipelineLayout`]
- [`ShaderStageFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        