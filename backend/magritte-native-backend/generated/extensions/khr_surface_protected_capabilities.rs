use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkSurfaceProtectedCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "supportsProtected")]
    supports_protected: Bool32,
}
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_surface_protected_capabilities");
