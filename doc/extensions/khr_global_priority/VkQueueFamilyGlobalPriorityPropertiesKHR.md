[VkQueueFamilyGlobalPriorityPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyGlobalPriorityPropertiesKHR.html) - Return structure for queue family global priority information query

# C Specifications
The definition of [`QueueFamilyGlobalPriorityPropertiesKHR`] is:
```c
// Provided by VK_KHR_global_priority
typedef struct VkQueueFamilyGlobalPriorityPropertiesKHR {
    VkStructureType             sType;
    void*                       pNext;
    uint32_t                    priorityCount;
    VkQueueGlobalPriorityKHR    priorities[VK_MAX_GLOBAL_PRIORITY_SIZE_KHR];
} VkQueueFamilyGlobalPriorityPropertiesKHR;
```
or the equivalent
```c
// Provided by VK_EXT_global_priority_query
typedef VkQueueFamilyGlobalPriorityPropertiesKHR VkQueueFamilyGlobalPriorityPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`priority_count`] is the number of supported global queue priorities in this queue family, and it  **must**  be greater than 0.
- [`priorities`] is an array of [`MAX_GLOBAL_PRIORITY_SIZE_EXT`][`QueueGlobalPriorityEXT`] enums representing all supported global queue priorities in this queue family. The first [`priority_count`] elements of the array will be valid.

# Description
If the [`QueueFamilyGlobalPriorityPropertiesKHR`] structure is included
in the [`p_next`] chain of the [`QueueFamilyProperties2`] structure
passed to [`get_physical_device_queue_family_properties2`], it is filled in
with the list of supported global queue priorities for the indicated family.The valid elements of [`priorities`] **must**  not contain any duplicate
values.The valid elements of [`priorities`] **must**  be a continuous sequence of
[`QueueGlobalPriorityKHR`] enums in the ascending order.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR`
-    Any given element of [`priorities`] **must**  be a valid [`QueueGlobalPriorityKHR`] value

# Related
- [`VK_KHR_global_priority`]
- [`QueueGlobalPriorityKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        