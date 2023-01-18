use crate::{cstr, vulkan1_3::PhysicalDeviceShaderTerminateInvocationFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceShaderTerminateInvocationFeatures`]
#[doc(alias = "VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR")]
pub type PhysicalDeviceShaderTerminateInvocationFeaturesKHR = PhysicalDeviceShaderTerminateInvocationFeatures;
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION")]
pub const KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME")]
pub const KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_terminate_invocation");
