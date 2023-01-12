[vkDeferredOperationJoinKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html) - Assign a thread to a deferred operation

# C Specifications
To assign a thread to a deferred operation, call:
```c
// Provided by VK_KHR_deferred_host_operations
VkResult vkDeferredOperationJoinKHR(
    VkDevice                                    device,
    VkDeferredOperationKHR                      operation);
```

# Parameters
- [`device`] is the device which owns [`operation`].
- [`operation`] is the deferred operation that the calling thread should work on.

# Description
The [`deferred_operation_join_khr`] command will execute a portion of the
deferred operation on the calling thread.The return value will be one of the following:
- A return value of `VK_SUCCESS` indicates that [`operation`] is complete. The application  **should**  use [`get_deferred_operation_result_khr`] to retrieve the result of [`operation`].
- A return value of `VK_THREAD_DONE_KHR` indicates that the deferred operation is not complete, but there is no work remaining to assign to threads. Future calls to [`deferred_operation_join_khr`] are not necessary and will simply harm performance. This situation  **may**  occur when other threads executing [`deferred_operation_join_khr`] are about to complete [`operation`], and the implementation is unable to partition the workload any further.
- A return value of `VK_THREAD_IDLE_KHR` indicates that the deferred operation is not complete, and there is no work for the thread to do at the time of the call. This situation  **may**  occur if the operation encounters a temporary reduction in parallelism. By returning `VK_THREAD_IDLE_KHR`, the implementation is signaling that it expects that more opportunities for parallelism will emerge as execution progresses, and that future calls to [`deferred_operation_join_khr`] **can**  be beneficial. In the meantime, the application  **can**  perform other work on the calling thread.
Implementations  **must**  guarantee forward progress by enforcing the following
invariants:
0. If only one thread has invoked [`deferred_operation_join_khr`] on a given operation, that thread  **must**  execute the operation to completion and return `VK_SUCCESS`.
1. If multiple threads have concurrently invoked [`deferred_operation_join_khr`] on the same operation, then at least one of them  **must**  complete the operation and return `VK_SUCCESS`.

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
-  [`operation`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_THREAD_DONE_KHR`  - `VK_THREAD_IDLE_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_deferred_host_operations`]
- [`DeferredOperationKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        