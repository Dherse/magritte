use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRgba10x6FormatsFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "formatRgba10x6WithoutYCbCrSampler")]
    pub format_rgba10x6_without_y_cb_cr_sampler: Bool32,
}
impl Default for PhysicalDeviceRgba10x6FormatsFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRgba10x6FormatsFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            format_rgba10x6_without_y_cb_cr_sampler: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_rgba10x6_formats::{
    EXT_RGBA10X6_FORMATS_EXTENSION_NAME, EXT_RGBA10X6_FORMATS_SPEC_VERSION,
};
