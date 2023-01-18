use crate::{
    cstr,
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkDisplayNativeHdrSurfaceCapabilitiesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "localDimmingSupport")]
    local_dimming_support: Bool32,
}
#[doc(alias = "VkSwapchainDisplayNativeHdrCreateInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "localDimmingEnable")]
    local_dimming_enable: Bool32,
}
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION")]
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME")]
pub const AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_display_native_hdr");
#[doc(alias = "vkSetLocalDimmingAMD")]
pub type FNSetLocalDimmingAmd =
    unsafe extern "system" fn(device: Device, swap_chain: SwapchainKHR, local_dimming_enable: Bool32);
