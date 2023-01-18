use crate::{cstr, vulkan1_2::PhysicalDeviceShaderFloat16Int8Features};
use std::ffi::CStr;
///See [`PhysicalDeviceShaderFloat16Int8Features`]
#[doc(alias = "VkPhysicalDeviceShaderFloat16Int8FeaturesKHR")]
pub type PhysicalDeviceShaderFloat16Int8FeaturesKHR = PhysicalDeviceShaderFloat16Int8Features;
///See [`PhysicalDeviceShaderFloat16Int8Features`]
#[doc(alias = "VkPhysicalDeviceFloat16Int8FeaturesKHR")]
pub type PhysicalDeviceFloat16Int8FeaturesKHR = PhysicalDeviceShaderFloat16Int8Features;
#[doc(alias = "VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT16_INT8_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_float16_int8");
