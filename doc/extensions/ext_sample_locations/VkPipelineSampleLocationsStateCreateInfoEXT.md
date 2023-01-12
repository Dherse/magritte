[VkPipelineSampleLocationsStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html) - Structure specifying sample locations for a pipeline

# C Specifications
Applications  **can**  also control the sample locations used for rasterization.If the [`p_next`] chain of the [`PipelineMultisampleStateCreateInfo`]
structure specified at pipeline creation time includes a
[`PipelineSampleLocationsStateCreateInfoEXT`] structure, then that
structure controls the sample locations used when rasterizing primitives
with the pipeline.The [`PipelineSampleLocationsStateCreateInfoEXT`] structure is defined
as:
```c
// Provided by VK_EXT_sample_locations
typedef struct VkPipelineSampleLocationsStateCreateInfoEXT {
    VkStructureType             sType;
    const void*                 pNext;
    VkBool32                    sampleLocationsEnable;
    VkSampleLocationsInfoEXT    sampleLocationsInfo;
} VkPipelineSampleLocationsStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`sample_locations_enable`] controls whether custom sample locations are used. If [`sample_locations_enable`] is `VK_FALSE`, the default sample locations are used and the values specified in [`sample_locations_info`] are ignored.
- [`sample_locations_info`] is the sample locations to use during rasterization if [`sample_locations_enable`] is `VK_TRUE` and the graphics pipeline is not created with `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT`.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`
-  [`sample_locations_info`] **must**  be a valid [`SampleLocationsInfoEXT`] structure

# Related
- [`ext_sample_locations`]
- [`Bool32`]
- [`SampleLocationsInfoEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        