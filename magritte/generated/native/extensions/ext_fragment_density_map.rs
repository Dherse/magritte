use crate::native::vulkan1_0::{
    AttachmentReference, BaseInStructure, BaseOutStructure, Bool32, Extent2D, StructureType,
};
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentDensityMap")]
    pub fragment_density_map: Bool32,
    #[doc(alias = "fragmentDensityMapDynamic")]
    pub fragment_density_map_dynamic: Bool32,
    #[doc(alias = "fragmentDensityMapNonSubsampledImages")]
    pub fragment_density_map_non_subsampled_images: Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentDensityMapFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            fragment_density_map: unsafe { std::mem::zeroed() },
            fragment_density_map_dynamic: unsafe { std::mem::zeroed() },
            fragment_density_map_non_subsampled_images: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minFragmentDensityTexelSize")]
    pub min_fragment_density_texel_size: Extent2D,
    #[doc(alias = "maxFragmentDensityTexelSize")]
    pub max_fragment_density_texel_size: Extent2D,
    #[doc(alias = "fragmentDensityInvocations")]
    pub fragment_density_invocations: Bool32,
}
impl Default for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentDensityMapPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            min_fragment_density_texel_size: unsafe { std::mem::zeroed() },
            max_fragment_density_texel_size: unsafe { std::mem::zeroed() },
            fragment_density_invocations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderPassFragmentDensityMapCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "fragmentDensityMapAttachment")]
    pub fragment_density_map_attachment: AttachmentReference,
}
impl Default for RenderPassFragmentDensityMapCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderPassFragmentDensityMapCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            fragment_density_map_attachment: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_fragment_density_map::{
    EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME, EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION,
};
