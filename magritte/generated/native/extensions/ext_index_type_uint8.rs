use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceIndexTypeUint8FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "indexTypeUint8")]
    pub index_type_uint8: Bool32,
}
impl Default for PhysicalDeviceIndexTypeUint8FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceIndexTypeUint8FeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            index_type_uint8: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_index_type_uint8::{
    EXT_INDEX_TYPE_UINT8_EXTENSION_NAME, EXT_INDEX_TYPE_UINT8_SPEC_VERSION,
};
