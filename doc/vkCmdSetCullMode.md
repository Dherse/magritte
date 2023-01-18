[vkCmdSetCullMode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullMode.html) - Set cull mode dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the cull mode, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetCullMode(
    VkCommandBuffer                             commandBuffer,
    VkCullModeFlags                             cullMode);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state
void vkCmdSetCullModeEXT(
    VkCommandBuffer                             commandBuffer,
    VkCullModeFlags                             cullMode);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`cull_mode`] specifies the cull mode property to use for drawing.

# Description
This command sets the cull mode for subsequent drawing commands when the
graphics pipeline is created with `VK_DYNAMIC_STATE_CULL_MODE` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineRasterizationStateCreateInfo`]::[`cull_mode`] value used to
create the currently active pipeline.
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`cull_mode`] **must**  be a valid combination of [`CullModeFlagBits`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_extended_dynamic_state`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`CullModeFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        