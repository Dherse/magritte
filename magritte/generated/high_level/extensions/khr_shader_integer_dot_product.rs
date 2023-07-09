pub use crate::common::extensions::khr_shader_integer_dot_product::{
    KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME, KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION,
};
use crate::vulkan1_3::{
    PhysicalDeviceShaderIntegerDotProductFeatures, PhysicalDeviceShaderIntegerDotProductProperties,
};
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR")]
pub type PhysicalDeviceShaderIntegerDotProductFeaturesKHR = PhysicalDeviceShaderIntegerDotProductFeatures;
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR")]
pub type PhysicalDeviceShaderIntegerDotProductPropertiesKHR = PhysicalDeviceShaderIntegerDotProductProperties;
