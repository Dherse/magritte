use crate::{cstr, vulkan1_2::PhysicalDeviceUniformBufferStandardLayoutFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceUniformBufferStandardLayoutFeatures`]
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR")]
pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR = PhysicalDeviceUniformBufferStandardLayoutFeatures;
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_uniform_buffer_standard_layout");
