pub use crate::common::extensions::ext_tooling_info::{EXT_TOOLING_INFO_EXTENSION_NAME, EXT_TOOLING_INFO_SPEC_VERSION};
use crate::vulkan1_3::{PhysicalDeviceToolProperties, ToolPurposeFlagBits, ToolPurposeFlags};
#[doc(alias = "VkToolPurposeFlagsEXT")]
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
#[doc(alias = "VkToolPurposeFlagBitsEXT")]
pub type ToolPurposeFlagBitsEXT = ToolPurposeFlagBits;
#[doc(alias = "VkPhysicalDeviceToolPropertiesEXT")]
pub type PhysicalDeviceToolPropertiesEXT = PhysicalDeviceToolProperties;
