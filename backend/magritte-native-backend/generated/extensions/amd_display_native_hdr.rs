//!# [VK_AMD_display_native_hdr](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_display_native_hdr.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_display_native_hdr/VK_AMD_display_native_hdr.md")]
use crate::{
    cstr,
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, StructureType},
};
use std::ffi::CStr;
///# [VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_display_native_hdr/VkDisplayNativeHdrSurfaceCapabilitiesAMD.md")]
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
///# [VkSwapchainDisplayNativeHdrCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_display_native_hdr/VkSwapchainDisplayNativeHdrCreateInfoAMD.md")]
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
///# [vkSetLocalDimmingAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_display_native_hdr/vkSetLocalDimmingAMD.md")]
#[doc(alias = "vkSetLocalDimmingAMD")]
pub type FNSetLocalDimmingAmd =
    unsafe extern "system" fn(device: Device, swap_chain: SwapchainKHR, local_dimming_enable: Bool32);
