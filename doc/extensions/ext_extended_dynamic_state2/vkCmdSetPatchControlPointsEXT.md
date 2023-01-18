[vkCmdSetPatchControlPointsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html) - Specify the number of control points per patch dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the number of control points
per patch, call:
```c
// Provided by VK_EXT_extended_dynamic_state2
void vkCmdSetPatchControlPointsEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    patchControlPoints);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`patch_control_points`] specifies the number of control points per patch.

# Description
This command sets the number of control points per patch for subsequent
drawing commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineTessellationStateCreateInfo`]::[`patch_control_points`] value
used to create the currently active pipeline.
## Valid Usage
-    The [extendedDynamicState2PatchControlPoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-extendedDynamicState2PatchControlPoints) feature  **must**  be enabled
-  [`patch_control_points`] **must**  be greater than zero and less than or equal to [`PhysicalDeviceLimits::max_tessellation_patch_size`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_extended_dynamic_state2`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        