//!# [VK_NV_fragment_shader_barycentric](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_shader_barycentric.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shader_barycentric/VK_NV_fragment_shader_barycentric.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shader_barycentric/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentShaderBarycentric")]
    fragment_shader_barycentric: Bool32,
}
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_fragment_shader_barycentric");
