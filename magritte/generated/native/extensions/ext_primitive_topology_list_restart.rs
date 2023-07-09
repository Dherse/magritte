use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "primitiveTopologyListRestart")]
    pub primitive_topology_list_restart: Bool32,
    #[doc(alias = "primitiveTopologyPatchListRestart")]
    pub primitive_topology_patch_list_restart: Bool32,
}
impl Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePrimitiveTopologyListRestartFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            primitive_topology_list_restart: unsafe { std::mem::zeroed() },
            primitive_topology_patch_list_restart: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_primitive_topology_list_restart::{
    EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME, EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION,
};
