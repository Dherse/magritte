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
