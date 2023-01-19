//!# [VK_NV_fragment_coverage_to_color](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_coverage_to_color.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_fragment_coverage_to_color/VK_NV_fragment_coverage_to_color.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPipelineCoverageToColorStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_coverage_to_color/VkPipelineCoverageToColorStateCreateInfoNV.md")]
#[doc(alias = "VkPipelineCoverageToColorStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineCoverageToColorStateCreateFlagsNV,
    #[doc(alias = "coverageToColorEnable")]
    coverage_to_color_enable: Bool32,
    #[doc(alias = "coverageToColorLocation")]
    coverage_to_color_location: u32,
}
#[doc(alias = "VkPipelineCoverageToColorStateCreateFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCoverageToColorStateCreateFlagsNV(u32);
impl PipelineCoverageToColorStateCreateFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_fragment_coverage_to_color");
