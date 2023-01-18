[vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html) - Reports the number of passes require for a performance query pool type

# C Specifications
To query the number of passes required to query a performance query pool on
a physical device, call:
```c
// Provided by VK_KHR_performance_query
void vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    VkPhysicalDevice                            physicalDevice,
    const VkQueryPoolPerformanceCreateInfoKHR*  pPerformanceQueryCreateInfo,
    uint32_t*                                   pNumPasses);
```

# Parameters
- [`physical_device`] is the handle to the physical device whose queue family performance query counter properties will be queried.
- [`p_performance_query_create_info`] is a pointer to a [`QueryPoolPerformanceCreateInfoKHR`] of the performance query that is to be created.
- [`p_num_passes`] is a pointer to an integer related to the number of passes required to query the performance query pool, as described below.

# Description
The [`p_performance_query_create_info`] member
[`QueryPoolPerformanceCreateInfoKHR::queue_family_index`] **must**  be a
queue family of [`physical_device`].
The number of passes required to capture the counters specified in the
[`p_performance_query_create_info`] member
[`QueryPoolPerformanceCreateInfoKHR`]`::pCounters` is returned in
[`p_num_passes`].
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_performance_query_create_info`] **must**  be a valid pointer to a valid [`QueryPoolPerformanceCreateInfoKHR`] structure
-  [`p_num_passes`] **must**  be a valid pointer to a `uint32_t` value

# Related
- [`VK_KHR_performance_query`]
- [`PhysicalDevice`]
- [`QueryPoolPerformanceCreateInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        