use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceYcbcrImageArraysFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "ycbcrImageArrays")]
    pub ycbcr_image_arrays: Bool32,
}
impl Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceYcbcrImageArraysFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            ycbcr_image_arrays: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_ycbcr_image_arrays::{
    EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME, EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION,
};
