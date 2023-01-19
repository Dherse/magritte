//!# [VK_EXT_scalar_block_layout](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_scalar_block_layout.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_scalar_block_layout/VK_EXT_scalar_block_layout.md")]
use crate::{cstr, vulkan1_2::PhysicalDeviceScalarBlockLayoutFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceScalarBlockLayoutFeatures`]
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeaturesEXT")]
pub type PhysicalDeviceScalarBlockLayoutFeaturesEXT = PhysicalDeviceScalarBlockLayoutFeatures;
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION")]
pub const EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME")]
pub const EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_scalar_block_layout");
