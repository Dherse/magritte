pub use crate::common::extensions::khr_shader_float_controls::{
    KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME, KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION,
};
use crate::vulkan1_2::{PhysicalDeviceFloatControlsProperties, ShaderFloatControlsIndependence};
#[doc(alias = "VkShaderFloatControlsIndependenceKHR")]
pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;
#[doc(alias = "VkPhysicalDeviceFloatControlsPropertiesKHR")]
pub type PhysicalDeviceFloatControlsPropertiesKHR = PhysicalDeviceFloatControlsProperties;
