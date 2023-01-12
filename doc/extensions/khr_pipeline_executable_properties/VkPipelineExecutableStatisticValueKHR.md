[VkPipelineExecutableStatisticValueKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html) - A union describing a pipeline executable statistic

# C Specifications
The [`PipelineExecutableStatisticValueKHR`] union is defined as:
```c
// Provided by VK_KHR_pipeline_executable_properties
typedef union VkPipelineExecutableStatisticValueKHR {
    VkBool32    b32;
    int64_t     i64;
    uint64_t    u64;
    double      f64;
} VkPipelineExecutableStatisticValueKHR;
```

# Members
- [`b32`] is the 32-bit boolean value if the [`PipelineExecutableStatisticFormatKHR`] is `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR`.
- [`i64`] is the signed 64-bit integer value if the [`PipelineExecutableStatisticFormatKHR`] is `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR`.
- [`u64`] is the unsigned 64-bit integer value if the [`PipelineExecutableStatisticFormatKHR`] is `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR`.
- [`f64`] is the 64-bit floating-point value if the [`PipelineExecutableStatisticFormatKHR`] is `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR`.

# Related
- [`khr_pipeline_executable_properties`]
- [`Bool32`]
- [`PipelineExecutableStatisticKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        