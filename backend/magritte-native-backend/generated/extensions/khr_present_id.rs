use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDevicePresentIdFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentIdFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "presentId")]
    present_id: Bool32,
}
#[doc(alias = "VkPresentIdKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentIdKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    swapchain_count: u32,
    #[doc(alias = "pPresentIds")]
    present_ids: *const u64,
}
#[doc(alias = "VK_KHR_PRESENT_ID_SPEC_VERSION")]
pub const KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PRESENT_ID_EXTENSION_NAME")]
pub const KHR_PRESENT_ID_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_present_id");
