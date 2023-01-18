use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, DeviceSize, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceRobustness2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "robustBufferAccess2")]
    robust_buffer_access2: Bool32,
    #[doc(alias = "robustImageAccess2")]
    robust_image_access2: Bool32,
    #[doc(alias = "nullDescriptor")]
    null_descriptor: Bool32,
}
#[doc(alias = "VkPhysicalDeviceRobustness2PropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "robustStorageBufferAccessSizeAlignment")]
    robust_storage_buffer_access_size_alignment: DeviceSize,
    #[doc(alias = "robustUniformBufferAccessSizeAlignment")]
    robust_uniform_buffer_access_size_alignment: DeviceSize,
}
#[doc(alias = "VK_EXT_ROBUSTNESS_2_SPEC_VERSION")]
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_ROBUSTNESS_2_EXTENSION_NAME")]
pub const EXT_ROBUSTNESS_2_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_robustness2");
