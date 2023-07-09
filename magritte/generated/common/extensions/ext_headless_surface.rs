use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkHeadlessSurfaceCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeadlessSurfaceCreateFlagsEXT(u32);
impl HeadlessSurfaceCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for HeadlessSurfaceCreateFlagsEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for HeadlessSurfaceCreateFlagsEXT {
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
unsafe impl crate::conv::FromLowLevel for HeadlessSurfaceCreateFlagsEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_SPEC_VERSION")]
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME")]
pub const EXT_HEADLESS_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_headless_surface");
