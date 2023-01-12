[VkAcquireProfilingLockInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockInfoKHR.html) - Structure specifying parameters to acquire the profiling lock

# C Specifications
The [`AcquireProfilingLockInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_performance_query
typedef struct VkAcquireProfilingLockInfoKHR {
    VkStructureType                   sType;
    const void*                       pNext;
    VkAcquireProfilingLockFlagsKHR    flags;
    uint64_t                          timeout;
} VkAcquireProfilingLockInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`timeout`] indicates how long the function waits, in nanoseconds, if the profiling lock is not available.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
If [`timeout`] is 0, [`acquire_profiling_lock_khr`] will not block while
attempting to acquire the profling lock.
If [`timeout`] is `UINT64_MAX`, the function will not return until the
profiling lock was acquired.

# Related
- [`khr_performance_query`]
- [VkAcquireProfilingLockFlagsKHR]()
- [`StructureType`]
- [`acquire_profiling_lock_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        