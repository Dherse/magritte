[VkCommandBufferAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferAllocateInfo.html) - Structure specifying the allocation parameters for command buffer object

# C Specifications
The [`CommandBufferAllocateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkCommandBufferAllocateInfo {
    VkStructureType         sType;
    const void*             pNext;
    VkCommandPool           commandPool;
    VkCommandBufferLevel    level;
    uint32_t                commandBufferCount;
} VkCommandBufferAllocateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`command_pool`] is the command pool from which the command buffers are allocated.
- [`level`] is a [`CommandBufferLevel`] value specifying the command buffer level.
- [`command_buffer_count`] is the number of command buffers to allocate from the pool.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`command_pool`] **must**  be a valid [`CommandPool`] handle
-  [`level`] **must**  be a valid [`CommandBufferLevel`] value

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferLevel`]
- [`CommandPool`]
- [`StructureType`]
- [`allocate_command_buffers`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        