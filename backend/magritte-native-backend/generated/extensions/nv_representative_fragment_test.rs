//!# [VK_NV_representative_fragment_test](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_representative_fragment_test.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_representative_fragment_test/VK_NV_representative_fragment_test.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_representative_fragment_test/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.md")]
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
///# [VkPipelineRepresentativeFragmentTestStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_representative_fragment_test/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.md")]
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
