[VkPipelineExecutableStatisticKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticKHR.html) - Structure describing a compile-time pipeline executable statistic

# C Specifications
The [`PipelineExecutableStatisticKHR`] structure is defined as:
```c
// Provided by VK_KHR_pipeline_executable_properties
typedef struct VkPipelineExecutableStatisticKHR {
    VkStructureType                           sType;
    void*                                     pNext;
    char                                      name[VK_MAX_DESCRIPTION_SIZE];
    char                                      description[VK_MAX_DESCRIPTION_SIZE];
    VkPipelineExecutableStatisticFormatKHR    format;
    VkPipelineExecutableStatisticValueKHR     value;
} VkPipelineExecutableStatisticKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`name`] is an array of `VK_MAX_DESCRIPTION_SIZE``char` containing a null-terminated UTF-8 string which is a short human readable name for this statistic.
- [`description`] is an array of `VK_MAX_DESCRIPTION_SIZE``char` containing a null-terminated UTF-8 string which is a human readable description for this statistic.
- [`format`] is a [`PipelineExecutableStatisticFormatKHR`] value specifying the format of the data found in [`value`].
- [`value`] is the value of this statistic.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`khr_pipeline_executable_properties`]
- [`PipelineExecutableStatisticFormatKHR`]
- [`PipelineExecutableStatisticValueKHR`]
- [`StructureType`]
- [`get_pipeline_executable_statistics_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        