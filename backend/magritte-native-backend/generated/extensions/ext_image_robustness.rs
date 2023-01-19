//!# [VK_EXT_image_robustness](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_robustness.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_image_robustness/VK_EXT_image_robustness.md")]
use crate::{cstr, vulkan1_3::PhysicalDeviceImageRobustnessFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceImageRobustnessFeatures`]
#[doc(alias = "VkPhysicalDeviceImageRobustnessFeaturesEXT")]
pub type PhysicalDeviceImageRobustnessFeaturesEXT = PhysicalDeviceImageRobustnessFeatures;
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION")]
pub const EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME")]
pub const EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_image_robustness");
