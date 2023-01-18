[vkWaitSemaphores](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitSemaphores.html) - Wait for timeline semaphores on the host

# C Specifications
To wait for a set of semaphores created with a [`SemaphoreType`] of
`VK_SEMAPHORE_TYPE_TIMELINE` to reach particular counter values on the
host, call:
```c
// Provided by VK_VERSION_1_2
VkResult vkWaitSemaphores(
    VkDevice                                    device,
    const VkSemaphoreWaitInfo*                  pWaitInfo,
    uint64_t                                    timeout);
```
or the equivalent command
```c
// Provided by VK_KHR_timeline_semaphore
VkResult vkWaitSemaphoresKHR(
    VkDevice                                    device,
    const VkSemaphoreWaitInfo*                  pWaitInfo,
    uint64_t                                    timeout);
```

# Parameters
- [`device`] is the logical device that owns the semaphores.
- [`p_wait_info`] is a pointer to a [`SemaphoreWaitInfo`] structure containing information about the wait condition.
- [`timeout`] is the timeout period in units of nanoseconds. [`timeout`] is adjusted to the closest value allowed by the implementation-dependent timeout accuracy, which  **may**  be substantially longer than one nanosecond, and  **may**  be longer than the requested period.

# Description
If the condition is satisfied when [`wait_semaphores`] is called, then
[`wait_semaphores`] returns immediately.
If the condition is not satisfied at the time [`wait_semaphores`] is
called, then [`wait_semaphores`] will block and wait until the condition
is satisfied or the [`timeout`] has expired, whichever is sooner.If [`timeout`] is zero, then [`wait_semaphores`] does not wait, but
simply returns information about the current state of the semaphores.
`VK_TIMEOUT` will be returned in this case if the condition is not
satisfied, even though no actual wait was performed.If the condition is satisfied before the [`timeout`] has expired,
[`wait_semaphores`] returns `VK_SUCCESS`.
Otherwise, [`wait_semaphores`] returns `VK_TIMEOUT` after the
[`timeout`] has expired.If device loss occurs (see [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device)) before
the timeout has expired, [`wait_semaphores`] **must**  return in finite time
with either `VK_SUCCESS` or `VK_ERROR_DEVICE_LOST`.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_wait_info`] **must**  be a valid pointer to a valid [`SemaphoreWaitInfo`] structure

## Return Codes
*   - `VK_SUCCESS`  - `VK_TIMEOUT` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`VK_KHR_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [`Device`]
- [`SemaphoreWaitInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        