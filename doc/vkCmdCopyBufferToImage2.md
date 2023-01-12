[vkCmdCopyBufferToImage2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBufferToImage2.html) - Copy data from a buffer into an image

# C Specifications
To copy data from a buffer object to an image object, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdCopyBufferToImage2(
    VkCommandBuffer                             commandBuffer,
    const VkCopyBufferToImageInfo2*             pCopyBufferToImageInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_copy_commands2
void vkCmdCopyBufferToImage2KHR(
    VkCommandBuffer                             commandBuffer,
    const VkCopyBufferToImageInfo2*             pCopyBufferToImageInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`p_copy_buffer_to_image_info`] is a pointer to a [`CopyBufferToImageInfo2`] structure describing the copy parameters.

# Description
This command is functionally identical to [`cmd_copy_buffer_to_image`], but
includes extensible sub-structures that include `sType` and `pNext`
parameters, allowing them to be more easily extended.
## Valid Usage
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, `srcBuffer` **must**  not be a protected buffer
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, `dstImage` **must**  not be a protected image
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not supported, `dstImage` **must**  not be an unprotected image

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_copy_buffer_to_image_info`] **must**  be a valid pointer to a valid [`CopyBufferToImageInfo2`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_copy_commands2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`CopyBufferToImageInfo2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        