[VkDeviceQueueCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateInfo.html) - Structure specifying parameters of a newly created device queue

# C Specifications
The [`DeviceQueueCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkDeviceQueueCreateInfo {
    VkStructureType             sType;
    const void*                 pNext;
    VkDeviceQueueCreateFlags    flags;
    uint32_t                    queueFamilyIndex;
    uint32_t                    queueCount;
    const float*                pQueuePriorities;
} VkDeviceQueueCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask indicating behavior of the queue.
- [`queue_family_index`] is an unsigned integer indicating the index of the queue family in which to create the queue on this device. This index corresponds to the index of an element of the `pQueueFamilyProperties` array that was returned by [`get_physical_device_queue_family_properties`].
- [`queue_count`] is an unsigned integer specifying the number of queues to create in the queue family indicated by [`queue_family_index`].
- [`queue_priorities`] is a pointer to an array of [`queue_count`] normalized floating point values, specifying priorities of work that will be submitted to each created queue. See [Queue Priority](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-priority) for more information.

# Description
## Valid Usage
-  [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by [`get_physical_device_queue_family_properties`]
-  [`queue_count`] **must**  be less than or equal to the [`queue_count`] member of the [`QueueFamilyProperties`] structure, as returned by [`get_physical_device_queue_family_properties`] in the `pQueueFamilyProperties`[queueFamilyIndex]
-    Each element of [`queue_priorities`] **must**  be between `0.0` and `1.0` inclusive
-    If the [protected memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-protectedMemory) feature is not enabled, the `VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT` bit of [`flags`] **must**  not be set
-    If [`flags`] includes `VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT`, [`queue_family_index`] **must**  be the index of a queue family that includes the `VK_QUEUE_PROTECTED_BIT` capability

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`DeviceQueueGlobalPriorityCreateInfoKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`DeviceQueueCreateFlagBits`] values
-  [`queue_priorities`] **must**  be a valid pointer to an array of [`queue_count`]`float` values
-  [`queue_count`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_0`]
- [`DeviceCreateInfo`]
- [`DeviceQueueCreateFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        