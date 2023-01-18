[vkCmdEndVideoCodingKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html) - End decode jobs

# C Specifications
To end video decode or encode operations, call:
```c
// Provided by VK_KHR_video_queue
void vkCmdEndVideoCodingKHR(
    VkCommandBuffer                             commandBuffer,
    const VkVideoEndCodingInfoKHR*              pEndCodingInfo);
```

# Parameters
- [`command_buffer`] is the command buffer to be filled by this function.
- [`p_end_coding_info`] is a pointer to a [`VideoEndCodingInfoKHR`] structure.

# Description
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_end_coding_info`] **must**  be a valid pointer to a valid [`VideoEndCodingInfoKHR`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support decode, or encode operations
-    This command  **must**  only be called outside of a render pass instance
-  [`command_buffer`] **must**  be a primary [`CommandBuffer`]

## Host Synchronization
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_video_queue`]
- [`CommandBuffer`]
- [`VideoEndCodingInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        