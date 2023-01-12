[vkCmdSetStencilReference](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilReference.html) - Set stencil reference value dynamically for a command buffer

# C Specifications
To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the stencil reference value,
call:
```c
// Provided by VK_VERSION_1_0
void vkCmdSetStencilReference(
    VkCommandBuffer                             commandBuffer,
    VkStencilFaceFlags                          faceMask,
    uint32_t                                    reference);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`face_mask`] is a bitmask of [`StencilFaceFlagBits`] specifying the set of stencil state for which to update the reference value, as described above for [`cmd_set_stencil_compare_mask`].
- [`reference`] is the new value to use as the stencil reference value.

# Description
This command sets the stencil reference value for subsequent drawing
commands when the graphics pipeline is created with
`VK_DYNAMIC_STATE_STENCIL_REFERENCE` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
Otherwise, this state is specified by the
[`PipelineDepthStencilStateCreateInfo`]::[`reference`] value used to
create the currently active pipeline, for both front and back faces.
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`face_mask`] **must**  be a valid combination of [`StencilFaceFlagBits`] values
-  [`face_mask`] **must**  not be `0`
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [VkStencilFaceFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        