[vkCmdSetExclusiveScissorNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html) - Set exclusive scissor rectangles dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the exclusive scissor
rectangles, call:
```c
// Provided by VK_NV_scissor_exclusive
void vkCmdSetExclusiveScissorNV(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstExclusiveScissor,
    uint32_t                                    exclusiveScissorCount,
    const VkRect2D*                             pExclusiveScissors);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`first_exclusive_scissor`] is the index of the first exclusive scissor rectangle whose state is updated by the command.
- [`exclusive_scissor_count`] is the number of exclusive scissor rectangles updated by the command.
- [`p_exclusive_scissors`] is a pointer to an array of [`Rect2D`] structures defining exclusive scissor rectangles.

# Description
The scissor rectangles taken from element i of
[`p_exclusive_scissors`] replace the current state for the scissor index
[`first_exclusive_scissor`] +  i, for i in [0,
[`exclusive_scissor_count`]).This command sets the exclusive scissor rectangles for subsequent drawing
commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineViewportExclusiveScissorStateCreateInfoNV`]::[`p_exclusive_scissors`]
values used to create the currently active pipeline.
## Valid Usage
-    The [exclusive scissor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-exclusiveScissor) feature  **must**  be enabled
-    The sum of [`first_exclusive_scissor`] and [`exclusive_scissor_count`] **must**  be between `1` and [`PhysicalDeviceLimits::max_viewports`], inclusive
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`first_exclusive_scissor`] **must**  be `0`
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`exclusive_scissor_count`] **must**  be `1`
-    The `x` and `y` members of `offset` in each member of [`p_exclusive_scissors`] **must**  be greater than or equal to `0`
-    Evaluation of (`offset.x` +  `extent.width`) for each member of [`p_exclusive_scissors`] **must**  not cause a signed integer addition overflow
-    Evaluation of (`offset.y` +  `extent.height`) for each member of [`p_exclusive_scissors`] **must**  not cause a signed integer addition overflow

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_exclusive_scissors`] **must**  be a valid pointer to an array of [`exclusive_scissor_count`][`Rect2D`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`exclusive_scissor_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`nv_scissor_exclusive`]
- [`CommandBuffer`]
- [`Rect2D`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        