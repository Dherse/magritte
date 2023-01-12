[VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR.html) - Structure describing whether global priority query can be supported by an implementation

# C Specifications
The [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] structure is
defined as:
```c
// Provided by VK_KHR_global_priority
typedef struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           globalPriorityQuery;
} VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR;
```
or the equivalent
```c
// Provided by VK_EXT_global_priority_query
typedef VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT;
```

# Members
The members of the [`PhysicalDeviceGlobalPriorityQueryFeaturesEXT`]
structure describe the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`global_priority_query`] indicates whether the implementation supports the ability to query global queue priorities.
If the [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR`

# Related
- [`khr_global_priority`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        