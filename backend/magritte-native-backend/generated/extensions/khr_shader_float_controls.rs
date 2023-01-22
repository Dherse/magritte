//!# [VK_KHR_shader_float_controls](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_float_controls.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_shader_float_controls/VK_KHR_shader_float_controls.md")]
use crate::{
    cstr,
    vulkan1_2::{PhysicalDeviceFloatControlsProperties, ShaderFloatControlsIndependence},
};
use std::ffi::CStr;
///See [`ShaderFloatControlsIndependence`]
#[doc(alias = "VkShaderFloatControlsIndependenceKHR")]
pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;
///See [`PhysicalDeviceFloatControlsProperties`]
#[doc(alias = "VkPhysicalDeviceFloatControlsPropertiesKHR")]
pub type PhysicalDeviceFloatControlsPropertiesKHR = PhysicalDeviceFloatControlsProperties;
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_float_controls");