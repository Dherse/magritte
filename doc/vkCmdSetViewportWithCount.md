[vkCmdSetViewportWithCount](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCount.html) - Set the viewport count and viewports dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the viewport count and
viewports, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetViewportWithCount(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    viewportCount,
    const VkViewport*                           pViewports);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state
void vkCmdSetViewportWithCountEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    viewportCount,
    const VkViewport*                           pViewports);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`viewport_count`] specifies the viewport count.
- [`p_viewports`] specifies the viewports to use for drawing.

# Description
This command sets the viewport count and viewports state for subsequent
drawing commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the corresponding
[`PipelineViewportStateCreateInfo`]::[`viewport_count`] and
[`p_viewports`] values used to create the currently active pipeline.
## Valid Usage
-  [`viewport_count`] **must**  be between `1` and [`PhysicalDeviceLimits::max_viewports`], inclusive
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`viewport_count`] **must**  be `1`
-  [`command_buffer`] **must**  not have [`CommandBufferInheritanceViewportScissorInfoNV::viewport_scissor2_d`] enabled

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_viewports`] **must**  be a valid pointer to an array of [`viewport_count`] valid [`Viewport`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`viewport_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_extended_dynamic_state`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`Viewport`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        