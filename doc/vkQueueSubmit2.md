[vkQueueSubmit2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSubmit2.html) - Submits command buffers to a queue

# C Specifications
To submit command buffers to a queue, call:
```c
// Provided by VK_VERSION_1_3
VkResult vkQueueSubmit2(
    VkQueue                                     queue,
    uint32_t                                    submitCount,
    const VkSubmitInfo2*                        pSubmits,
    VkFence                                     fence);
```
or the equivalent command
```c
// Provided by VK_KHR_synchronization2
VkResult vkQueueSubmit2KHR(
    VkQueue                                     queue,
    uint32_t                                    submitCount,
    const VkSubmitInfo2*                        pSubmits,
    VkFence                                     fence);
```

# Parameters
- [`queue`] is the queue that the command buffers will be submitted to.
- [`submit_count`] is the number of elements in the [`p_submits`] array.
- [`p_submits`] is a pointer to an array of [`SubmitInfo2`] structures, each specifying a command buffer submission batch.
- [`fence`] is an  **optional**  handle to a fence to be signaled once all submitted command buffers have completed execution. If [`fence`] is not [`crate::Handle::null`], it defines a [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling).

# Description
[`queue_submit2`] is a [queue submission
command](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-submission), with each batch defined by an element of [`p_submits`].Semaphore operations submitted with [`queue_submit2`] have additional
ordering constraints compared to other submission commands, with
dependencies involving previous and subsequent queue operations.
Information about these additional constraints can be found in the
[semaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores) section of [the
synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization).If any command buffer submitted to this queue is in the
[executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle), it is moved to the
[pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
Once execution of all submissions of a command buffer complete, it moves
from the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle), back to the
[executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
If a command buffer was recorded with the
`VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT` flag, it instead moves
back to the [invalid state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).If [`queue_submit2`] fails, it  **may**  return
`VK_ERROR_OUT_OF_HOST_MEMORY` or `VK_ERROR_OUT_OF_DEVICE_MEMORY`.
If it does, the implementation  **must**  ensure that the state and contents of
any resources or synchronization primitives referenced by the submitted
command buffers and any semaphores referenced by [`p_submits`] is
unaffected by the call or its failure.
If [`queue_submit2`] fails in such a way that the implementation is
unable to make that guarantee, the implementation  **must**  return
`VK_ERROR_DEVICE_LOST`.
See [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device).
## Valid Usage
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be unsignaled
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  not be associated with any other queue command that has not yet completed execution on that queue
-    The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2) feature  **must**  be enabled
-    If a command recorded into the `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] referenced an [`Event`], that event  **must**  not be referenced by a command that has been submitted to another queue and is still in the *pending state*
-    The `semaphore` member of any binary semaphore element of the `pSignalSemaphoreInfos` member of any element of [`p_submits`] **must**  be unsignaled when the semaphore signal operation it defines is executed on the device
-    The `stageMask` member of any element of the `pSignalSemaphoreInfos` member of any element of [`p_submits`] **must**  only include pipeline stages that are supported by the queue family which [`queue`] belongs to
-    The `stageMask` member of any element of the `pWaitSemaphoreInfos` member of any element of [`p_submits`] **must**  only include pipeline stages that are supported by the queue family which [`queue`] belongs to
-    When a semaphore wait operation for a binary semaphore is executed, as defined by the `semaphore` member of any element of the `pWaitSemaphoreInfos` member of any element of [`p_submits`], there  **must**  be no other queues waiting on the same semaphore
-    The `semaphore` member of any element of the `pWaitSemaphoreInfos` member of any element of [`p_submits`] **must**  be semaphores that are signaled, or have [semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) previously submitted for execution
-    Any `semaphore` member of any element of the `pWaitSemaphoreInfos` member of any element of [`p_submits`] that was created with a [`SemaphoreTypeKHR`] of `VK_SEMAPHORE_TYPE_BINARY_KHR` **must**  reference a semaphore signal operation that has been submitted for execution and any semaphore signal operations on which it depends (if any)  **must**  have also been submitted for execution
-    The `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] **must**  be in the [pending or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    If a command recorded into the `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] was not recorded with the `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT`, it  **must**  not be in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    Any [secondary command buffers recorded](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-secondary) into the `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] **must**  be in the [pending or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    If any [secondary command buffers recorded](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-secondary) into the `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] was not recorded with the `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT`, it  **must**  not be in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    The `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] **must**  have been allocated from a [`CommandPool`] that was created for the same queue family [`queue`] belongs to
-    If a command recorded into the `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] includes a [Queue Family Transfer Acquire Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire), there  **must**  exist a previously submitted [Queue Family Transfer Release Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release) on a queue in the queue family identified by the acquire operation, with parameters matching the acquire operation as defined in the definition of such [acquire operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire), and which happens before the acquire operation
-    If a command recorded into the `commandBuffer` member of any element of the `pCommandBufferInfos` member of any element of [`p_submits`] was a [`cmd_begin_query`] whose `queryPool` was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the [profiling lock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#profiling-lock) **must**  have been held continuously on the [`Device`] that [`queue`] was retrieved from, throughout recording of those command buffers
-    If [`queue`] was not created with `VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT`, the `flags` member of any element of [`p_submits`] **must**  not include `VK_SUBMIT_PROTECTED_BIT_KHR`

## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle
-    If [`submit_count`] is not `0`, [`p_submits`] **must**  be a valid pointer to an array of [`submit_count`] valid [`SubmitInfo2`] structures
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
- [`khr_synchronization2`]
- [`crate::vulkan1_3`]
- [`Fence`]
- [`Queue`]
- [`SubmitInfo2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        