use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkIOSSurfaceCreateFlagsMVK")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IosSurfaceCreateFlagsMVK(u32);
impl IosSurfaceCreateFlagsMVK {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for IosSurfaceCreateFlagsMVK {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for IosSurfaceCreateFlagsMVK {
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
unsafe impl crate::conv::FromLowLevel for IosSurfaceCreateFlagsMVK {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_MVK_IOS_SURFACE_SPEC_VERSION")]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_MVK_IOS_SURFACE_EXTENSION_NAME")]
pub const MVK_IOS_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_MVK_ios_surface");
