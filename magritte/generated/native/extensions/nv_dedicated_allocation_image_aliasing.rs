use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "dedicatedAllocationImageAliasing")]
    pub dedicated_allocation_image_aliasing: Bool32,
}
impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            dedicated_allocation_image_aliasing: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_dedicated_allocation_image_aliasing::{
    NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME, NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION,
};
