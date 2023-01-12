[vkCmdSetPrimitiveRestartEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnable.html) - Set primitive assembly restart state dynamically for a command buffer

# C Specifications
To [dynamically control](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) whether a special vertex
index value is treated as restarting the assembly of primitives, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetPrimitiveRestartEnable(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    primitiveRestartEnable);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state2
void vkCmdSetPrimitiveRestartEnableEXT(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    primitiveRestartEnable);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`primitive_restart_enable`] controls whether a special vertex index value is treated as restarting the assembly of primitives. It behaves in the same way as [`PipelineInputAssemblyStateCreateInfo`]::[`primitive_restart_enable`]

# Description
This command sets the primitive restart enable for subsequent drawing
commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineInputAssemblyStateCreateInfo`]::[`primitive_restart_enable`]
value used to create the currently active pipeline.
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_extended_dynamic_state2`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        