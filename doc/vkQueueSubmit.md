[vkQueueSubmit](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit.html) - Submits a sequence of semaphores or command buffers to a queue

# C Specifications
To submit command buffers to a queue, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkQueueSubmit(
    VkQueue                                     queue,
    uint32_t                                    submitCount,
    const VkSubmitInfo*                         pSubmits,
    VkFence                                     fence);
```

# Parameters
- [`queue`] is the queue that the command buffers will be submitted to.
- [`submit_count`] is the number of elements in the [`p_submits`] array.
- [`p_submits`] is a pointer to an array of [`SubmitInfo`] structures, each specifying a command buffer submission batch.
- [`fence`] is an  **optional**  handle to a fence to be signaled once all submitted command buffers have completed execution. If [`fence`] is not [`crate::Handle::null`], it defines a [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling).

# Description
[`queue_submit`] is a [queue submission
command](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-submission), with each batch defined by an element of [`p_submits`].
Batches begin execution in the order they appear in [`p_submits`], but  **may** 
complete out of order.Fence and semaphore operations submitted with [`queue_submit`] have
additional ordering constraints compared to other submission commands, with
dependencies involving previous and subsequent queue operations.
Information about these additional constraints can be found in the
[semaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores) and [fence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences) sections of [the synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization).Details on the interaction of `pWaitDstStageMask` with synchronization
are described in the [semaphore wait
operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting) section of [the synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization).The order that batches appear in [`p_submits`] is used to determine
[submission order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-submission-order), and thus all the
[implicit ordering guarantees](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-implicit) that respect it.
Other than these implicit ordering guarantees and any [explicit synchronization primitives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization), these batches  **may**  overlap or
otherwise execute out of order.If any command buffer submitted to this queue is in the
[executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle), it is moved to the
[pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
Once execution of all submissions of a command buffer complete, it moves
from the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle), back to the
[executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
If a command buffer was recorded with the
`VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT` flag, it instead moves to
the [invalid state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).If [`queue_submit`] fails, it  **may**  return
`VK_ERROR_OUT_OF_HOST_MEMORY` or `VK_ERROR_OUT_OF_DEVICE_MEMORY`.
If it does, the implementation  **must**  ensure that the state and contents of
any resources or synchronization primitives referenced by the submitted
command buffers and any semaphores referenced by [`p_submits`] is
unaffected by the call or its failure.
If [`queue_submit`] fails in such a way that the implementation is unable
to make that guarantee, the implementation  **must**  return
`VK_ERROR_DEVICE_LOST`.
See [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device).
## Valid Usage
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be unsignaled
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  not be associated with any other queue command that has not yet completed execution on that queue
-    Any calls to [`cmd_set_event`], [`cmd_reset_event`] or [`cmd_wait_events`] that have been recorded into any of the command buffer elements of the `pCommandBuffers` member of any element of [`p_submits`],  **must**  not reference any [`Event`] that is referenced by any of those commands in a command buffer that has been submitted to another queue and is still in the *pending state*
-    Any stage flag included in any element of the `pWaitDstStageMask` member of any element of [`p_submits`] **must**  be a pipeline stage supported by one of the capabilities of [`queue`], as specified in the [table of supported pipeline stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-supported)
-    Each binary semaphore element of the `pSignalSemaphores` member of any element of [`p_submits`] **must**  be unsignaled when the semaphore signal operation it defines is executed on the device
-    When a semaphore wait operation referring to a binary semaphore defined by any element of the `pWaitSemaphores` member of any element of [`p_submits`] executes on [`queue`], there  **must**  be no other queues waiting on the same semaphore
-    All elements of the `pWaitSemaphores` member of all elements of [`p_submits`] created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY` **must**  reference a semaphore signal operation that has been submitted for execution and any semaphore signal operations on which it depends (if any)  **must**  have also been submitted for execution
-    Each element of the `pCommandBuffers` member of each element of [`p_submits`] **must**  be in the [pending or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    If any element of the `pCommandBuffers` member of any element of [`p_submits`] was not recorded with the `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT`, it  **must**  not be in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    Any [secondary command buffers recorded](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-secondary) into any element of the `pCommandBuffers` member of any element of [`p_submits`] **must**  be in the [pending or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    If any [secondary command buffers recorded](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-secondary) into any element of the `pCommandBuffers` member of any element of [`p_submits`] was not recorded with the `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT`, it  **must**  not be in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    Each element of the `pCommandBuffers` member of each element of [`p_submits`] **must**  have been allocated from a [`CommandPool`] that was created for the same queue family [`queue`] belongs to
-    If any element of `pSubmits->pCommandBuffers` includes a [Queue Family Transfer Acquire Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire), there  **must**  exist a previously submitted [Queue Family Transfer Release Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release) on a queue in the queue family identified by the acquire operation, with parameters matching the acquire operation as defined in the definition of such [acquire operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire), and which happens-before the acquire operation
-    If a command recorded into any element of `pCommandBuffers` was a [`cmd_begin_query`] whose `queryPool` was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the [profiling lock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#profiling-lock) **must**  have been held continuously on the [`Device`] that [`queue`] was retrieved from, throughout recording of those command buffers
-    Any resource created with `VK_SHARING_MODE_EXCLUSIVE` that is read by an operation specified by [`p_submits`] **must**  not be owned by any queue family other than the one which [`queue`] belongs to, at the time it is executed
-    Any resource created with `VK_SHARING_MODE_CONCURRENT` that is accessed by an operation specified by [`p_submits`] **must**  have included the queue family of [`queue`] at resource creation time
-    If [`queue`] was not created with `VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT`, there  **must**  be no element of [`p_submits`] that includes an [`ProtectedSubmitInfo`] structure in its `pNext` chain with `protectedSubmit` equal to [`TRUE`]

## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle
-    If [`submit_count`] is not `0`, [`p_submits`] **must**  be a valid pointer to an array of [`submit_count`] valid [`SubmitInfo`] structures
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be a valid [`Fence`] handle
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
- [`Fence`]
- [`Queue`]
- [`SubmitInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        