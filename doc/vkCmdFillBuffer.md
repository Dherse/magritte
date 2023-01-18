[vkCmdFillBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdFillBuffer.html) - Fill a region of a buffer with a fixed value

# C Specifications
To clear buffer data, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdFillBuffer(
    VkCommandBuffer                             commandBuffer,
    VkBuffer                                    dstBuffer,
    VkDeviceSize                                dstOffset,
    VkDeviceSize                                size,
    uint32_t                                    data);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`dst_buffer`] is the buffer to be filled.
- [`dst_offset`] is the byte offset into the buffer at which to start filling, and  **must**  be a multiple of 4.
- [`size`] is the number of bytes to fill, and  **must**  be either a multiple of 4, or [`WHOLE_SIZE`] to fill the range from `offset` to the end of the buffer. If [`WHOLE_SIZE`] is used and the remaining size of the buffer is not a multiple of 4, then the nearest smaller multiple is used.
- [`data`] is the 4-byte word written repeatedly to the buffer to fill [`size`] bytes of data. The data word is written to memory according to the host endianness.

# Description
[`cmd_fill_buffer`] is treated as a “transfer” operation for the
purposes of synchronization barriers.
The `VK_BUFFER_USAGE_TRANSFER_DST_BIT` **must**  be specified in `usage`
of [`BufferCreateInfo`] in order for the buffer to be compatible with
[`cmd_fill_buffer`].
## Valid Usage
-  [`dst_offset`] **must**  be less than the size of [`dst_buffer`]
-  [`dst_offset`] **must**  be a multiple of `4`
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  be greater than `0`
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  be less than or equal to the size of [`dst_buffer`] minus [`dst_offset`]
-    If [`size`] is not equal to [`WHOLE_SIZE`], [`size`] **must**  be a multiple of `4`
-  [`dst_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, [`dst_buffer`] **must**  not be a protected buffer
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, [`dst_buffer`] **must**  not be an unprotected buffer

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`dst_buffer`] **must**  be a valid [`Buffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics or compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Both of [`command_buffer`], and [`dst_buffer`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        