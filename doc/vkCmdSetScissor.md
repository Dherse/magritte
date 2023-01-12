[vkCmdSetScissor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissor.html) - Set scissor rectangles dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the scissor rectangles,
call:
```c
// Provided by VK_VERSION_1_0
void vkCmdSetScissor(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstScissor,
    uint32_t                                    scissorCount,
    const VkRect2D*                             pScissors);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`first_scissor`] is the index of the first scissor whose state is updated by the command.
- [`scissor_count`] is the number of scissors whose rectangles are updated by the command.
- [`p_scissors`] is a pointer to an array of [`Rect2D`] structures defining scissor rectangles.

# Description
The scissor rectangles taken from element i of [`p_scissors`] replace
the current state for the scissor index [`first_scissor`] +  i,
for i in [0, [`scissor_count`]).This command sets the scissor rectangles for subsequent drawing commands
when the graphics pipeline is created with `VK_DYNAMIC_STATE_SCISSOR`
set in [`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineViewportStateCreateInfo`]::[`p_scissors`] values used to
create the currently active pipeline.
## Valid Usage
-    The sum of [`first_scissor`] and [`scissor_count`] **must**  be between `1` and [`PhysicalDeviceLimits::max_viewports`], inclusive
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`first_scissor`] **must**  be `0`
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`scissor_count`] **must**  be `1`
-    The `x` and `y` members of `offset` member of any element of [`p_scissors`] **must**  be greater than or equal to `0`
-    Evaluation of (`offset.x` +  `extent.width`) **must**  not cause a signed integer addition overflow for any element of [`p_scissors`]
-    Evaluation of (`offset.y` +  `extent.height`) **must**  not cause a signed integer addition overflow for any element of [`p_scissors`]
-    If this command is recorded in a secondary command buffer with [`CommandBufferInheritanceViewportScissorInfoNV::viewport_scissor2_d`] enabled, then this function  **must**  not be called

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_scissors`] **must**  be a valid pointer to an array of [`scissor_count`][`Rect2D`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`scissor_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`Rect2D`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        