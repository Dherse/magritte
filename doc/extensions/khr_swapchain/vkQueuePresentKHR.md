[vkQueuePresentKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html) - Queue an image for presentation

# C Specifications
After queueing all rendering commands and transitioning the image to the
correct layout, to queue an image for presentation, call:
```c
// Provided by VK_KHR_swapchain
VkResult vkQueuePresentKHR(
    VkQueue                                     queue,
    const VkPresentInfoKHR*                     pPresentInfo);
```

# Parameters
- [`queue`] is a queue that is capable of presentation to the target surface’s platform on the same device as the image’s swapchain.
- [`p_present_info`] is a pointer to a [`PresentInfoKHR`] structure specifying parameters of the presentation.

# Description
## Valid Usage
-    Each element of `pSwapchains` member of [`p_present_info`] **must**  be a swapchain that is created for a surface for which presentation is supported from [`queue`] as determined using a call to [`get_physical_device_surface_support_khr`]
-    If more than one member of `pSwapchains` was created from a display surface, all display surfaces referenced that refer to the same display  **must**  use the same display mode
-    When a semaphore wait operation referring to a binary semaphore defined by the elements of the `pWaitSemaphores` member of [`p_present_info`] executes on [`queue`], there  **must**  be no other queues waiting on the same semaphore
-    All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  be semaphores that are signaled, or have [semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) previously submitted for execution
-    All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  be created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
-    All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  reference a semaphore signal operation that has been submitted for execution and any semaphore signal operations on which it depends (if any)  **must**  have also been submitted for execution
Any writes to memory backing the images referenced by the
`pImageIndices` and `pSwapchains` members of [`p_present_info`],
that are available before [`queue_present_khr`] is executed, are
automatically made visible to the read access performed by the presentation
engine.
This automatic visibility operation for an image happens-after the semaphore
signal operation, and happens-before the presentation engine accesses the
image.Queueing an image for presentation defines a set of *queue operations*,
including waiting on the semaphores and submitting a presentation request to
the presentation engine.
However, the scope of this set of queue operations does not include the
actual processing of the image by the presentation engine.If [`queue_present_khr`] fails to enqueue the corresponding set of queue
operations, it  **may**  return `VK_ERROR_OUT_OF_HOST_MEMORY` or
`VK_ERROR_OUT_OF_DEVICE_MEMORY`.
If it does, the implementation  **must**  ensure that the state and contents of
any resources or synchronization primitives referenced is unaffected by the
call or its failure.If [`queue_present_khr`] fails in such a way that the implementation is
unable to make that guarantee, the implementation  **must**  return
`VK_ERROR_DEVICE_LOST`.However, if the presentation request is rejected by the presentation engine
with an error `VK_ERROR_OUT_OF_DATE_KHR`,
`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`,
or `VK_ERROR_SURFACE_LOST_KHR`, the set of queue operations are still
considered to be enqueued and thus any semaphore wait operation specified in
[`PresentInfoKHR`] will execute when the corresponding queue operation
is complete.Calls to [`queue_present_khr`] **may**  block, but  **must**  return in finite
time.If any `swapchain` member of [`p_present_info`] was created with
`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`,
`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` will be returned if that
swapchain does not have exclusive full-screen access, possibly for
implementation-specific reasons outside of the application’s control.
## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle
-  [`p_present_info`] **must**  be a valid pointer to a valid [`PresentInfoKHR`] structure

## Host Synchronization
- Host access to [`queue`] **must**  be externally synchronized
- Host access to `pPresentInfo->pWaitSemaphores`[]  **must**  be externally synchronized
- Host access to `pPresentInfo->pSwapchains`[]  **must**  be externally synchronized

## Command Properties
## Return Codes
*   - `VK_SUCCESS`  - `VK_SUBOPTIMAL_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`

# Related
- [`VK_KHR_swapchain`]
- [`PresentInfoKHR`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        