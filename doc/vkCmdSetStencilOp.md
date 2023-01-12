[vkCmdSetStencilOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOp.html) - Set stencil operation dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the stencil operation, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdSetStencilOp(
    VkCommandBuffer                             commandBuffer,
    VkStencilFaceFlags                          faceMask,
    VkStencilOp                                 failOp,
    VkStencilOp                                 passOp,
    VkStencilOp                                 depthFailOp,
    VkCompareOp                                 compareOp);
```
or the equivalent command
```c
// Provided by VK_EXT_extended_dynamic_state
void vkCmdSetStencilOpEXT(
    VkCommandBuffer                             commandBuffer,
    VkStencilFaceFlags                          faceMask,
    VkStencilOp                                 failOp,
    VkStencilOp                                 passOp,
    VkStencilOp                                 depthFailOp,
    VkCompareOp                                 compareOp);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`face_mask`] is a bitmask of [`StencilFaceFlagBits`] specifying the set of stencil state for which to update the stencil operation.
- [`fail_op`] is a [`StencilOp`] value specifying the action performed on samples that fail the stencil test.
- [`pass_op`] is a [`StencilOp`] value specifying the action performed on samples that pass both the depth and stencil tests.
- [`depth_fail_op`] is a [`StencilOp`] value specifying the action performed on samples that pass the stencil test and fail the depth test.
- [`compare_op`] is a [`CompareOp`] value specifying the comparison operator used in the stencil test.

# Description
This command sets the stencil operation for subsequent drawing commands when
the graphics pipeline is created with `VK_DYNAMIC_STATE_STENCIL_OP` set
in [`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the corresponding
[`PipelineDepthStencilStateCreateInfo`]::[`fail_op`], [`pass_op`],
[`depth_fail_op`], and [`compare_op`] values used to create the currently
active pipeline, for both front and back faces.
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`face_mask`] **must**  be a valid combination of [`StencilFaceFlagBits`] values
-  [`face_mask`] **must**  not be `0`
-  [`fail_op`] **must**  be a valid [`StencilOp`] value
-  [`pass_op`] **must**  be a valid [`StencilOp`] value
-  [`depth_fail_op`] **must**  be a valid [`StencilOp`] value
-  [`compare_op`] **must**  be a valid [`CompareOp`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_extended_dynamic_state`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`CompareOp`]
- [VkStencilFaceFlags]()
- [`StencilOp`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        