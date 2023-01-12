[vkCmdSetViewport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewport.html) - Set the viewport dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the viewport transformation
parameters, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdSetViewport(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstViewport,
    uint32_t                                    viewportCount,
    const VkViewport*                           pViewports);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`first_viewport`] is the index of the first viewport whose parameters are updated by the command.
- [`viewport_count`] is the number of viewports whose parameters are updated by the command.
- [`p_viewports`] is a pointer to an array of [`Viewport`] structures specifying viewport parameters.

# Description
This command sets the viewport transformation parameters state for
subsequent drawing commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_VIEWPORT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineViewportStateCreateInfo`]::[`p_viewports`] values used to
create the currently active pipeline.The viewport parameters taken from element i of [`p_viewports`]
replace the current state for the viewport index [`first_viewport`]
+  i, for i in [0, [`viewport_count`]).
## Valid Usage
-    The sum of [`first_viewport`] and [`viewport_count`] **must**  be between `1` and [`PhysicalDeviceLimits::max_viewports`], inclusive
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`first_viewport`] **must**  be `0`
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
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`Viewport`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        