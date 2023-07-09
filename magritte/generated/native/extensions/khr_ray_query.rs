use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceRayQueryFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayQueryFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "rayQuery")]
    pub ray_query: Bool32,
}
impl Default for PhysicalDeviceRayQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRayQueryFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            ray_query: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_ray_query::{KHR_RAY_QUERY_EXTENSION_NAME, KHR_RAY_QUERY_SPEC_VERSION};
