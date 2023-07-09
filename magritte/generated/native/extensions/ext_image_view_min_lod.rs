use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceImageViewMinLodFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minLod")]
    pub min_lod: Bool32,
}
impl Default for PhysicalDeviceImageViewMinLodFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceImageViewMinLodFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            min_lod: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageViewMinLodCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "minLod")]
    pub min_lod: f32,
}
impl Default for ImageViewMinLodCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageViewMinLodCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            min_lod: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_image_view_min_lod::{
    EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME, EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION,
};
