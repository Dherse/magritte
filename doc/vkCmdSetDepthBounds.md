[vkCmdSetDepthBounds](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBounds.html) - Set depth bounds range dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the depth bounds range,
call:
```c
// Provided by VK_VERSION_1_0
void vkCmdSetDepthBounds(
    VkCommandBuffer                             commandBuffer,
    float                                       minDepthBounds,
    float                                       maxDepthBounds);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`min_depth_bounds`] is the minimum depth bound.
- [`max_depth_bounds`] is the maximum depth bound.

# Description
This command sets the depth bounds range for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_DEPTH_BOUNDS` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineDepthStencilStateCreateInfo`]::[`min_depth_bounds`] and
[`PipelineDepthStencilStateCreateInfo`]::[`max_depth_bounds`] values
used to create the currently active pipeline.
## Valid Usage
-    Unless the `[`VK_EXT_depth_range_unrestricted`]` extension is enabled [`min_depth_bounds`] **must**  be between `0.0` and `1.0`, inclusive
-    Unless the `[`VK_EXT_depth_range_unrestricted`]` extension is enabled [`max_depth_bounds`] **must**  be between `0.0` and `1.0`, inclusive

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        