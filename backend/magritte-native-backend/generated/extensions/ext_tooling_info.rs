//!# [VK_EXT_tooling_info](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_tooling_info.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_tooling_info/VK_EXT_tooling_info.md")]
use crate::{
    cstr,
    vulkan1_3::{
        FNGetPhysicalDeviceToolProperties, PhysicalDeviceToolProperties, ToolPurposeFlagBits, ToolPurposeFlags,
    },
};
use std::ffi::CStr;
///See [`ToolPurposeFlags`]
#[doc(alias = "VkToolPurposeFlagsEXT")]
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
///See [`ToolPurposeFlagBits`]
#[doc(alias = "VkToolPurposeFlagBitsEXT")]
pub type ToolPurposeFlagBitsEXT = ToolPurposeFlagBits;
///See [`PhysicalDeviceToolProperties`]
#[doc(alias = "VkPhysicalDeviceToolPropertiesEXT")]
pub type PhysicalDeviceToolPropertiesEXT = PhysicalDeviceToolProperties;
#[doc(alias = "VK_EXT_TOOLING_INFO_SPEC_VERSION")]
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_TOOLING_INFO_EXTENSION_NAME")]
pub const EXT_TOOLING_INFO_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_tooling_info");
///See [`get_physical_device_tool_properties`]
#[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
pub type FNGetPhysicalDeviceToolPropertiesExt = FNGetPhysicalDeviceToolProperties;
