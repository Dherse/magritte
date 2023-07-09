use crate::native::{
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, LogicOp, StructureType},
    vulkan1_3::{FNCmdSetDepthBiasEnable, FNCmdSetPrimitiveRestartEnable, FNCmdSetRasterizerDiscardEnable},
};
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "extendedDynamicState2")]
    pub extended_dynamic_state2: Bool32,
    #[doc(alias = "extendedDynamicState2LogicOp")]
    pub extended_dynamic_state2_logic_op: Bool32,
    #[doc(alias = "extendedDynamicState2PatchControlPoints")]
    pub extended_dynamic_state2_patch_control_points: Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceExtendedDynamicState2FeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            extended_dynamic_state2: unsafe { std::mem::zeroed() },
            extended_dynamic_state2_logic_op: unsafe { std::mem::zeroed() },
            extended_dynamic_state2_patch_control_points: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_extended_dynamic_state2::{
    EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME, EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION,
};
///See [`cmd_set_rasterizer_discard_enable`]
#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
pub type FNCmdSetRasterizerDiscardEnableExt = FNCmdSetRasterizerDiscardEnable;
///See [`cmd_set_depth_bias_enable`]
#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
pub type FNCmdSetDepthBiasEnableExt = FNCmdSetDepthBiasEnable;
///See [`cmd_set_primitive_restart_enable`]
#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
pub type FNCmdSetPrimitiveRestartEnableExt = FNCmdSetPrimitiveRestartEnable;
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
pub type FNCmdSetPatchControlPointsExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
#[doc(alias = "vkCmdSetLogicOpEXT")]
pub type FNCmdSetLogicOpExt = unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
