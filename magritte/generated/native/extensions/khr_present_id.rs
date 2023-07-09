use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDevicePresentIdFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentIdFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "presentId")]
    pub present_id: Bool32,
}
impl Default for PhysicalDevicePresentIdFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePresentIdFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            present_id: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPresentIdKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentIdKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    pub swapchain_count: u32,
    #[doc(alias = "pPresentIds")]
    pub present_ids: *const u64,
}
impl Default for PresentIdKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PresentIdKhr,
            p_next: unsafe { std::mem::zeroed() },
            swapchain_count: unsafe { std::mem::zeroed() },
            present_ids: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_present_id::{KHR_PRESENT_ID_EXTENSION_NAME, KHR_PRESENT_ID_SPEC_VERSION};
