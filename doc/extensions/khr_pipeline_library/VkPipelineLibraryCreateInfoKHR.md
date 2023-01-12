[VkPipelineLibraryCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html) - Structure specifying pipeline libraries to use when creating a pipeline

# C Specifications
The [`PipelineLibraryCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_pipeline_library
typedef struct VkPipelineLibraryCreateInfoKHR {
    VkStructureType      sType;
    const void*          pNext;
    uint32_t             libraryCount;
    const VkPipeline*    pLibraries;
} VkPipelineLibraryCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`library_count`] is the number of pipeline libraries in [`libraries`].
- [`libraries`] is a pointer to an array of [`Pipeline`] structures specifying pipeline libraries to use when creating a pipeline.

# Description
## Valid Usage
-    Each element of [`libraries`] **must**  have been created with `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-    If [`library_count`] is not `0`, [`libraries`] **must**  be a valid pointer to an array of [`library_count`] valid [`Pipeline`] handles

# Related
- [`khr_pipeline_library`]
- [`Pipeline`]
- [`RayTracingPipelineCreateInfoKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        