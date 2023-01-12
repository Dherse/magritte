[vkCmdEndRenderPass2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html) - End the current render pass

# C Specifications
To record a command to end a render pass instance after recording the
commands for the last subpass, call:
```c
// Provided by VK_VERSION_1_2
void vkCmdEndRenderPass2(
    VkCommandBuffer                             commandBuffer,
    const VkSubpassEndInfo*                     pSubpassEndInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_create_renderpass2
void vkCmdEndRenderPass2KHR(
    VkCommandBuffer                             commandBuffer,
    const VkSubpassEndInfo*                     pSubpassEndInfo);
```

# Parameters
- [`command_buffer`] is the command buffer in which to end the current render pass instance.
- [`p_subpass_end_info`] is a pointer to a [`SubpassEndInfo`] structure containing information about how the previous subpass will be ended.

# Description
[`cmd_end_render_pass2`] is semantically identical to
[`cmd_end_render_pass`], except that it is extensible.
## Valid Usage
-    The current subpass index  **must**  be equal to the number of subpasses in the render pass minus one
-    This command  **must**  not be recorded when transform feedback is active
-    The current render pass instance  **must**  not have been begun with [`cmd_begin_rendering`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_subpass_end_info`] **must**  be a valid pointer to a valid [`SubpassEndInfo`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called inside of a render pass instance
-  [`command_buffer`] **must**  be a primary [`CommandBuffer`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_create_renderpass2`]
- [`crate::vulkan1_2`]
- [`CommandBuffer`]
- [`SubpassEndInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        