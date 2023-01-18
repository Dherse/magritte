use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DescriptorType, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "mutableDescriptorType")]
    mutable_descriptor_type: Bool32,
}
#[doc(alias = "VkMutableDescriptorTypeListVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MutableDescriptorTypeListVALVE {
    #[doc(alias = "descriptorTypeCount")]
    descriptor_type_count: u32,
    #[doc(alias = "pDescriptorTypes")]
    descriptor_types: *const DescriptorType,
}
#[doc(alias = "VkMutableDescriptorTypeCreateInfoVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoVALVE {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "mutableDescriptorTypeListCount")]
    mutable_descriptor_type_list_count: u32,
    #[doc(alias = "pMutableDescriptorTypeLists")]
    mutable_descriptor_type_lists: *const MutableDescriptorTypeListVALVE,
}
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &'static CStr = cstr!("VK_VALVE_mutable_descriptor_type");
