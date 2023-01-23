//!# [VK_EXT_fragment_density_map](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_density_map.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_fragment_density_map/VK_EXT_fragment_density_map.md")]
use crate::{
    cstr,
    vulkan1_0::{AttachmentReference, BaseInStructure, BaseOutStructure, Bool32, Extent2D, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceFragmentDensityMapFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_fragment_density_map/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentDensityMap")]
    fragment_density_map: Bool32,
    #[doc(alias = "fragmentDensityMapDynamic")]
    fragment_density_map_dynamic: Bool32,
    #[doc(alias = "fragmentDensityMapNonSubsampledImages")]
    fragment_density_map_non_subsampled_images: Bool32,
}
///# [VkPhysicalDeviceFragmentDensityMapPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_fragment_density_map/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.md")]
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "minFragmentDensityTexelSize")]
    min_fragment_density_texel_size: Extent2D,
    #[doc(alias = "maxFragmentDensityTexelSize")]
    max_fragment_density_texel_size: Extent2D,
    #[doc(alias = "fragmentDensityInvocations")]
    fragment_density_invocations: Bool32,
}
///# [VkRenderPassFragmentDensityMapCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_fragment_density_map/VkRenderPassFragmentDensityMapCreateInfoEXT.md")]
#[doc(alias = "VkRenderPassFragmentDensityMapCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "fragmentDensityMapAttachment")]
    fragment_density_map_attachment: AttachmentReference,
}
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_fragment_density_map");
