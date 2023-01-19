//!# [VK_KHR_shader_subgroup_extended_types](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_subgroup_extended_types.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_shader_subgroup_extended_types/VK_KHR_shader_subgroup_extended_types.md")]
use crate::{cstr, vulkan1_2::PhysicalDeviceShaderSubgroupExtendedTypesFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`]
#[doc(alias = "VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR")]
pub type PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR = PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_shader_subgroup_extended_types");
