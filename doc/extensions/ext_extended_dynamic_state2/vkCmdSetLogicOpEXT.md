[vkCmdSetLogicOpEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html) - Select which logical operation to apply for blend state dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the logical operation to
apply for blend state, call:
```c
// Provided by VK_EXT_extended_dynamic_state2
void vkCmdSetLogicOpEXT(
    VkCommandBuffer                             commandBuffer,
    VkLogicOp                                   logicOp);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`logic_op`] specifies the logical operation to apply for blend state.

# Description
This command sets the logical operation for blend state for subsequent
drawing commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_LOGIC_OP_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineColorBlendStateCreateInfo`]::[`logic_op`] value used to
create the currently active pipeline.
## Valid Usage
-    The [extendedDynamicState2LogicOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-extendedDynamicState2LogicOp) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`logic_op`] **must**  be a valid [`LogicOp`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_extended_dynamic_state2`]
- [`CommandBuffer`]
- [`LogicOp`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        