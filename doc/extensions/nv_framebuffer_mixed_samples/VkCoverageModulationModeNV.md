[VkCoverageModulationModeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageModulationModeNV.html) - Specify the coverage modulation mode

# C Specifications
Possible values of
[`PipelineCoverageModulationStateCreateInfoNV::coverage_modulation_mode`],
specifying which color components are modulated, are:
```c
// Provided by VK_NV_framebuffer_mixed_samples
typedef enum VkCoverageModulationModeNV {
    VK_COVERAGE_MODULATION_MODE_NONE_NV = 0,
    VK_COVERAGE_MODULATION_MODE_RGB_NV = 1,
    VK_COVERAGE_MODULATION_MODE_ALPHA_NV = 2,
    VK_COVERAGE_MODULATION_MODE_RGBA_NV = 3,
} VkCoverageModulationModeNV;
```

# Description
- [`VK_COVERAGE_MODULATION_MODE_NV`] specifies that no components are multiplied by the modulation factor.
- [`VK_COVERAGE_MODULATION_MODE_NV`] specifies that the red, green, and blue components are multiplied by the modulation factor.
- [`VK_COVERAGE_MODULATION_MODE_NV`] specifies that the alpha component is multiplied by the modulation factor.
- [`VK_COVERAGE_MODULATION_MODE_NV`] specifies that all components are multiplied by the modulation factor.

# Related
- [`nv_framebuffer_mixed_samples`]
- [`PipelineCoverageModulationStateCreateInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        