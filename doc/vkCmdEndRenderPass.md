[vkCmdEndRenderPass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass.html) - End the current render pass

# C Specifications
To record a command to end a render pass instance after recording the
commands for the last subpass, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdEndRenderPass(
    VkCommandBuffer                             commandBuffer);
```

# Parameters
- [`command_buffer`] is the command buffer in which to end the current render pass instance.

# Description
Ending a render pass instance performs any multisample resolve operations on
the final subpass.
## Valid Usage
-    The current subpass index  **must**  be equal to the number of subpasses in the render pass minus one
-    This command  **must**  not be recorded when transform feedback is active
-    The current render pass instance  **must**  not have been begun with [`cmd_begin_rendering`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called inside of a render pass instance
-  [`command_buffer`] **must**  be a primary [`CommandBuffer`]

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
        