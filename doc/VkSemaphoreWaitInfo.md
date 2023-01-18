[VkSemaphoreWaitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfo.html) - Structure containing information about the semaphore wait condition

# C Specifications
The [`SemaphoreWaitInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSemaphoreWaitInfo {
    VkStructureType         sType;
    const void*             pNext;
    VkSemaphoreWaitFlags    flags;
    uint32_t                semaphoreCount;
    const VkSemaphore*      pSemaphores;
    const uint64_t*         pValues;
} VkSemaphoreWaitInfo;
```
or the equivalent
```c
// Provided by VK_KHR_timeline_semaphore
typedef VkSemaphoreWaitInfo VkSemaphoreWaitInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`SemaphoreWaitFlagBits`] specifying additional parameters for the semaphore wait operation.
- [`semaphore_count`] is the number of semaphores to wait on.
- [`semaphores`] is a pointer to an array of [`semaphore_count`] semaphore handles to wait on.
- [`values`] is a pointer to an array of [`semaphore_count`] timeline semaphore values.

# Description
## Valid Usage
-    All of the elements of [`semaphores`] **must**  reference a semaphore that was created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be a valid combination of [`SemaphoreWaitFlagBits`] values
-  [`semaphores`] **must**  be a valid pointer to an array of [`semaphore_count`] valid [`Semaphore`] handles
-  [`values`] **must**  be a valid pointer to an array of [`semaphore_count`]`uint64_t` values
-  [`semaphore_count`] **must**  be greater than `0`

# Related
- [`VK_KHR_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [`Semaphore`]
- [`SemaphoreWaitFlags`]
- [`StructureType`]
- [`wait_semaphores`]
- [`wait_semaphores_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        