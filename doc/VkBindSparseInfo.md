[VkBindSparseInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindSparseInfo.html) - Structure specifying a sparse binding operation

# C Specifications
The [`BindSparseInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkBindSparseInfo {
    VkStructureType                             sType;
    const void*                                 pNext;
    uint32_t                                    waitSemaphoreCount;
    const VkSemaphore*                          pWaitSemaphores;
    uint32_t                                    bufferBindCount;
    const VkSparseBufferMemoryBindInfo*         pBufferBinds;
    uint32_t                                    imageOpaqueBindCount;
    const VkSparseImageOpaqueMemoryBindInfo*    pImageOpaqueBinds;
    uint32_t                                    imageBindCount;
    const VkSparseImageMemoryBindInfo*          pImageBinds;
    uint32_t                                    signalSemaphoreCount;
    const VkSemaphore*                          pSignalSemaphores;
} VkBindSparseInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`wait_semaphore_count`] is the number of semaphores upon which to wait before executing the sparse binding operations for the batch.
- [`wait_semaphores`] is a pointer to an array of semaphores upon which to wait on before the sparse binding operations for this batch begin execution. If semaphores to wait on are provided, they define a [semaphore wait operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting).
- [`buffer_bind_count`] is the number of sparse buffer bindings to perform in the batch.
- [`buffer_binds`] is a pointer to an array of [`SparseBufferMemoryBindInfo`] structures.
- [`image_opaque_bind_count`] is the number of opaque sparse image bindings to perform.
- [`image_opaque_binds`] is a pointer to an array of [`SparseImageOpaqueMemoryBindInfo`] structures, indicating opaque sparse image bindings to perform.
- [`image_bind_count`] is the number of sparse image bindings to perform.
- [`image_binds`] is a pointer to an array of [`SparseImageMemoryBindInfo`] structures, indicating sparse image bindings to perform.
- [`signal_semaphore_count`] is the number of semaphores to be signaled once the sparse binding operations specified by the structure have completed execution.
- [`signal_semaphores`] is a pointer to an array of semaphores which will be signaled when the sparse binding operations for this batch have completed execution. If semaphores to be signaled are provided, they define a [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling).

# Description
## Valid Usage
-    If any element of [`wait_semaphores`] or [`signal_semaphores`] was created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` then the [`p_next`] chain  **must**  include a [`TimelineSemaphoreSubmitInfo`] structure
-    If the [`p_next`] chain of this structure includes a [`TimelineSemaphoreSubmitInfo`] structure and any element of [`wait_semaphores`] was created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` then its `waitSemaphoreValueCount` member  **must**  equal [`wait_semaphore_count`]
-    If the [`p_next`] chain of this structure includes a [`TimelineSemaphoreSubmitInfo`] structure and any element of [`signal_semaphores`] was created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` then its `signalSemaphoreValueCount` member  **must**  equal [`signal_semaphore_count`]
-    For each element of [`signal_semaphores`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of [`TimelineSemaphoreSubmitInfo::signal_semaphore_values`] **must**  have a value greater than the current value of the semaphore when the [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) is executed
-    For each element of [`wait_semaphores`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of [`TimelineSemaphoreSubmitInfo::wait_semaphore_values`] **must**  have a value which does not differ from the current value of the semaphore or from the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
-    For each element of [`signal_semaphores`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` the corresponding element of [`TimelineSemaphoreSubmitInfo::signal_semaphore_values`] **must**  have a value which does not differ from the current value of the semaphore or from the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_SPARSE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DeviceGroupBindSparseInfo`] or [`TimelineSemaphoreSubmitInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-    If [`wait_semaphore_count`] is not `0`, [`wait_semaphores`] **must**  be a valid pointer to an array of [`wait_semaphore_count`] valid [`Semaphore`] handles
-    If [`buffer_bind_count`] is not `0`, [`buffer_binds`] **must**  be a valid pointer to an array of [`buffer_bind_count`] valid [`SparseBufferMemoryBindInfo`] structures
-    If [`image_opaque_bind_count`] is not `0`, [`image_opaque_binds`] **must**  be a valid pointer to an array of [`image_opaque_bind_count`] valid [`SparseImageOpaqueMemoryBindInfo`] structures
-    If [`image_bind_count`] is not `0`, [`image_binds`] **must**  be a valid pointer to an array of [`image_bind_count`] valid [`SparseImageMemoryBindInfo`] structures
-    If [`signal_semaphore_count`] is not `0`, [`signal_semaphores`] **must**  be a valid pointer to an array of [`signal_semaphore_count`] valid [`Semaphore`] handles
-    Both of the elements of [`signal_semaphores`], and the elements of [`wait_semaphores`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`Semaphore`]
- [`SparseBufferMemoryBindInfo`]
- [`SparseImageMemoryBindInfo`]
- [`SparseImageOpaqueMemoryBindInfo`]
- [`StructureType`]
- [`queue_bind_sparse`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        