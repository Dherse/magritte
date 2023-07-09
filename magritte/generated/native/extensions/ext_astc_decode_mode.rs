use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Format, StructureType};
#[doc(alias = "VkImageViewASTCDecodeModeEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewAstcDecodeModeEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "decodeMode")]
    pub decode_mode: Format,
}
impl Default for ImageViewAstcDecodeModeEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageViewAstcDecodeModeExt,
            p_next: unsafe { std::mem::zeroed() },
            decode_mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceASTCDecodeFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAstcDecodeFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "decodeModeSharedExponent")]
    pub decode_mode_shared_exponent: Bool32,
}
impl Default for PhysicalDeviceAstcDecodeFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceAstcDecodeFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            decode_mode_shared_exponent: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_astc_decode_mode::{
    EXT_ASTC_DECODE_MODE_EXTENSION_NAME, EXT_ASTC_DECODE_MODE_SPEC_VERSION,
};
