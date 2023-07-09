use crate::cstr;
use std::ffi::CStr;
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
impl Default for PipelineCoverageToColorStateCreateFlagsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCoverageToColorStateCreateFlagsNV {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineCoverageToColorStateCreateFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_fragment_coverage_to_color");
