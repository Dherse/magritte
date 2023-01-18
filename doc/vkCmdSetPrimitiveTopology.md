[vkCmdSetPrimitiveTopology](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopology.html) - Set primitive topology state dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) primitive topology, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetPrimitiveTopology(
    VkCommandBuffer                             commandBuffer,
    VkPrimitiveTopology                         primitiveTopology);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state
void vkCmdSetPrimitiveTopologyEXT(
    VkCommandBuffer                             commandBuffer,
    VkPrimitiveTopology                         primitiveTopology);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`primitive_topology`] specifies the primitive topology to use for drawing.

# Description
This command sets the primitive topology for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineInputAssemblyStateCreateInfo::topology`] value used to
create the currently active pipeline.
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`primitive_topology`] **must**  be a valid [`PrimitiveTopology`] value
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
- [`PrimitiveTopology`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        