[vkCmdSetViewportWScalingNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html) - Set the viewport W scaling dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the viewport  **W**  scaling
parameters, call:
```c
// Provided by VK_NV_clip_space_w_scaling
void vkCmdSetViewportWScalingNV(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstViewport,
    uint32_t                                    viewportCount,
    const VkViewportWScalingNV*                 pViewportWScalings);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`first_viewport`] is the index of the first viewport whose parameters are updated by the command.
- [`viewport_count`] is the number of viewports whose parameters are updated by the command.
- [`p_viewport_w_scalings`] is a pointer to an array of [`ViewportWScalingNV`] structures specifying viewport parameters.

# Description
The viewport parameters taken from element i of
[`p_viewport_w_scalings`] replace the current state for the viewport index
[`first_viewport`] +  i, for i in [0,
[`viewport_count`]).This command sets the viewport  **W**  scaling for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineViewportWScalingStateCreateInfoNV`]::[`p_viewport_w_scalings`]
values used to create the currently active pipeline.
## Valid Usage
-    The sum of [`first_viewport`] and [`viewport_count`] **must**  be between `1` and [`PhysicalDeviceLimits::max_viewports`], inclusive

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_viewport_w_scalings`] **must**  be a valid pointer to an array of [`viewport_count`][`ViewportWScalingNV`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`viewport_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_NV_clip_space_w_scaling`]
- [`CommandBuffer`]
- [`ViewportWScalingNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        