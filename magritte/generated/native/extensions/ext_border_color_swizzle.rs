use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ComponentMapping, StructureType};
#[doc(alias = "VkSamplerBorderColorComponentMappingCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub components: ComponentMapping,
    pub srgb: Bool32,
}
impl Default for SamplerBorderColorComponentMappingCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SamplerBorderColorComponentMappingCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            components: unsafe { std::mem::zeroed() },
            srgb: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceBorderColorSwizzleFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "borderColorSwizzle")]
    pub border_color_swizzle: Bool32,
    #[doc(alias = "borderColorSwizzleFromImage")]
    pub border_color_swizzle_from_image: Bool32,
}
impl Default for PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceBorderColorSwizzleFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            border_color_swizzle: unsafe { std::mem::zeroed() },
            border_color_swizzle_from_image: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_border_color_swizzle::{
    EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME, EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION,
};
