use crate::native::vulkan1_0::{BaseOutStructure, Bool32, ImageViewType, StructureType};
#[doc(alias = "VkPhysicalDeviceImageViewImageFormatInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "imageViewType")]
    pub image_view_type: ImageViewType,
}
impl Default for PhysicalDeviceImageViewImageFormatInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceImageViewImageFormatInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            image_view_type: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFilterCubicImageViewImageFormatPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "filterCubic")]
    pub filter_cubic: Bool32,
    #[doc(alias = "filterCubicMinmax")]
    pub filter_cubic_minmax: Bool32,
}
impl Default for FilterCubicImageViewImageFormatPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::FilterCubicImageViewImageFormatPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            filter_cubic: unsafe { std::mem::zeroed() },
            filter_cubic_minmax: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_filter_cubic::{EXT_FILTER_CUBIC_EXTENSION_NAME, EXT_FILTER_CUBIC_SPEC_VERSION};
