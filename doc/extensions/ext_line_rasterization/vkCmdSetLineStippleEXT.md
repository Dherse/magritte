[vkCmdSetLineStippleEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEXT.html) - Set line stipple dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the line stipple state,
call:
```c
// Provided by VK_EXT_line_rasterization
void vkCmdSetLineStippleEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    lineStippleFactor,
    uint16_t                                    lineStipplePattern);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`line_stipple_factor`] is the repeat factor used in stippled line rasterization.
- [`line_stipple_pattern`] is the bit pattern used in stippled line rasterization.

# Description
This command sets the line stipple state for subsequent drawing commands
when the graphics pipeline is created with
`VK_DYNAMIC_STATE_LINE_STIPPLE_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineRasterizationLineStateCreateInfoEXT`]::[`line_stipple_factor`]
and
[`PipelineRasterizationLineStateCreateInfoEXT`]::[`line_stipple_pattern`]
values used to create the currently active pipeline.
## Valid Usage
-  [`line_stipple_factor`] **must**  be in the range [1,256]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_line_rasterization`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        