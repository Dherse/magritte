//!# [VK_EXT_primitive_topology_list_restart](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_primitive_topology_list_restart.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_primitive_topology_list_restart/VK_EXT_primitive_topology_list_restart.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_primitive_topology_list_restart/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "primitiveTopologyListRestart")]
    primitive_topology_list_restart: Bool32,
    #[doc(alias = "primitiveTopologyPatchListRestart")]
    primitive_topology_patch_list_restart: Bool32,
}
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME: &'static CStr =
    cstr!("VK_EXT_primitive_topology_list_restart");
