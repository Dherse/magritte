[vkWaitForFences](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForFences.html) - Wait for one or more fences to become signaled

# C Specifications
To wait for one or more fences to enter the signaled state on the host,
call:
```c
// Provided by VK_VERSION_1_0
VkResult vkWaitForFences(
    VkDevice                                    device,
    uint32_t                                    fenceCount,
    const VkFence*                              pFences,
    VkBool32                                    waitAll,
    uint64_t                                    timeout);
```

# Parameters
- [`device`] is the logical device that owns the fences.
- [`fence_count`] is the number of fences to wait on.
- [`p_fences`] is a pointer to an array of [`fence_count`] fence handles.
- [`wait_all`] is the condition that  **must**  be satisfied to successfully unblock the wait. If [`wait_all`] is [`TRUE`], then the condition is that all fences in [`p_fences`] are signaled. Otherwise, the condition is that at least one fence in [`p_fences`] is signaled.
- [`timeout`] is the timeout period in units of nanoseconds. [`timeout`] is adjusted to the closest value allowed by the implementation-dependent timeout accuracy, which  **may**  be substantially longer than one nanosecond, and  **may**  be longer than the requested period.

# Description
If the condition is satisfied when [`wait_for_fences`] is called, then
[`wait_for_fences`] returns immediately.
If the condition is not satisfied at the time [`wait_for_fences`] is
called, then [`wait_for_fences`] will block and wait until the condition
is satisfied or the [`timeout`] has expired, whichever is sooner.If [`timeout`] is zero, then [`wait_for_fences`] does not wait, but
simply returns the current state of the fences.
`VK_TIMEOUT` will be returned in this case if the condition is not
satisfied, even though no actual wait was performed.If the condition is satisfied before the [`timeout`] has expired,
[`wait_for_fences`] returns `VK_SUCCESS`.
Otherwise, [`wait_for_fences`] returns `VK_TIMEOUT` after the
[`timeout`] has expired.If device loss occurs (see [Lost Device](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device)) before
the timeout has expired, [`wait_for_fences`] **must**  return in finite time
with either `VK_SUCCESS` or `VK_ERROR_DEVICE_LOST`.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_fences`] **must**  be a valid pointer to an array of [`fence_count`] valid [`Fence`] handles
-  [`fence_count`] **must**  be greater than `0`
-    Each element of [`p_fences`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_TIMEOUT` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`Device`]
- [`Fence`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        