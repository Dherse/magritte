//!# [VK_EXT_extended_dynamic_state2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state2.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_extended_dynamic_state2/VK_EXT_extended_dynamic_state2.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, LogicOp, StructureType},
    vulkan1_3::{FNCmdSetDepthBiasEnable, FNCmdSetPrimitiveRestartEnable, FNCmdSetRasterizerDiscardEnable},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceExtendedDynamicState2FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_extended_dynamic_state2/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceExtendedDynamicState2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "extendedDynamicState2")]
    extended_dynamic_state2: Bool32,
    #[doc(alias = "extendedDynamicState2LogicOp")]
    extended_dynamic_state2_logic_op: Bool32,
    #[doc(alias = "extendedDynamicState2PatchControlPoints")]
    extended_dynamic_state2_patch_control_points: Bool32,
}
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_extended_dynamic_state2");
///See [`cmd_set_rasterizer_discard_enable`]
#[doc(alias = "vkCmdSetRasterizerDiscardEnableEXT")]
pub type FNCmdSetRasterizerDiscardEnableExt = FNCmdSetRasterizerDiscardEnable;
///See [`cmd_set_depth_bias_enable`]
#[doc(alias = "vkCmdSetDepthBiasEnableEXT")]
pub type FNCmdSetDepthBiasEnableExt = FNCmdSetDepthBiasEnable;
///See [`cmd_set_primitive_restart_enable`]
#[doc(alias = "vkCmdSetPrimitiveRestartEnableEXT")]
pub type FNCmdSetPrimitiveRestartEnableExt = FNCmdSetPrimitiveRestartEnable;
///# [vkCmdSetPatchControlPointsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_extended_dynamic_state2/vkCmdSetPatchControlPointsEXT.md")]
#[doc(alias = "vkCmdSetPatchControlPointsEXT")]
pub type FNCmdSetPatchControlPointsExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
///# [vkCmdSetLogicOpEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_extended_dynamic_state2/vkCmdSetLogicOpEXT.md")]
#[doc(alias = "vkCmdSetLogicOpEXT")]
pub type FNCmdSetLogicOpExt = unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
