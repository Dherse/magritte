use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderImageFootprintFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "imageFootprint")]
    pub image_footprint: Bool32,
}
impl Default for PhysicalDeviceShaderImageFootprintFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderImageFootprintFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            image_footprint: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_shader_image_footprint::{
    NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME, NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION,
};
