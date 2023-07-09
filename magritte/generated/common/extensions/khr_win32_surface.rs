use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkWin32SurfaceCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Win32SurfaceCreateFlagsKHR(u32);
impl Win32SurfaceCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for Win32SurfaceCreateFlagsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for Win32SurfaceCreateFlagsKHR {
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
unsafe impl crate::conv::FromLowLevel for Win32SurfaceCreateFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_KHR_WIN32_SURFACE_SPEC_VERSION")]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
#[doc(alias = "VK_KHR_WIN32_SURFACE_EXTENSION_NAME")]
pub const KHR_WIN32_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_win32_surface");
