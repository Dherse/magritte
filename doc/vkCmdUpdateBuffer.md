[vkCmdUpdateBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdUpdateBuffer.html) - Update a buffer's contents from host memory

# C Specifications
To update buffer data inline in a command buffer, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdUpdateBuffer(
    VkCommandBuffer                             commandBuffer,
    VkBuffer                                    dstBuffer,
    VkDeviceSize                                dstOffset,
    VkDeviceSize                                dataSize,
    const void*                                 pData);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`dst_buffer`] is a handle to the buffer to be updated.
- [`dst_offset`] is the byte offset into the buffer to start updating, and  **must**  be a multiple of 4.
- [`data_size`] is the number of bytes to update, and  **must**  be a multiple of 4.
- [`p_data`] is a pointer to the source data for the buffer update, and  **must**  be at least [`data_size`] bytes in size.

# Description
[`data_size`] **must**  be less than or equal to 65536 bytes.
For larger updates, applications  **can**  use buffer to buffer
[copies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-buffers).The source data is copied from the user pointer to the command buffer when
the command is called.[`cmd_update_buffer`] is only allowed outside of a render pass.
This command is treated as a “transfer” operation for the purposes of
synchronization barriers.
The `VK_BUFFER_USAGE_TRANSFER_DST_BIT` **must**  be specified in `usage`
of [`BufferCreateInfo`] in order for the buffer to be compatible with
[`cmd_update_buffer`].
## Valid Usage
-  [`dst_offset`] **must**  be less than the size of [`dst_buffer`]
-  [`data_size`] **must**  be less than or equal to the size of [`dst_buffer`] minus [`dst_offset`]
-  [`dst_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_offset`] **must**  be a multiple of `4`
-  [`data_size`] **must**  be less than or equal to `65536`
-  [`data_size`] **must**  be a multiple of `4`
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, [`dst_buffer`] **must**  not be a protected buffer
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, [`dst_buffer`] **must**  not be an unprotected buffer

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`dst_buffer`] **must**  be a valid [`Buffer`] handle
-  [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance
-  [`data_size`] **must**  be greater than `0`
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
        