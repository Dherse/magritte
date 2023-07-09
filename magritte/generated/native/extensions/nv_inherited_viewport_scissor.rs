use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType, Viewport};
#[doc(alias = "VkPhysicalDeviceInheritedViewportScissorFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "inheritedViewportScissor2D")]
    pub inherited_viewport_scissor2_d: Bool32,
}
impl Default for PhysicalDeviceInheritedViewportScissorFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceInheritedViewportScissorFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            inherited_viewport_scissor2_d: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCommandBufferInheritanceViewportScissorInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceViewportScissorInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "viewportScissor2D")]
    pub viewport_scissor2_d: Bool32,
    #[doc(alias = "viewportDepthCount")]
    pub viewport_depth_count: u32,
    #[doc(alias = "pViewportDepths")]
    pub viewport_depths: *const Viewport,
}
impl Default for CommandBufferInheritanceViewportScissorInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::CommandBufferInheritanceViewportScissorInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            viewport_scissor2_d: unsafe { std::mem::zeroed() },
            viewport_depth_count: unsafe { std::mem::zeroed() },
            viewport_depths: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_inherited_viewport_scissor::{
    NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME, NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION,
};
