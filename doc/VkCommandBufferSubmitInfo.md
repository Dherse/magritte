[VkCommandBufferSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfo.html) - Structure specifying a command buffer submission

# C Specifications
The [`CommandBufferSubmitInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkCommandBufferSubmitInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkCommandBuffer    commandBuffer;
    uint32_t           deviceMask;
} VkCommandBufferSubmitInfo;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkCommandBufferSubmitInfo VkCommandBufferSubmitInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`command_buffer`] is a [`CommandBuffer`] to be submitted for execution.
- [`device_mask`] is a bitmask indicating which devices in a device group execute the command buffer. A [`device_mask`] of `0` is equivalent to setting all bits corresponding to valid devices in the group to `1`.

# Description
## Valid Usage
-  [`command_buffer`] **must**  not have been allocated with `VK_COMMAND_BUFFER_LEVEL_SECONDARY`
-    If [`device_mask`] is not `0`, it  **must**  be a valid device mask

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]
- [`CommandBuffer`]
- [`StructureType`]
- [`SubmitInfo2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        