[VkPerformanceQuerySubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html) - Structure indicating which counter pass index is active for performance queries

# C Specifications
The [`PerformanceQuerySubmitInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_performance_query
typedef struct VkPerformanceQuerySubmitInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           counterPassIndex;
} VkPerformanceQuerySubmitInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`counter_pass_index`] specifies which counter pass index is active.

# Description
If the [`SubmitInfo`]::[`p_next`] chain does not include this
structure, the batch defaults to use counter pass index 0.
## Valid Usage
-  [`counter_pass_index`] **must**  be less than the number of counter passes required by any queries within the batch. The required number of counter passes for a performance query is obtained by calling [`get_physical_device_queue_family_performance_query_passes_khr`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`

# Related
- [`khr_performance_query`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        