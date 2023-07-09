pub use crate::common::extensions::nv_viewport_swizzle::ViewportSwizzleNV;
use crate::native::vulkan1_0::{BaseInStructure, StructureType};
#[doc(alias = "VkPipelineViewportSwizzleStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    #[doc(alias = "viewportCount")]
    pub viewport_count: u32,
    #[doc(alias = "pViewportSwizzles")]
    pub viewport_swizzles: *const ViewportSwizzleNV,
}
impl Default for PipelineViewportSwizzleStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineViewportSwizzleStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            viewport_count: unsafe { std::mem::zeroed() },
            viewport_swizzles: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_viewport_swizzle::{
    PipelineViewportSwizzleStateCreateFlagsNV, ViewportCoordinateSwizzleNV, NV_VIEWPORT_SWIZZLE_EXTENSION_NAME,
    NV_VIEWPORT_SWIZZLE_SPEC_VERSION,
};
