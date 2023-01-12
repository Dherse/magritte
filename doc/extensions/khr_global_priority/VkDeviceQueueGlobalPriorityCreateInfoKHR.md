[VkDeviceQueueGlobalPriorityCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueGlobalPriorityCreateInfoKHR.html) - Specify a system wide priority

# C Specifications
A queue  **can**  be created with a system-wide priority by adding a
[`DeviceQueueGlobalPriorityCreateInfoKHR`] structure to the [`p_next`]
chain of [`DeviceQueueCreateInfo`].The [`DeviceQueueGlobalPriorityCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_global_priority
typedef struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
    VkStructureType             sType;
    const void*                 pNext;
    VkQueueGlobalPriorityKHR    globalPriority;
} VkDeviceQueueGlobalPriorityCreateInfoKHR;
```
or the equivalent
```c
// Provided by VK_EXT_global_priority
typedef VkDeviceQueueGlobalPriorityCreateInfoKHR VkDeviceQueueGlobalPriorityCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`global_priority`] is the system-wide priority associated to this queue as specified by [`QueueGlobalPriorityEXT`]

# Description
A queue created without specifying
[`DeviceQueueGlobalPriorityCreateInfoKHR`] will default to
`VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR`
-  [`global_priority`] **must**  be a valid [`QueueGlobalPriorityKHR`] value

# Related
- [`ext_global_priority`]
- [`khr_global_priority`]
- [`QueueGlobalPriorityKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        