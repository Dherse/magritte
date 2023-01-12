[vkReleaseProfilingLockKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html) - Releases the profiling lock

# C Specifications
To release the profiling lock, call:
```c
// Provided by VK_KHR_performance_query
void vkReleaseProfilingLockKHR(
    VkDevice                                    device);
```

# Parameters
- [`device`] is the logical device to cease profiling on.

# Description
## Valid Usage
-    The profiling lock of [`device`] **must**  have been held via a previous successful call to [`acquire_profiling_lock_khr`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle

# Related
- [`khr_performance_query`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        