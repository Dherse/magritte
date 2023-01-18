[VK_NV_coverage_reduction_mode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_coverage_reduction_mode.html) - device extension

# Description
When using a framebuffer with mixed samples, a per-fragment coverage
reduction operation is performed which generates color sample coverage from
the pixel coverage.
This extension defines the following modes to control how this reduction is
performed.
- Merge: When there are more samples in the pixel coverage than color samples, there is an implementation-dependent association of each pixel coverage sample to a color sample. In the merge mode, the color sample coverage is computed such that only if any associated sample in the pixel coverage is covered, the color sample is covered. This is the default mode.
- Truncate: When there are more raster samples (N) than color samples(M), there is one to one association of the first M raster samples to the M color samples; other raster samples are ignored.
When the number of raster samples is equal to the color samples, there is a
one to one mapping between them in either of the above modes.The new command
[`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`] can
be used to query the various raster, color, depth/stencil sample count and
reduction mode combinations that are supported by the implementation.
This extension would allow an implementation to support the behavior of both
[`VK_NV_framebuffer_mixed_samples`] and [`VK_AMD_mixed_attachment_samples`]
extensions simultaneously.

# Registered extension number
251

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_NV_framebuffer_mixed_samples`]`

# Contacts
- Kedarnath Thangudu [kthangudu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_coverage_reduction_mode] @kthangudu%0A<<Here describe the issue or question you have about the VK_NV_coverage_reduction_mode extension>>)

# New commands
- [`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]

# New structures
- [`FramebufferMixedSamplesCombinationNV`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceCoverageReductionModeFeaturesNV`] 
- Extending [`PipelineMultisampleStateCreateInfo`]:  - [`PipelineCoverageReductionStateCreateInfoNV`]

# New enums
- [`CoverageReductionModeNV`]

# New bitmasks
- [`PipelineCoverageReductionStateCreateFlagsNV`]

# New constants
- [`NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME`]
- [`NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`  - `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`

# Version history
- Revision 1, 2019-01-29 (Kedarnath Thangudu)  - Internal revisions

# Other information
* 2019-01-29
*   - Kedarnath Thangudu, NVIDIA  - Jeff Bolz, NVIDIA

# Related
- [`CoverageReductionModeNV`]
- [`FramebufferMixedSamplesCombinationNV`]
- [`PhysicalDeviceCoverageReductionModeFeaturesNV`]
- [`PipelineCoverageReductionStateCreateFlagsNV`]
- [`PipelineCoverageReductionStateCreateInfoNV`]
- [`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        