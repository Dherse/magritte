[vkCmdSetDiscardRectangleEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) - Set discard rectangles dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the discard rectangles,
call:
```c
// Provided by VK_EXT_discard_rectangles
void vkCmdSetDiscardRectangleEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstDiscardRectangle,
    uint32_t                                    discardRectangleCount,
    const VkRect2D*                             pDiscardRectangles);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`first_discard_rectangle`] is the index of the first discard rectangle whose state is updated by the command.
- [`discard_rectangle_count`] is the number of discard rectangles whose state are updated by the command.
- [`p_discard_rectangles`] is a pointer to an array of [`Rect2D`] structures specifying discard rectangles.

# Description
The discard rectangle taken from element i of [`p_discard_rectangles`]
replace the current state for the discard rectangle at index
[`first_discard_rectangle`] +  i, for i in [0,
[`discard_rectangle_count`]).This command sets the discard rectangles for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineDiscardRectangleStateCreateInfoEXT`]::[`p_discard_rectangles`]
values used to create the currently active pipeline.
## Valid Usage
-    The sum of [`first_discard_rectangle`] and [`discard_rectangle_count`] **must**  be less than or equal to [`PhysicalDeviceDiscardRectanglePropertiesEXT::max_discard_rectangles`]
-    The `x` and `y` member of `offset` in each [`Rect2D`] element of [`p_discard_rectangles`] **must**  be greater than or equal to `0`
-    Evaluation of (`offset.x` +  `extent.width`) in each [`Rect2D`] element of [`p_discard_rectangles`] **must**  not cause a signed integer addition overflow
-    Evaluation of (`offset.y` +  `extent.height`) in each [`Rect2D`] element of [`p_discard_rectangles`] **must**  not cause a signed integer addition overflow
-    If this command is recorded in a secondary command buffer with [`CommandBufferInheritanceViewportScissorInfoNV::viewport_scissor2_d`] enabled, then this function  **must**  not be called

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_discard_rectangles`] **must**  be a valid pointer to an array of [`discard_rectangle_count`][`Rect2D`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`discard_rectangle_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_discard_rectangles`]
- [`CommandBuffer`]
- [`Rect2D`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        