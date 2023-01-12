[vkCmdCopyBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyBuffer.html) - Copy data between buffer regions

# C Specifications
To copy data between buffer objects, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdCopyBuffer(
    VkCommandBuffer                             commandBuffer,
    VkBuffer                                    srcBuffer,
    VkBuffer                                    dstBuffer,
    uint32_t                                    regionCount,
    const VkBufferCopy*                         pRegions);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`src_buffer`] is the source buffer.
- [`dst_buffer`] is the destination buffer.
- [`region_count`] is the number of regions to copy.
- [`p_regions`] is a pointer to an array of [`BufferCopy`] structures specifying the regions to copy.

# Description
Each region in [`p_regions`] is copied from the source buffer to the same
region of the destination buffer.
[`src_buffer`] and [`dst_buffer`] **can**  be the same buffer or alias the
same memory, but the resulting values are undefined if the copy regions
overlap in memory.
## Valid Usage
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`src_buffer`] **must**  not be a protected buffer
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, [`dst_buffer`] **must**  not be a protected buffer
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`]() is not supported, [`dst_buffer`] **must**  not be an unprotected buffer

-    The `srcOffset` member of each element of [`p_regions`] **must**  be less than the size of [`src_buffer`]
-    The `dstOffset` member of each element of [`p_regions`] **must**  be less than the size of [`dst_buffer`]
-    The `size` member of each element of [`p_regions`] **must**  be less than or equal to the size of [`src_buffer`] minus `srcOffset`
-    The `size` member of each element of [`p_regions`] **must**  be less than or equal to the size of [`dst_buffer`] minus `dstOffset`
-    The union of the source regions, and the union of the destination regions, specified by the elements of [`p_regions`],  **must**  not overlap in memory
-  [`src_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` usage flag
-    If [`src_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`dst_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`src_buffer`] **must**  be a valid [`Buffer`] handle
-  [`dst_buffer`] **must**  be a valid [`Buffer`] handle
-  [`p_regions`] **must**  be a valid pointer to an array of [`region_count`] valid [`BufferCopy`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer, graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance
-  [`region_count`] **must**  be greater than `0`
-    Each of [`command_buffer`], [`dst_buffer`], and [`src_buffer`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`BufferCopy`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        