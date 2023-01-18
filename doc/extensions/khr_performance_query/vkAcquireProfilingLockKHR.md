[vkAcquireProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html) - Acquires the profiling lock

# C Specifications
To record and submit a command buffer containing a performance query pool
the profiling lock  **must**  be held.
The profiling lock  **must**  be acquired prior to any call to
[`begin_command_buffer`] that will be using a performance query pool.
The profiling lock  **must**  be held while any command buffer containing a
performance query pool is in the *recording*, *executable*, or *pending
state*.
To acquire the profiling lock, call:
```c
// Provided by VK_KHR_performance_query
VkResult vkAcquireProfilingLockKHR(
    VkDevice                                    device,
    const VkAcquireProfilingLockInfoKHR*        pInfo);
```

# Parameters
- [`device`] is the logical device to profile.
- [`p_info`] is a pointer to a [`AcquireProfilingLockInfoKHR`] structure containing information about how the profiling is to be acquired.

# Description
Implementations **may**  allow multiple actors to hold the profiling lock
concurrently.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`AcquireProfilingLockInfoKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_TIMEOUT`

# Related
- [`VK_KHR_performance_query`]
- [`AcquireProfilingLockInfoKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        