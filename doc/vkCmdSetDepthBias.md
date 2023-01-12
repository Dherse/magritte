[vkCmdSetDepthBias](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBias.html) - Set depth bias factors and clamp dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the depth bias parameters,
call:
```c
// Provided by VK_VERSION_1_0
void vkCmdSetDepthBias(
    VkCommandBuffer                             commandBuffer,
    float                                       depthBiasConstantFactor,
    float                                       depthBiasClamp,
    float                                       depthBiasSlopeFactor);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`depth_bias_constant_factor`] is a scalar factor controlling the constant depth value added to each fragment.
- [`depth_bias_clamp`] is the maximum (or minimum) depth bias of a fragment.
- [`depth_bias_slope_factor`] is a scalar factor applied to a fragment’s slope in depth bias calculations.

# Description
This command sets the depth bias parameters for subsequent drawing commands
when the graphics pipeline is created with `VK_DYNAMIC_STATE_DEPTH_BIAS`
set in [`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the corresponding
[`PipelineRasterizationStateCreateInfo`]::[`depth_bias_constant_factor`],
[`depth_bias_clamp`], and [`depth_bias_slope_factor`] values used to create
the currently active pipeline.
## Valid Usage
-    If the [depth bias clamping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-depthBiasClamp) feature is not enabled, [`depth_bias_clamp`] **must**  be `0.0`

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
        