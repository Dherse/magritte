[VkPerformanceCounterKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterKHR.html) - Structure providing information about a counter

# C Specifications
The [`PerformanceCounterKHR`] structure is defined as:
```c
// Provided by VK_KHR_performance_query
typedef struct VkPerformanceCounterKHR {
    VkStructureType                   sType;
    void*                             pNext;
    VkPerformanceCounterUnitKHR       unit;
    VkPerformanceCounterScopeKHR      scope;
    VkPerformanceCounterStorageKHR    storage;
    uint8_t                           uuid[VK_UUID_SIZE];
} VkPerformanceCounterKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`unit`] is a [`PerformanceCounterUnitKHR`] specifying the unit that the counter data will record.
- [`scope`] is a [`PerformanceCounterScopeKHR`] specifying the scope that the counter belongs to.
- [`storage`] is a [`PerformanceCounterStorageKHR`] specifying the storage type that the counter’s data uses.
- [`uuid`] is an array of size `VK_UUID_SIZE`, containing 8-bit values that represent a universally unique identifier for the counter of the physical device.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`khr_performance_query`]
- [`PerformanceCounterScopeKHR`]
- [`PerformanceCounterStorageKHR`]
- [`PerformanceCounterUnitKHR`]
- [`StructureType`]
- [`enumerate_physical_device_queue_family_performance_query_counters_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        