//![VK_NV_coverage_reduction_mode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_coverage_reduction_mode.html) - device extension
//!# Description
//!When using a framebuffer with mixed samples, a per-fragment coverage
//!reduction operation is performed which generates color sample coverage from
//!the pixel coverage.
//!This extension defines the following modes to control how this reduction is
//!performed.
//! - Merge: When there are more samples in the pixel coverage than color
//!samples, there is an implementation-dependent association of each pixel
//!coverage sample to a color sample.
//!In the merge mode, the color sample coverage is computed such that only
//!if any associated sample in the pixel coverage is covered, the color
//!sample is covered.
//!This is the default mode.
//! - Truncate: When there are more raster samples (N) than color samples(M),
//!there is one to one association of the first M raster samples to the M
//!color samples; other raster samples are ignored.
//!When the number of raster samples is equal to the color samples, there is a
//!one to one mapping between them in either of the above modes.The new command
//![`GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`] can
//!be used to query the various raster, color, depth/stencil sample count and
//!reduction mode combinations that are supported by the implementation.
//!This extension would allow an implementation to support the behavior of both
//![`VK_NV_framebuffer_mixed_samples`] and [`VK_AMD_mixed_attachment_samples`]
//!extensions simultaneously.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_NV_framebuffer_mixed_samples`]`
//!# Contacts
//! - Kedarnath Thangudu [kthangudu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_coverage_reduction_mode]
//!   @kthangudu%0A<<Here describe the issue or question you have about the
//!   VK_NV_coverage_reduction_mode extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`]
//!# New structures
//! - [`FramebufferMixedSamplesCombinationNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceCoverageReductionModeFeaturesNV`]
//!
//! - Extending [`PipelineMultisampleStateCreateInfo`]:
//! - [`PipelineCoverageReductionStateCreateInfoNV`]
//!# New enums
//! - [`CoverageReductionModeNV`]
//!# New bitmasks
//! - [`PipelineCoverageReductionStateCreateFlagsNV`]
//!# New constants
//! - [`NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME`]
//! - [`NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`
//! - `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`
//!# Version History
//! - Revision 1, 2019-01-29 (Kedarnath Thangudu)
//! - Internal revisions
//!# Other info
//! * 2019-01-29
//!*
//! - Kedarnath Thangudu, NVIDIA
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`CoverageReductionModeNV`]
//! - [`FramebufferMixedSamplesCombinationNV`]
//! - [`PhysicalDeviceCoverageReductionModeFeaturesNV`]
//! - [`PipelineCoverageReductionStateCreateFlagsNV`]
//! - [`PipelineCoverageReductionStateCreateInfoNV`]
//! - [`GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION")]
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME")]
pub const NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_coverage_reduction_mode");
///[VkCoverageReductionModeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageReductionModeNV.html) - Specify the coverage reduction mode
///# C Specifications
///Possible values of
///[`PipelineCoverageReductionStateCreateInfoNV::coverage_reduction_mode`],
///specifying how color sample coverage is generated from pixel coverage, are:
///```c
///// Provided by VK_NV_coverage_reduction_mode
///typedef enum VkCoverageReductionModeNV {
///    VK_COVERAGE_REDUCTION_MODE_MERGE_NV = 0,
///    VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV = 1,
///} VkCoverageReductionModeNV;
///```
///# Description
/// - [`COVERAGE_REDUCTION_MODE_MERGE`] specifies that each color
///sample will be associated with an implementation-dependent subset of
///samples in the pixel coverage.
///If any of those associated samples are covered, the color sample is
///covered.
/// - [`COVERAGE_REDUCTION_MODE_TRUNCATE`] specifies that for color
///samples present in the color attachments, a color sample is covered if
///the pixel coverage sample with the same
///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)i is
///covered; other pixel coverage samples are discarded.
///# Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`FramebufferMixedSamplesCombinationNV`]
/// - [`PipelineCoverageReductionStateCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCoverageReductionModeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CoverageReductionModeNV(i32);
impl const Default for CoverageReductionModeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for CoverageReductionModeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("CoverageReductionModeNV")
            .field(match *self {
                Self::COVERAGE_REDUCTION_MODE_MERGE => &"COVERAGE_REDUCTION_MODE_MERGE",
                Self::COVERAGE_REDUCTION_MODE_TRUNCATE => &"COVERAGE_REDUCTION_MODE_TRUNCATE",
                other => unreachable!("invalid value for `CoverageReductionModeNV`: {:?}", other),
            })
            .finish()
    }
}
impl CoverageReductionModeNV {
    ///[`COVERAGE_REDUCTION_MODE_MERGE`] specifies that each color
    ///sample will be associated with an implementation-dependent subset of
    ///samples in the pixel coverage.
    ///If any of those associated samples are covered, the color sample is
    ///covered.
    pub const COVERAGE_REDUCTION_MODE_MERGE: Self = Self(0);
    ///[`COVERAGE_REDUCTION_MODE_TRUNCATE`] specifies that for color
    ///samples present in the color attachments, a color sample is covered if
    ///the pixel coverage sample with the same
    ///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)i is
    ///covered; other pixel coverage samples are discarded.
    pub const COVERAGE_REDUCTION_MODE_TRUNCATE: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
