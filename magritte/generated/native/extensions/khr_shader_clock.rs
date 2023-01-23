//!# [VK_KHR_shader_clock](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_clock.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_shader_clock/VK_KHR_shader_clock.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceShaderClockFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_shader_clock/VkPhysicalDeviceShaderClockFeaturesKHR.md")]
#[doc(alias = "VkPhysicalDeviceShaderClockFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSubgroupClock")]
    shader_subgroup_clock: Bool32,
    #[doc(alias = "shaderDeviceClock")]
    shader_device_clock: Bool32,
}
#[doc(alias = "VK_KHR_SHADER_CLOCK_SPEC_VERSION")]
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_CLOCK_EXTENSION_NAME")]
pub const KHR_SHADER_CLOCK_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_clock");
