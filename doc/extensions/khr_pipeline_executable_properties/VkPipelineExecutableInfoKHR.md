[VkPipelineExecutableInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInfoKHR.html) - Structure describing a pipeline executable to query for associated statistics or internal representations

# C Specifications
The [`PipelineExecutableInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_pipeline_executable_properties
typedef struct VkPipelineExecutableInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkPipeline         pipeline;
    uint32_t           executableIndex;
} VkPipelineExecutableInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`pipeline`] is the pipeline to query.
- [`executable_index`] is the index of the pipeline executable to query in the array of executable properties returned by [`get_pipeline_executable_properties_khr`].

# Description
## Valid Usage
-  [`executable_index`] **must**  be less than the number of pipeline executables associated with [`pipeline`] as returned in the `pExecutableCount` parameter of [`get_pipeline_executable_properties_khr`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle

# Related
- [`VK_KHR_pipeline_executable_properties`]
- [`Pipeline`]
- [`StructureType`]
- [`get_pipeline_executable_internal_representations_khr`]
- [`get_pipeline_executable_statistics_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        