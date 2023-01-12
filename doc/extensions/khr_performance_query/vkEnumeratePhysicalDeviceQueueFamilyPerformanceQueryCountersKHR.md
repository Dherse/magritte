[vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html) - Reports properties of the performance query counters available on a queue family of a device

# C Specifications
To enumerate the performance query counters available on a queue family of a
physical device, call:
```c
// Provided by VK_KHR_performance_query
VkResult vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t                                    queueFamilyIndex,
    uint32_t*                                   pCounterCount,
    VkPerformanceCounterKHR*                    pCounters,
    VkPerformanceCounterDescriptionKHR*         pCounterDescriptions);
```

# Parameters
- [`physical_device`] is the handle to the physical device whose queue family performance query counter properties will be queried.
- [`queue_family_index`] is the index into the queue family of the physical device we want to get properties for.
- [`p_counter_count`] is a pointer to an integer related to the number of counters available or queried, as described below.
- [`p_counters`] is either `NULL` or a pointer to an array of [`PerformanceCounterKHR`] structures.
- [`p_counter_descriptions`] is either `NULL` or a pointer to an array of [`PerformanceCounterDescriptionKHR`] structures.

# Description
If [`p_counters`] is `NULL` and [`p_counter_descriptions`] is `NULL`, then
the number of counters available is returned in [`p_counter_count`].
Otherwise, [`p_counter_count`] **must**  point to a variable set by the user to
the number of elements in the [`p_counters`], [`p_counter_descriptions`],
or both arrays and on return the variable is overwritten with the number of
structures actually written out.
If [`p_counter_count`] is less than the number of counters available, at
most [`p_counter_count`] structures will be written, and `VK_INCOMPLETE`
will be returned instead of `VK_SUCCESS`, to indicate that not all the
available counters were returned.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_counter_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_counter_count`] is not `0`, and [`p_counters`] is not `NULL`, [`p_counters`] **must**  be a valid pointer to an array of [`p_counter_count`][`PerformanceCounterKHR`] structures
-    If the value referenced by [`p_counter_count`] is not `0`, and [`p_counter_descriptions`] is not `NULL`, [`p_counter_descriptions`] **must**  be a valid pointer to an array of [`p_counter_count`][`PerformanceCounterDescriptionKHR`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`khr_performance_query`]
- [`PerformanceCounterDescriptionKHR`]
- [`PerformanceCounterKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        