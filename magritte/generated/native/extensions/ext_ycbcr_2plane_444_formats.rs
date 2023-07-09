use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "ycbcr2plane444Formats")]
    pub ycbcr2plane444_formats: Bool32,
}
impl Default for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceYcbcr2Plane444FormatsFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            ycbcr2plane444_formats: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_ycbcr_2plane_444_formats::{
    EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME, EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION,
};
