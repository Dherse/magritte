//!# [VK_KHR_ray_query](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_query.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_ray_query/VK_KHR_ray_query.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceRayQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_ray_query/VkPhysicalDeviceRayQueryFeaturesKHR.md")]
#[doc(alias = "VkPhysicalDeviceRayQueryFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayQueryFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "rayQuery")]
    ray_query: Bool32,
}
#[doc(alias = "VK_KHR_RAY_QUERY_SPEC_VERSION")]
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_RAY_QUERY_EXTENSION_NAME")]
pub const KHR_RAY_QUERY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_ray_query");
