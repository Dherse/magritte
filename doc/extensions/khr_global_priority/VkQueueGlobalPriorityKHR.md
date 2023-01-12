[VkQueueGlobalPriorityKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueGlobalPriorityKHR.html) - Values specifying a system-wide queue priority

# C Specifications
Possible values of
[`DeviceQueueGlobalPriorityCreateInfoKHR::global_priority`],
specifying a system-wide priority level are:
```c
// Provided by VK_KHR_global_priority
typedef enum VkQueueGlobalPriorityKHR {
    VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR = 128,
    VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR = 256,
    VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR = 512,
    VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR = 1024,
    VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT = VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR,
    VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT = VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR,
    VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT = VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR,
    VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT = VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR,
} VkQueueGlobalPriorityKHR;
```
or the equivalent
```c
// Provided by VK_EXT_global_priority
typedef VkQueueGlobalPriorityKHR VkQueueGlobalPriorityEXT;
```

# Description
Priority values are sorted in ascending order.
A comparison operation on the enum values can be used to determine the
priority order.
- [`VK_QUEUE_GLOBAL_PRIORITY_KHR`] is below the system default. Useful for non-interactive tasks.
- [`VK_QUEUE_GLOBAL_PRIORITY_KHR`] is the system default priority.
- [`VK_QUEUE_GLOBAL_PRIORITY_KHR`] is above the system default.
- [`VK_QUEUE_GLOBAL_PRIORITY_KHR`] is the highest priority. Useful for critical tasks.

# Related
- [`ext_global_priority`]
- [`khr_global_priority`]
- [`DeviceQueueGlobalPriorityCreateInfoKHR`]
- [`QueueFamilyGlobalPriorityPropertiesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        