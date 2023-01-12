[vkGetSemaphoreCounterValue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreCounterValue.html) - Query the current state of a timeline semaphore

# C Specifications
To query the current counter value of a semaphore created with a
[`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` from the host,
call:
```c
// Provided by VK_VERSION_1_2
VkResult vkGetSemaphoreCounterValue(
    VkDevice                                    device,
    VkSemaphore                                 semaphore,
    uint64_t*                                   pValue);
```
or the equivalent command
```c
// Provided by VK_KHR_timeline_semaphore
VkResult vkGetSemaphoreCounterValueKHR(
    VkDevice                                    device,
    VkSemaphore                                 semaphore,
    uint64_t*                                   pValue);
```

# Parameters
- [`device`] is the logical device that owns the semaphore.
- [`semaphore`] is the handle of the semaphore to query.
- [`p_value`] is a pointer to a 64-bit integer value in which the current counter value of the semaphore is returned.

# Description
## Valid Usage
-  [`semaphore`] **must**  have been created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`semaphore`] **must**  be a valid [`Semaphore`] handle
-  [`p_value`] **must**  be a valid pointer to a `uint64_t` value
-  [`semaphore`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`khr_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [`Device`]
- [`Semaphore`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        