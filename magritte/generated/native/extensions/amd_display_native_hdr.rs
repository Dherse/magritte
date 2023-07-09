use crate::native::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, StructureType},
};
#[doc(alias = "VkDisplayNativeHdrSurfaceCapabilitiesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "localDimmingSupport")]
    pub local_dimming_support: Bool32,
}
impl Default for DisplayNativeHdrSurfaceCapabilitiesAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayNativeHdrSurfaceCapabilitiesAmd,
            p_next: unsafe { std::mem::zeroed() },
            local_dimming_support: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSwapchainDisplayNativeHdrCreateInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "localDimmingEnable")]
    pub local_dimming_enable: Bool32,
}
impl Default for SwapchainDisplayNativeHdrCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::SwapchainDisplayNativeHdrCreateInfoAmd,
            p_next: unsafe { std::mem::zeroed() },
            local_dimming_enable: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_display_native_hdr::{
    AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME, AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION,
};
#[doc(alias = "vkSetLocalDimmingAMD")]
pub type FNSetLocalDimmingAmd =
    unsafe extern "system" fn(device: Device, swap_chain: SwapchainKHR, local_dimming_enable: Bool32);
