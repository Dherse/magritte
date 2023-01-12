[VkQueueFamilyProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2.html) - Structure providing information about a queue family

# C Specifications
The [`QueueFamilyProperties2`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkQueueFamilyProperties2 {
    VkStructureType            sType;
    void*                      pNext;
    VkQueueFamilyProperties    queueFamilyProperties;
} VkQueueFamilyProperties2;
```
or the equivalent
```c
// Provided by VK_KHR_get_physical_device_properties2
typedef VkQueueFamilyProperties2 VkQueueFamilyProperties2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`queue_family_properties`] is a [`QueueFamilyProperties`] structure which is populated with the same values as in [`get_physical_device_queue_family_properties`].

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`QueueFamilyCheckpointProperties2NV`], [`QueueFamilyCheckpointPropertiesNV`], [`QueueFamilyGlobalPriorityPropertiesKHR`], [`QueueFamilyQueryResultStatusProperties2KHR`], or [`VideoQueueFamilyProperties2KHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`crate::vulkan1_1`]
- [`QueueFamilyProperties`]
- [`StructureType`]
- [`get_physical_device_queue_family_properties2`]
- [`get_physical_device_queue_family_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        