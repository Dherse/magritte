[VkDependencyInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyInfo.html) - Structure specifying dependency information for a synchronization command

# C Specifications
The [`DependencyInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkDependencyInfo {
    VkStructureType                  sType;
    const void*                      pNext;
    VkDependencyFlags                dependencyFlags;
    uint32_t                         memoryBarrierCount;
    const VkMemoryBarrier2*          pMemoryBarriers;
    uint32_t                         bufferMemoryBarrierCount;
    const VkBufferMemoryBarrier2*    pBufferMemoryBarriers;
    uint32_t                         imageMemoryBarrierCount;
    const VkImageMemoryBarrier2*     pImageMemoryBarriers;
} VkDependencyInfo;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkDependencyInfo VkDependencyInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`dependency_flags`] is a bitmask of [`DependencyFlagBits`] specifying how execution and memory dependencies are formed.
- [`memory_barrier_count`] is the length of the [`memory_barriers`] array.
- [`memory_barriers`] is a pointer to an array of [`MemoryBarrier2`] structures defining memory dependencies between any memory accesses.
- [`buffer_memory_barrier_count`] is the length of the [`buffer_memory_barriers`] array.
- [`buffer_memory_barriers`] is a pointer to an array of [`BufferMemoryBarrier2`] structures defining memory dependencies between buffer ranges.
- [`image_memory_barrier_count`] is the length of the [`image_memory_barriers`] array.
- [`image_memory_barriers`] is a pointer to an array of [`ImageMemoryBarrier2`] structures defining memory dependencies between image subresources.

# Description
This structure defines a set of [memory dependencies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory), as well as [queue
family transfer operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) and [image layout transitions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).Each member of [`memory_barriers`], [`buffer_memory_barriers`], and
[`image_memory_barriers`] defines a separate
[memory dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory).
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEPENDENCY_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`dependency_flags`] **must**  be a valid combination of [`DependencyFlagBits`] values
-    If [`memory_barrier_count`] is not `0`, [`memory_barriers`] **must**  be a valid pointer to an array of [`memory_barrier_count`] valid [`MemoryBarrier2`] structures
-    If [`buffer_memory_barrier_count`] is not `0`, [`buffer_memory_barriers`] **must**  be a valid pointer to an array of [`buffer_memory_barrier_count`] valid [`BufferMemoryBarrier2`] structures
-    If [`image_memory_barrier_count`] is not `0`, [`image_memory_barriers`] **must**  be a valid pointer to an array of [`image_memory_barrier_count`] valid [`ImageMemoryBarrier2`] structures

# Related
- [`khr_synchronization2`]
- [`crate::vulkan1_3`]
- [`BufferMemoryBarrier2`]
- [VkDependencyFlags]()
- [`ImageMemoryBarrier2`]
- [`MemoryBarrier2`]
- [`StructureType`]
- [`cmd_pipeline_barrier2`]
- [`cmd_pipeline_barrier2_khr`]
- [`cmd_set_event2`]
- [`cmd_set_event2_khr`]
- [`cmd_wait_events2`]
- [`cmd_wait_events2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        