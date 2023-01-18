[vkCmdSetDepthBoundsTestEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnable.html) - Set depth bounds test enable dynamically for a command buffer

# C Specifications
To [dynamically enable or disable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the depth
bounds test, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetDepthBoundsTestEnable(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    depthBoundsTestEnable);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state
void vkCmdSetDepthBoundsTestEnableEXT(
    VkCommandBuffer                             commandBuffer,
    VkBool32                                    depthBoundsTestEnable);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`depth_bounds_test_enable`] specifies if the depth bounds test is enabled.

# Description
This command sets the depth bounds enable for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineDepthStencilStateCreateInfo`]::[`depth_bounds_test_enable`]
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
- [`VK_EXT_extended_dynamic_state`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        