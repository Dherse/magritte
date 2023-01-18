[vkCmdBeginRendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRendering.html) - Begin a dynamic render pass instance

# C Specifications
To begin a render pass instance, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdBeginRendering(
    VkCommandBuffer                             commandBuffer,
    const VkRenderingInfo*                      pRenderingInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_dynamic_rendering
void vkCmdBeginRenderingKHR(
    VkCommandBuffer                             commandBuffer,
    const VkRenderingInfo*                      pRenderingInfo);
```

# Parameters
- [`command_buffer`] is the command buffer in which to record the command.
- [`p_rendering_info`] is a pointer to a [`RenderingInfo`] structure specifying details of the render pass instance to begin.

# Description
After beginning a render pass instance, the command buffer is ready to
record [draw commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing).If `pRenderingInfo->flags` includes `VK_RENDERING_RESUMING_BIT` then
this render pass is resumed from a render pass instance that has been
suspended earlier in [submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order).
## Valid Usage
-    The [`dynamicRendering`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-dynamicRendering) feature  **must**  be enabled
-    If [`command_buffer`] is a secondary command buffer, `pRenderingInfo->flags` **must**  not include `VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_rendering_info`] **must**  be a valid pointer to a valid [`RenderingInfo`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called outside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_dynamic_rendering`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`RenderingInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        