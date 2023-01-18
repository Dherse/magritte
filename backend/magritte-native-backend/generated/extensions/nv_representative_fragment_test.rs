use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "representativeFragmentTest")]
    representative_fragment_test: Bool32,
}
#[doc(alias = "VkPipelineRepresentativeFragmentTestStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "representativeFragmentTestEnable")]
    representative_fragment_test_enable: Bool32,
}
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_representative_fragment_test");
