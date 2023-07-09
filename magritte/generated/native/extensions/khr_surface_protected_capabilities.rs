use crate::native::vulkan1_0::{BaseInStructure, Bool32, StructureType};
#[doc(alias = "VkSurfaceProtectedCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceProtectedCapabilitiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "supportsProtected")]
    pub supports_protected: Bool32,
}
impl Default for SurfaceProtectedCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::SurfaceProtectedCapabilitiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            supports_protected: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_surface_protected_capabilities::{
    KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME, KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION,
};
