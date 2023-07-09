use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DescriptorType, StructureType};
#[doc(alias = "VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "mutableDescriptorType")]
    pub mutable_descriptor_type: Bool32,
}
impl Default for PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMutableDescriptorTypeFeaturesValve,
            p_next: unsafe { std::mem::zeroed() },
            mutable_descriptor_type: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMutableDescriptorTypeListVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MutableDescriptorTypeListVALVE {
    #[doc(alias = "descriptorTypeCount")]
    pub descriptor_type_count: u32,
    #[doc(alias = "pDescriptorTypes")]
    pub descriptor_types: *const DescriptorType,
}
impl Default for MutableDescriptorTypeListVALVE {
    fn default() -> Self {
        Self {
            descriptor_type_count: unsafe { std::mem::zeroed() },
            descriptor_types: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMutableDescriptorTypeCreateInfoVALVE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoVALVE {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "mutableDescriptorTypeListCount")]
    pub mutable_descriptor_type_list_count: u32,
    #[doc(alias = "pMutableDescriptorTypeLists")]
    pub mutable_descriptor_type_lists: *const MutableDescriptorTypeListVALVE,
}
impl Default for MutableDescriptorTypeCreateInfoVALVE {
    fn default() -> Self {
        Self {
            s_type: StructureType::MutableDescriptorTypeCreateInfoValve,
            p_next: unsafe { std::mem::zeroed() },
            mutable_descriptor_type_list_count: unsafe { std::mem::zeroed() },
            mutable_descriptor_type_lists: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::valve_mutable_descriptor_type::{
    VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME, VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION,
};
