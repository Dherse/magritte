[VkInitializePerformanceApiInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) - Structure specifying parameters of initialize of the device

# C Specifications
The [`InitializePerformanceApiInfoINTEL`] structure is defined as :
```c
// Provided by VK_INTEL_performance_query
typedef struct VkInitializePerformanceApiInfoINTEL {
    VkStructureType    sType;
    const void*        pNext;
    void*              pUserData;
} VkInitializePerformanceApiInfoINTEL;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`user_data`] is a pointer for application data.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_INTEL_performance_query`]
- [`StructureType`]
- [`initialize_performance_api_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        