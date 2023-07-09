use crate::native::{
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
    vulkan1_3::{
        FNCmdBindVertexBuffers2, FNCmdSetCullMode, FNCmdSetDepthBoundsTestEnable, FNCmdSetDepthCompareOp,
        FNCmdSetDepthTestEnable, FNCmdSetDepthWriteEnable, FNCmdSetFrontFace, FNCmdSetPrimitiveTopology,
        FNCmdSetScissorWithCount, FNCmdSetStencilOp, FNCmdSetStencilTestEnable, FNCmdSetViewportWithCount,
    },
};
#[doc(alias = "VkPhysicalDeviceExtendedDynamicStateFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "extendedDynamicState")]
    pub extended_dynamic_state: Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExtendedDynamicStateFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            extended_dynamic_state: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_extended_dynamic_state::{
    EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME, EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION,
};
///See [`cmd_set_cull_mode`]
#[doc(alias = "vkCmdSetCullModeEXT")]
pub type FNCmdSetCullModeExt = FNCmdSetCullMode;
///See [`cmd_set_front_face`]
#[doc(alias = "vkCmdSetFrontFaceEXT")]
pub type FNCmdSetFrontFaceExt = FNCmdSetFrontFace;
///See [`cmd_set_primitive_topology`]
#[doc(alias = "vkCmdSetPrimitiveTopologyEXT")]
pub type FNCmdSetPrimitiveTopologyExt = FNCmdSetPrimitiveTopology;
///See [`cmd_set_viewport_with_count`]
#[doc(alias = "vkCmdSetViewportWithCountEXT")]
pub type FNCmdSetViewportWithCountExt = FNCmdSetViewportWithCount;
///See [`cmd_set_scissor_with_count`]
#[doc(alias = "vkCmdSetScissorWithCountEXT")]
pub type FNCmdSetScissorWithCountExt = FNCmdSetScissorWithCount;
///See [`cmd_bind_vertex_buffers2`]
#[doc(alias = "vkCmdBindVertexBuffers2EXT")]
pub type FNCmdBindVertexBuffers2Ext = FNCmdBindVertexBuffers2;
///See [`cmd_set_depth_test_enable`]
#[doc(alias = "vkCmdSetDepthTestEnableEXT")]
pub type FNCmdSetDepthTestEnableExt = FNCmdSetDepthTestEnable;
///See [`cmd_set_depth_write_enable`]
#[doc(alias = "vkCmdSetDepthWriteEnableEXT")]
pub type FNCmdSetDepthWriteEnableExt = FNCmdSetDepthWriteEnable;
///See [`cmd_set_depth_compare_op`]
#[doc(alias = "vkCmdSetDepthCompareOpEXT")]
pub type FNCmdSetDepthCompareOpExt = FNCmdSetDepthCompareOp;
///See [`cmd_set_depth_bounds_test_enable`]
#[doc(alias = "vkCmdSetDepthBoundsTestEnableEXT")]
pub type FNCmdSetDepthBoundsTestEnableExt = FNCmdSetDepthBoundsTestEnable;
///See [`cmd_set_stencil_test_enable`]
#[doc(alias = "vkCmdSetStencilTestEnableEXT")]
pub type FNCmdSetStencilTestEnableExt = FNCmdSetStencilTestEnable;
///See [`cmd_set_stencil_op`]
#[doc(alias = "vkCmdSetStencilOpEXT")]
pub type FNCmdSetStencilOpExt = FNCmdSetStencilOp;
