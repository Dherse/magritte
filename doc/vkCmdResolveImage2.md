[vkCmdResolveImage2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResolveImage2.html) - Resolve regions of an image

# C Specifications
To resolve a multisample image to a non-multisample image, call:
```c
// Provided by VK_VERSION_1_3
void vkCmdResolveImage2(
    VkCommandBuffer                             commandBuffer,
    const VkResolveImageInfo2*                  pResolveImageInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_copy_commands2
void vkCmdResolveImage2KHR(
    VkCommandBuffer                             commandBuffer,
    const VkResolveImageInfo2*                  pResolveImageInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`p_resolve_image_info`] is a pointer to a [`ResolveImageInfo2`] structure describing the resolve parameters.

# Description
This command is functionally identical to [`cmd_resolve_image`], but
includes extensible sub-structures that include `sType` and `pNext`
parameters, allowing them to be more easily extended.
## Valid Usage
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, `srcImage` **must**  not be a protected image
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, `dstImage` **must**  not be a protected image
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not supported, `dstImage` **must**  not be an unprotected image

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_resolve_image_info`] **must**  be a valid pointer to a valid [`ResolveImageInfo2`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called outside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_copy_commands2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`ResolveImageInfo2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        