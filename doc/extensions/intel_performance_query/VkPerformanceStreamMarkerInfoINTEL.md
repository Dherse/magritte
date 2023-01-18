[VkPerformanceStreamMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html) - Structure specifying stream performance markers

# C Specifications
The [`PerformanceStreamMarkerInfoINTEL`] structure is defined as:
```c
// Provided by VK_INTEL_performance_query
typedef struct VkPerformanceStreamMarkerInfoINTEL {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           marker;
} VkPerformanceStreamMarkerInfoINTEL;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`marker`] is the marker value that will be recorded into the reports consumed by an external application.

# Description
## Valid Usage
-    The value written by the application into [`marker`] **must**  only used the valid bits as reported by [`get_performance_parameter_intel`] with the `VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_INTEL_performance_query`]
- [`StructureType`]
- [`cmd_set_performance_stream_marker_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        