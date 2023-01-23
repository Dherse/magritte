//!# [VK_AMD_device_coherent_memory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_device_coherent_memory.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_device_coherent_memory/VK_AMD_device_coherent_memory.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceCoherentMemoryFeaturesAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_device_coherent_memory/VkPhysicalDeviceCoherentMemoryFeaturesAMD.md")]
#[doc(alias = "VkPhysicalDeviceCoherentMemoryFeaturesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceCoherentMemory")]
    device_coherent_memory: Bool32,
}
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION")]
pub const AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME")]
pub const AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_device_coherent_memory");
