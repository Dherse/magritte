use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "representativeFragmentTest")]
    pub representative_fragment_test: Bool32,
}
impl Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRepresentativeFragmentTestFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            representative_fragment_test: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRepresentativeFragmentTestStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "representativeFragmentTestEnable")]
    pub representative_fragment_test_enable: Bool32,
}
impl Default for PipelineRepresentativeFragmentTestStateCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRepresentativeFragmentTestStateCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            representative_fragment_test_enable: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_representative_fragment_test::{
    NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME, NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION,
};
