use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceCornerSampledImageFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "cornerSampledImage")]
    pub corner_sampled_image: Bool32,
}
impl Default for PhysicalDeviceCornerSampledImageFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceCornerSampledImageFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            corner_sampled_image: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_corner_sampled_image::{
    NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME, NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION,
};
