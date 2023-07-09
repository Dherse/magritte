pub use crate::common::extensions::nv_clip_space_w_scaling::ViewportWScalingNV;
use crate::native::vulkan1_0::{BaseInStructure, Bool32, CommandBuffer, StructureType};
#[doc(alias = "VkPipelineViewportWScalingStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "viewportWScalingEnable")]
    pub viewport_w_scaling_enable: Bool32,
    #[doc(alias = "viewportCount")]
    pub viewport_count: u32,
    #[doc(alias = "pViewportWScalings")]
    pub viewport_w_scalings: *const ViewportWScalingNV,
}
impl Default for PipelineViewportWScalingStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineViewportWScalingStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            viewport_w_scaling_enable: unsafe { std::mem::zeroed() },
            viewport_count: unsafe { std::mem::zeroed() },
            viewport_w_scalings: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_clip_space_w_scaling::{
    NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME, NV_CLIP_SPACE_W_SCALING_SPEC_VERSION,
};
#[doc(alias = "vkCmdSetViewportWScalingNV")]
pub type FNCmdSetViewportWScalingNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_w_scalings: *const ViewportWScalingNV,
);
