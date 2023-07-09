use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentDensityMapDeferred")]
    pub fragment_density_map_deferred: Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMap2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentDensityMap2FeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            fragment_density_map_deferred: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentDensityMap2PropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "subsampledLoads")]
    pub subsampled_loads: Bool32,
    #[doc(alias = "subsampledCoarseReconstructionEarlyAccess")]
    pub subsampled_coarse_reconstruction_early_access: Bool32,
    #[doc(alias = "maxSubsampledArrayLayers")]
    pub max_subsampled_array_layers: u32,
    #[doc(alias = "maxDescriptorSetSubsampledSamplers")]
    pub max_descriptor_set_subsampled_samplers: u32,
}
impl Default for PhysicalDeviceFragmentDensityMap2PropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentDensityMap2PropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            subsampled_loads: unsafe { std::mem::zeroed() },
            subsampled_coarse_reconstruction_early_access: unsafe { std::mem::zeroed() },
            max_subsampled_array_layers: unsafe { std::mem::zeroed() },
            max_descriptor_set_subsampled_samplers: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_fragment_density_map2::{
    EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME, EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION,
};
