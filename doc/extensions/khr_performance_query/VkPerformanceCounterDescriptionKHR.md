[VkPerformanceCounterDescriptionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionKHR.html) - Structure providing more detailed information about a counter

# C Specifications
The [`PerformanceCounterDescriptionKHR`] structure is defined as:
```c
// Provided by VK_KHR_performance_query
typedef struct VkPerformanceCounterDescriptionKHR {
    VkStructureType                            sType;
    void*                                      pNext;
    VkPerformanceCounterDescriptionFlagsKHR    flags;
    char                                       name[VK_MAX_DESCRIPTION_SIZE];
    char                                       category[VK_MAX_DESCRIPTION_SIZE];
    char                                       description[VK_MAX_DESCRIPTION_SIZE];
} VkPerformanceCounterDescriptionKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`PerformanceCounterDescriptionFlagBitsKHR`] indicating the usage behavior for the counter.
- [`name`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated UTF-8 string specifying the name of the counter.
- [`category`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated UTF-8 string specifying the category of the counter.
- [`description`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated UTF-8 string specifying the description of the counter.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_performance_query`]
- [`PerformanceCounterDescriptionFlagsKHR`]
- [`StructureType`]
- [`enumerate_physical_device_queue_family_performance_query_counters_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        