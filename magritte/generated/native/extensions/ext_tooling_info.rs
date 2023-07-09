use crate::native::vulkan1_3::{
    FNGetPhysicalDeviceToolProperties, PhysicalDeviceToolProperties, ToolPurposeFlagBits, ToolPurposeFlags,
};
///See [`ToolPurposeFlags`]
#[doc(alias = "VkToolPurposeFlagsEXT")]
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
///See [`ToolPurposeFlagBits`]
#[doc(alias = "VkToolPurposeFlagBitsEXT")]
pub type ToolPurposeFlagBitsEXT = ToolPurposeFlagBits;
///See [`PhysicalDeviceToolProperties`]
#[doc(alias = "VkPhysicalDeviceToolPropertiesEXT")]
pub type PhysicalDeviceToolPropertiesEXT = PhysicalDeviceToolProperties;
pub use crate::common::extensions::ext_tooling_info::{EXT_TOOLING_INFO_EXTENSION_NAME, EXT_TOOLING_INFO_SPEC_VERSION};
///See [`get_physical_device_tool_properties`]
#[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
pub type FNGetPhysicalDeviceToolPropertiesExt = FNGetPhysicalDeviceToolProperties;
