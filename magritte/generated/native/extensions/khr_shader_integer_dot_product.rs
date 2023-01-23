//!# [VK_KHR_shader_integer_dot_product](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_integer_dot_product.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_shader_integer_dot_product/VK_KHR_shader_integer_dot_product.md")]
use crate::{
    cstr,
    vulkan1_3::{PhysicalDeviceShaderIntegerDotProductFeatures, PhysicalDeviceShaderIntegerDotProductProperties},
};
use std::ffi::CStr;
///See [`PhysicalDeviceShaderIntegerDotProductFeatures`]
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR")]
pub type PhysicalDeviceShaderIntegerDotProductFeaturesKHR = PhysicalDeviceShaderIntegerDotProductFeatures;
///See [`PhysicalDeviceShaderIntegerDotProductProperties`]
#[doc(alias = "VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR")]
pub type PhysicalDeviceShaderIntegerDotProductPropertiesKHR = PhysicalDeviceShaderIntegerDotProductProperties;
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_integer_dot_product");
