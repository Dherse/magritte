[VkPipelineCoverageReductionStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html) - Structure specifying parameters controlling coverage reduction

# C Specifications
The [`PipelineCoverageReductionStateCreateInfoNV`] structure is defined
as:
```c
// Provided by VK_NV_coverage_reduction_mode
typedef struct VkPipelineCoverageReductionStateCreateInfoNV {
    VkStructureType                                  sType;
    const void*                                      pNext;
    VkPipelineCoverageReductionStateCreateFlagsNV    flags;
    VkCoverageReductionModeNV                        coverageReductionMode;
} VkPipelineCoverageReductionStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value controlling how color sample coverage is generated from pixel coverage.

# Description
If this structure is not included in the [`p_next`] chain, or if the
extension is not enabled, the default coverage reduction mode is inferred as
follows:
- If the `[`VK_NV_framebuffer_mixed_samples`]` extension is enabled, then it is as if the [`coverage_reduction_mode`] is `VK_COVERAGE_REDUCTION_MODE_MERGE_NV`.
- If the `[`VK_AMD_mixed_attachment_samples`]` extension is enabled, then it is as if the [`coverage_reduction_mode`] is `VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV`.
- If both `[`VK_NV_framebuffer_mixed_samples`]` and `[`VK_AMD_mixed_attachment_samples`]` are enabled, then the default coverage reduction mode is implementation-dependent.

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`
-  [`flags`] **must**  be `0`
-  [`coverage_reduction_mode`] **must**  be a valid [`CoverageReductionModeNV`] value

# Related
- [`VK_NV_coverage_reduction_mode`]
- [`CoverageReductionModeNV`]
- [`PipelineCoverageReductionStateCreateFlagsNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        