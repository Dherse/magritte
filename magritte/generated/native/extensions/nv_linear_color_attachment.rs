use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceLinearColorAttachmentFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "linearColorAttachment")]
    pub linear_color_attachment: Bool32,
}
impl Default for PhysicalDeviceLinearColorAttachmentFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceLinearColorAttachmentFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            linear_color_attachment: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_linear_color_attachment::{
    NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME, NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION,
};
