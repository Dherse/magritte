use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DescriptorSet, DescriptorSetLayout, Device, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "descriptorSetHostMapping")]
    descriptor_set_host_mapping: Bool32,
}
#[doc(alias = "VkDescriptorSetBindingReferenceVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetBindingReferenceVALVE {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "descriptorSetLayout")]
    descriptor_set_layout: DescriptorSetLayout,
    binding: u32,
}
#[doc(alias = "VkDescriptorSetLayoutHostMappingInfoVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DescriptorSetLayoutHostMappingInfoVALVE {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "descriptorOffset")]
    descriptor_offset: usize,
    #[doc(alias = "descriptorSize")]
    descriptor_size: u32,
}
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME: &'static CStr =
    cstr!("VK_VALVE_descriptor_set_host_mapping");
#[doc(alias = "vkGetDescriptorSetLayoutHostMappingInfoVALVE")]
pub type FNGetDescriptorSetLayoutHostMappingInfoValve = unsafe extern "system" fn(
    device: Device,
    p_binding_reference: *const DescriptorSetBindingReferenceVALVE,
    p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
);
#[doc(alias = "vkGetDescriptorSetHostMappingVALVE")]
pub type FNGetDescriptorSetHostMappingValve =
    unsafe extern "system" fn(device: Device, descriptor_set: DescriptorSet, pp_data: *mut *mut std::ffi::c_void);
