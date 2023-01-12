[VkSemaphoreSignalInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSignalInfo.html) - Structure containing information about a semaphore signal operation

# C Specifications
The [`SemaphoreSignalInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSemaphoreSignalInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkSemaphore        semaphore;
    uint64_t           value;
} VkSemaphoreSignalInfo;
```
or the equivalent
```c
// Provided by VK_KHR_timeline_semaphore
typedef VkSemaphoreSignalInfo VkSemaphoreSignalInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`semaphore`] is the handle of the semaphore to signal.
- [`value`] is the value to signal.

# Description
## Valid Usage
-  [`semaphore`] **must**  have been created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`
-  [`value`] **must**  have a value greater than the current value of the semaphore
-  [`value`] **must**  be less than the value of any pending semaphore signal operations
-  [`value`] **must**  have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on [`semaphore`] by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`semaphore`] **must**  be a valid [`Semaphore`] handle

# Related
- [`khr_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [`Semaphore`]
- [`StructureType`]
- [`signal_semaphore`]
- [`signal_semaphore_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        