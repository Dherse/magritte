use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDevice4444FormatsFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "formatA4R4G4B4")]
    pub format_a4r4g4b4: Bool32,
    #[doc(alias = "formatA4B4G4R4")]
    pub format_a4b4g4r4: Bool32,
}
impl Default for PhysicalDevice4444FormatsFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevice4444FormatsFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            format_a4r4g4b4: unsafe { std::mem::zeroed() },
            format_a4b4g4r4: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_4444_formats::{EXT_4444_FORMATS_EXTENSION_NAME, EXT_4444_FORMATS_SPEC_VERSION};
