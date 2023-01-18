[vkCmdSetRasterizerDiscardEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnable.html) - Control whether primitives are discarded before the rasterization stage dynamically for a command buffer

# C Specifications
To [dynamically enable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) whether primitives are
discarded before the rasterization stage, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetRasterizerDiscardEnable(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    rasterizerDiscardEnable);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state2
void vkCmdSetRasterizerDiscardEnableEXT(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    rasterizerDiscardEnable);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`rasterizer_discard_enable`] controls whether primitives are discarded immediately before the rasterization stage.

# Description
This command sets the discard enable for subsequent drawing commands when
the graphics pipeline is created with
`VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineRasterizationStateCreateInfo`]::[`rasterizer_discard_enable`]
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
- [`VK_EXT_extended_dynamic_state2`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        