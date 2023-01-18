[VkPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutablePropertiesKHR.html) - Structure describing a pipeline executable

# C Specifications
The [`PipelineExecutablePropertiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_pipeline_executable_properties
typedef struct VkPipelineExecutablePropertiesKHR {
    VkStructureType       sType;
    void*                 pNext;
    VkShaderStageFlags    stages;
    char                  name[VK_MAX_DESCRIPTION_SIZE];
    char                  description[VK_MAX_DESCRIPTION_SIZE];
    uint32_t              subgroupSize;
} VkPipelineExecutablePropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`stages`] is a bitmask of zero or more [`ShaderStageFlagBits`] indicating which shader stages (if any) were principally used as inputs to compile this pipeline executable.
- [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8 string which is a short human readable name for this pipeline executable.
- [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8 string which is a human readable description for this pipeline executable.
- [`subgroup_size`] is the subgroup size with which this pipeline executable is dispatched.

# Description
Not all implementations have a 1:1 mapping between shader stages and
pipeline executables and some implementations  **may**  reduce a given shader
stage to fixed function hardware programming such that no pipeline
executable is available.
No guarantees are provided about the mapping between shader stages and
pipeline executables and [`stages`] **should**  be considered a best effort
hint.
Because the application  **cannot**  rely on the [`stages`] field to provide an
exact description, [`name`] and [`description`] provide a human readable
name and description which more accurately describes the given pipeline
executable.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_pipeline_executable_properties`]
- [`ShaderStageFlags`]
- [`StructureType`]
- [`get_pipeline_executable_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        