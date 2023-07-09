use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(u32);
impl PipelineRasterizationDepthClipStateCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRasterizationDepthClipStateCreateFlagsEXT {
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
unsafe impl crate::conv::FromLowLevel for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_depth_clip_enable");
