[VkCoverageReductionModeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageReductionModeNV.html) - Specify the coverage reduction mode

# C Specifications
Possible values of
[`PipelineCoverageReductionStateCreateInfoNV::coverage_reduction_mode`],
specifying how color sample coverage is generated from pixel coverage, are:
```c
// Provided by VK_NV_coverage_reduction_mode
typedef enum VkCoverageReductionModeNV {
    VK_COVERAGE_REDUCTION_MODE_MERGE_NV = 0,
    VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV = 1,
} VkCoverageReductionModeNV;
```

# Description
- [`MERGE`] specifies that each color sample will be associated with an implementation-dependent subset of samples in the pixel coverage. If any of those associated samples are covered, the color sample is covered.
- [`TRUNCATE`] specifies that for color samples present in the color attachments, a color sample is covered if the pixel coverage sample with the same [sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)i is covered; other pixel coverage samples are discarded.

# Related
- [`VK_NV_coverage_reduction_mode`]
- [`FramebufferMixedSamplesCombinationNV`]
- [`PipelineCoverageReductionStateCreateInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        