[vkCmdSetDepthWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnable.html) - Set depth write enable dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the depth write enable,
call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetDepthWriteEnable(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    depthWriteEnable);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state
void vkCmdSetDepthWriteEnableEXT(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    depthWriteEnable);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`depth_write_enable`] specifies if depth writes are enabled.

# Description
This command sets the depth write enable for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineDepthStencilStateCreateInfo`]::[`depth_write_enable`] value
used to create the currently active pipeline.
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_extended_dynamic_state`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        