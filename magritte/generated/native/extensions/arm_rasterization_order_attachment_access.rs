use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "rasterizationOrderColorAttachmentAccess")]
    pub rasterization_order_color_attachment_access: Bool32,
    #[doc(alias = "rasterizationOrderDepthAttachmentAccess")]
    pub rasterization_order_depth_attachment_access: Bool32,
    #[doc(alias = "rasterizationOrderStencilAttachmentAccess")]
    pub rasterization_order_stencil_attachment_access: Bool32,
}
impl Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesArm,
            p_next: unsafe { std::mem::zeroed() },
            rasterization_order_color_attachment_access: unsafe { std::mem::zeroed() },
            rasterization_order_depth_attachment_access: unsafe { std::mem::zeroed() },
            rasterization_order_stencil_attachment_access: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::arm_rasterization_order_attachment_access::{
    PipelineColorBlendStateCreateFlagBits, PipelineDepthStencilStateCreateFlagBits,
    ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME, ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION,
};
