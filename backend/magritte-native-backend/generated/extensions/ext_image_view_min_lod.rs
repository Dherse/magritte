//!# [VK_EXT_image_view_min_lod](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_view_min_lod.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_image_view_min_lod/VK_EXT_image_view_min_lod.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceImageViewMinLodFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_view_min_lod/VkPhysicalDeviceImageViewMinLodFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceImageViewMinLodFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "minLod")]
    min_lod: Bool32,
}
///# [VkImageViewMinLodCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_view_min_lod/VkImageViewMinLodCreateInfoEXT.md")]
#[doc(alias = "VkImageViewMinLodCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "minLod")]
    min_lod: f32,
}
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION")]
pub const EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME")]
pub const EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_image_view_min_lod");
