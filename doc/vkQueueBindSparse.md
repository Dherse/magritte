[vkQueueBindSparse](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueBindSparse.html) - Bind device memory to a sparse resource object

# C Specifications
To submit sparse binding operations to a queue, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkQueueBindSparse(
    VkQueue                                     queue,
    uint32_t                                    bindInfoCount,
    const VkBindSparseInfo*                     pBindInfo,
    VkFence                                     fence);
```

# Parameters
- [`queue`] is the queue that the sparse binding operations will be submitted to.
- [`bind_info_count`] is the number of elements in the [`p_bind_info`] array.
- [`p_bind_info`] is a pointer to an array of [`BindSparseInfo`] structures, each specifying a sparse binding submission batch.
- [`fence`] is an  **optional**  handle to a fence to be signaled. If [`fence`] is not [`crate::Handle::null`], it defines a [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling).

# Description
[`queue_bind_sparse`] is a [queue submission
command](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-submission), with each batch defined by an element of [`p_bind_info`] as a
[`BindSparseInfo`] structure.
Batches begin execution in the order they appear in [`p_bind_info`], but
 **may**  complete out of order.Within a batch, a given range of a resource  **must**  not be bound more than
once.
Across batches, if a range is to be bound to one allocation and offset and
then to another allocation and offset, then the application  **must**  guarantee
(usually using semaphores) that the binding operations are executed in the
correct order, as well as to order binding operations against the execution
of command buffer submissions.As no operation to [`queue_bind_sparse`] causes any pipeline stage to
access memory, synchronization primitives used in this command effectively
only define execution dependencies.Additional information about fence and semaphore operation is described in
[the synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization).
## Valid Usage
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be unsignaled
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  not be associated with any other queue command that has not yet completed execution on that queue
-    Each element of the `pSignalSemaphores` member of each element of [`p_bind_info`] **must**  be unsignaled when the semaphore signal operation it defines is executed on the device
-    When a semaphore wait operation referring to a binary semaphore defined by any element of the `pWaitSemaphores` member of any element of [`p_bind_info`] executes on [`queue`], there  **must**  be no other queues waiting on the same semaphore
-    All elements of the `pWaitSemaphores` member of all elements of the [`p_bind_info`] parameter referring to a binary semaphore  **must**  be semaphores that are signaled, or have [semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) previously submitted for execution
-    All elements of the `pWaitSemaphores` member of all elements of [`p_bind_info`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY` **must**  reference a semaphore signal operation that has been submitted for execution and any semaphore signal operations on which it depends (if any)  **must**  have also been submitted for execution

## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle
-    If [`bind_info_count`] is not `0`, [`p_bind_info`] **must**  be a valid pointer to an array of [`bind_info_count`] valid [`BindSparseInfo`] structures
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be a valid [`Fence`] handle
-    The [`queue`] **must**  support sparse binding operations
-    Both of [`fence`], and [`queue`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`queue`] **must**  be externally synchronized
- Host access to [`fence`] **must**  be externally synchronized

## Command Properties
## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`BindSparseInfo`]
- [`Fence`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        