use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ClearColorValue, Format, StructureType};
#[doc(alias = "VkSamplerCustomBorderColorCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "customBorderColor")]
    pub custom_border_color: ClearColorValue,
    pub format: Format,
}
impl Default for SamplerCustomBorderColorCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SamplerCustomBorderColorCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            custom_border_color: unsafe { std::mem::zeroed() },
            format: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceCustomBorderColorPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxCustomBorderColorSamplers")]
    pub max_custom_border_color_samplers: u32,
}
impl Default for PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceCustomBorderColorPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            max_custom_border_color_samplers: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceCustomBorderColorFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "customBorderColors")]
    pub custom_border_colors: Bool32,
    #[doc(alias = "customBorderColorWithoutFormat")]
    pub custom_border_color_without_format: Bool32,
}
impl Default for PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceCustomBorderColorFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            custom_border_colors: unsafe { std::mem::zeroed() },
            custom_border_color_without_format: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_custom_border_color::{
    EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME, EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION,
};
