//!# [VK_EXT_shader_demote_to_helper_invocation](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_demote_to_helper_invocation.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_shader_demote_to_helper_invocation/VK_EXT_shader_demote_to_helper_invocation.md")]
use crate::{cstr, vulkan1_3::PhysicalDeviceShaderDemoteToHelperInvocationFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceShaderDemoteToHelperInvocationFeatures`]
#[doc(alias = "VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT")]
pub type PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT = PhysicalDeviceShaderDemoteToHelperInvocationFeatures;
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME: &'static CStr =
    cstr!("VK_EXT_shader_demote_to_helper_invocation");
