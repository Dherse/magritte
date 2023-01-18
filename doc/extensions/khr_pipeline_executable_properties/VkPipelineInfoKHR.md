[VkPipelineInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoKHR.html) - Structure describing a pipeline

# C Specifications
The [`PipelineInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_pipeline_executable_properties
typedef struct VkPipelineInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkPipeline         pipeline;
} VkPipelineInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`pipeline`] is a [`Pipeline`] handle.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle

# Related
- [`VK_KHR_pipeline_executable_properties`]
- [`Pipeline`]
- [`StructureType`]
- [`get_pipeline_executable_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        