[vkSignalSemaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSignalSemaphore.html) - Signal a timeline semaphore on the host

# C Specifications
To signal a semaphore created with a [`SemaphoreType`] of
`VK_SEMAPHORE_TYPE_TIMELINE` with a particular counter value, on the
host, call:
```c
// Provided by VK_VERSION_1_2
VkResult vkSignalSemaphore(
    VkDevice                                    device,
    const VkSemaphoreSignalInfo*                pSignalInfo);
```
or the equivalent command
```c
// Provided by VK_KHR_timeline_semaphore
VkResult vkSignalSemaphoreKHR(
    VkDevice                                    device,
    const VkSemaphoreSignalInfo*                pSignalInfo);
```

# Parameters
- [`device`] is the logical device that owns the semaphore.
- [`p_signal_info`] is a pointer to a [`SemaphoreSignalInfo`] structure containing information about the signal operation.

# Description
When [`signal_semaphore`] is executed on the host, it defines and
immediately executes a [*semaphore
signal operation*](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) which sets the timeline semaphore to the given value.The first synchronization scope is defined by the host execution model, but
includes execution of [`signal_semaphore`] on the host and anything that
happened-before it.The second synchronization scope is empty.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_signal_info`] **must**  be a valid pointer to a valid [`SemaphoreSignalInfo`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_timeline_semaphore`]
- [`crate::vulkan1_2`]
- [`Device`]
- [`SemaphoreSignalInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        