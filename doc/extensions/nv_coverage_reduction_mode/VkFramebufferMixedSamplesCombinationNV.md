[VkFramebufferMixedSamplesCombinationNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html) - Structure specifying a supported sample count combination

# C Specifications
The [`FramebufferMixedSamplesCombinationNV`] structure is defined as:
```c
// Provided by VK_NV_coverage_reduction_mode
typedef struct VkFramebufferMixedSamplesCombinationNV {
    VkStructureType              sType;
    void*                        pNext;
    VkCoverageReductionModeNV    coverageReductionMode;
    VkSampleCountFlagBits        rasterizationSamples;
    VkSampleCountFlags           depthStencilSamples;
    VkSampleCountFlags           colorSamples;
} VkFramebufferMixedSamplesCombinationNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value specifying the coverage reduction mode.
- [`rasterization_samples`] is a [`SampleCountFlagBits`] specifying the number of rasterization samples in the supported combination.
- [`depth_stencil_samples`] specifies the number of samples in the depth stencil attachment in the supported combination. A value of 0 indicates the combination does not have a depth stencil attachment.
- [`color_samples`] specifies the number of color samples in a color attachment in the supported combination. A value of 0 indicates the combination does not have a color attachment.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_NV_coverage_reduction_mode`]
- [`CoverageReductionModeNV`]
- [`SampleCountFlagBits`]
- [`SampleCountFlags`]
- [`StructureType`]
- [`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        