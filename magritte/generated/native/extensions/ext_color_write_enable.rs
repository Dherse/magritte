//!# [VK_EXT_color_write_enable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_color_write_enable.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_color_write_enable/VK_EXT_color_write_enable.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceColorWriteEnableFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_color_write_enable/VkPhysicalDeviceColorWriteEnableFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceColorWriteEnableFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "colorWriteEnable")]
    color_write_enable: Bool32,
}
///# [VkPipelineColorWriteCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_color_write_enable/VkPipelineColorWriteCreateInfoEXT.md")]
#[doc(alias = "VkPipelineColorWriteCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "attachmentCount")]
    attachment_count: u32,
    #[doc(alias = "pColorWriteEnables")]
    color_write_enables: *const Bool32,
}
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION")]
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME")]
pub const EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_color_write_enable");
///# [vkCmdSetColorWriteEnableEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteEnableEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_color_write_enable/vkCmdSetColorWriteEnableEXT.md")]
#[doc(alias = "vkCmdSetColorWriteEnableEXT")]
pub type FNCmdSetColorWriteEnableExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_color_write_enables: *const Bool32,
);
