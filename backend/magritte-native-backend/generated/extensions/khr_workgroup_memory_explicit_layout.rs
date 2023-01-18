use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "workgroupMemoryExplicitLayout")]
    workgroup_memory_explicit_layout: Bool32,
    #[doc(alias = "workgroupMemoryExplicitLayoutScalarBlockLayout")]
    workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
    #[doc(alias = "workgroupMemoryExplicitLayout8BitAccess")]
    workgroup_memory_explicit_layout8_bit_access: Bool32,
    #[doc(alias = "workgroupMemoryExplicitLayout16BitAccess")]
    workgroup_memory_explicit_layout16_bit_access: Bool32,
}
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_workgroup_memory_explicit_layout");
