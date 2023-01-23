//!# [VK_KHR_shader_atomic_int64](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_atomic_int64.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_shader_atomic_int64/VK_KHR_shader_atomic_int64.md")]
use crate::{cstr, vulkan1_2::PhysicalDeviceShaderAtomicInt64Features};
use std::ffi::CStr;
///See [`PhysicalDeviceShaderAtomicInt64Features`]
#[doc(alias = "VkPhysicalDeviceShaderAtomicInt64FeaturesKHR")]
pub type PhysicalDeviceShaderAtomicInt64FeaturesKHR = PhysicalDeviceShaderAtomicInt64Features;
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION")]
pub const KHR_SHADER_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME")]
pub const KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_atomic_int64");
