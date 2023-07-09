use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "perViewPositionAllComponents")]
    pub per_view_position_all_components: Bool32,
}
impl Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMultiviewPerViewAttributesPropertiesNvx,
            p_next: unsafe { std::mem::zeroed() },
            per_view_position_all_components: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nvx_multiview_per_view_attributes::{
    NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME, NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION,
};
