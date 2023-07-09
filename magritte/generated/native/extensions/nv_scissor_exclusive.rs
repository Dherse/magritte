use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Rect2D, StructureType};
#[doc(alias = "VkPhysicalDeviceExclusiveScissorFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "exclusiveScissor")]
    pub exclusive_scissor: Bool32,
}
impl Default for PhysicalDeviceExclusiveScissorFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExclusiveScissorFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            exclusive_scissor: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineViewportExclusiveScissorStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "exclusiveScissorCount")]
    pub exclusive_scissor_count: u32,
    #[doc(alias = "pExclusiveScissors")]
    pub exclusive_scissors: *const Rect2D,
}
impl Default for PipelineViewportExclusiveScissorStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineViewportExclusiveScissorStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            exclusive_scissor_count: unsafe { std::mem::zeroed() },
            exclusive_scissors: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_scissor_exclusive::{
    NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME, NV_SCISSOR_EXCLUSIVE_SPEC_VERSION,
};
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
pub type FNCmdSetExclusiveScissorNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const Rect2D,
);
