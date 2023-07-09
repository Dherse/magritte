use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VkRefreshCycleDurationGOOGLE")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    #[doc(alias = "refreshDuration")]
    pub refresh_duration: u64,
}
#[doc(alias = "VkPastPresentationTimingGOOGLE")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    #[doc(alias = "presentID")]
    pub present_id: u32,
    #[doc(alias = "desiredPresentTime")]
    pub desired_present_time: u64,
    #[doc(alias = "actualPresentTime")]
    pub actual_present_time: u64,
    #[doc(alias = "earliestPresentTime")]
    pub earliest_present_time: u64,
    #[doc(alias = "presentMargin")]
    pub present_margin: u64,
}
#[doc(alias = "VkPresentTimeGOOGLE")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentTimeGOOGLE {
    #[doc(alias = "presentID")]
    pub present_id: u32,
    #[doc(alias = "desiredPresentTime")]
    pub desired_present_time: u64,
}
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION")]
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME")]
pub const GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static CStr = cstr!("VK_GOOGLE_display_timing");
