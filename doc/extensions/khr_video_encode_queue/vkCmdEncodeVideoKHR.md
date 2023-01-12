[vkCmdEncodeVideoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html) - Encode operation for bitstream generation

# C Specifications
To launch an encode operation that results in bitstream generation, call:
```c
// Provided by VK_KHR_video_encode_queue
void vkCmdEncodeVideoKHR(
    VkCommandBuffer                             commandBuffer,
    const VkVideoEncodeInfoKHR*                 pEncodeInfo);
```

# Parameters
- [`command_buffer`] is the command buffer to be filled with this function for encoding to generate a bitstream.
- [`p_encode_info`] is a pointer to a [`VideoEncodeInfoKHR`] structure.

# Description
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_encode_info`] **must**  be a valid pointer to a valid [`VideoEncodeInfoKHR`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support encode operations
-    This command  **must**  only be called outside of a render pass instance
-  [`command_buffer`] **must**  be a primary [`CommandBuffer`]

## Host Synchronization
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_video_encode_queue`]
- [`CommandBuffer`]
- [`VideoEncodeInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        