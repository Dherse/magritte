[vkCmdBindIndexBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer.html) - Bind an index buffer to a command buffer

# C Specifications
To bind an index buffer to a command buffer, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdBindIndexBuffer(
    VkCommandBuffer                             commandBuffer,
    VkBuffer                                    buffer,
    VkDeviceSize                                offset,
    VkIndexType                                 indexType);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`buffer`] is the buffer being bound.
- [`offset`] is the starting offset in bytes within [`buffer`] used in index buffer address calculations.
- [`index_type`] is a [`IndexType`] value specifying the size of the indices.

# Description
## Valid Usage
-  [`offset`] **must**  be less than the size of [`buffer`]
-    The sum of [`offset`] and the address of the range of [`DeviceMemory`] object that is backing [`buffer`],  **must**  be a multiple of the type indicated by [`index_type`]
-  [`buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDEX_BUFFER_BIT` flag
-    If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`index_type`] **must**  not be `VK_INDEX_TYPE_NONE_KHR`
-    If [`index_type`] is `VK_INDEX_TYPE_UINT8_EXT`, the [indexTypeUint8](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-indexTypeUint8) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`index_type`] **must**  be a valid [`IndexType`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    Both of [`buffer`], and [`command_buffer`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]
- [`IndexType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        