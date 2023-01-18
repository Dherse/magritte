[vkCmdEndRendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRendering.html) - End a dynamic render pass instance

# C Specifications
To end a render pass instance, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdEndRendering(
    VkCommandBuffer                             commandBuffer);
```
or the equivalent command
```c
// Provided by VK_KHR_dynamic_rendering
void vkCmdEndRenderingKHR(
    VkCommandBuffer                             commandBuffer);
```

# Parameters
- [`command_buffer`] is the command buffer in which to record the command.

# Description
If the value of `pRenderingInfo->flags` used to begin this render pass
instance included `VK_RENDERING_SUSPENDING_BIT`, then this render pass
is suspended and will be resumed later in
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
## Valid Usage
-    The current render pass instance  **must**  have been begun with [`cmd_begin_rendering`]
-    The current render pass instance  **must**  have been begun in [`command_buffer`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called inside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_dynamic_rendering`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        