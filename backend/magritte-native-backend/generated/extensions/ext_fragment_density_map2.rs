use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentDensityMapDeferred")]
    fragment_density_map_deferred: Bool32,
}
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2PropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "subsampledLoads")]
    subsampled_loads: Bool32,
    #[doc(alias = "subsampledCoarseReconstructionEarlyAccess")]
    subsampled_coarse_reconstruction_early_access: Bool32,
    #[doc(alias = "maxSubsampledArrayLayers")]
    max_subsampled_array_layers: u32,
    #[doc(alias = "maxDescriptorSetSubsampledSamplers")]
    max_descriptor_set_subsampled_samplers: u32,
}
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_fragment_density_map2");
