//!# [VK_EXT_image_drm_format_modifier](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_drm_format_modifier.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VK_EXT_image_drm_format_modifier.md")]
use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Device, FormatFeatureFlags, Image, SharingMode, StructureType,
        SubresourceLayout, VulkanResultCodes,
    },
    vulkan1_3::FormatFeatureFlags2,
};
use std::ffi::CStr;
///# [VkDrmFormatModifierPropertiesListEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkDrmFormatModifierPropertiesListEXT.md")]
#[doc(alias = "VkDrmFormatModifierPropertiesListEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "drmFormatModifierCount")]
    drm_format_modifier_count: u32,
    #[doc(alias = "pDrmFormatModifierProperties")]
    drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
}
///# [VkDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkDrmFormatModifierPropertiesEXT.md")]
#[doc(alias = "VkDrmFormatModifierPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    #[doc(alias = "drmFormatModifier")]
    drm_format_modifier: u64,
    #[doc(alias = "drmFormatModifierPlaneCount")]
    drm_format_modifier_plane_count: u32,
    #[doc(alias = "drmFormatModifierTilingFeatures")]
    drm_format_modifier_tiling_features: FormatFeatureFlags,
}
///# [VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.md")]
#[doc(alias = "VkPhysicalDeviceImageDrmFormatModifierInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "drmFormatModifier")]
    drm_format_modifier: u64,
    #[doc(alias = "sharingMode")]
    sharing_mode: SharingMode,
    #[doc(alias = "queueFamilyIndexCount")]
    queue_family_index_count: u32,
    #[doc(alias = "pQueueFamilyIndices")]
    queue_family_indices: *const u32,
}
///# [VkImageDrmFormatModifierListCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkImageDrmFormatModifierListCreateInfoEXT.md")]
#[doc(alias = "VkImageDrmFormatModifierListCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "drmFormatModifierCount")]
    drm_format_modifier_count: u32,
    #[doc(alias = "pDrmFormatModifiers")]
    drm_format_modifiers: *const u64,
}
///# [VkImageDrmFormatModifierExplicitCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkImageDrmFormatModifierExplicitCreateInfoEXT.md")]
#[doc(alias = "VkImageDrmFormatModifierExplicitCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "drmFormatModifier")]
    drm_format_modifier: u64,
    #[doc(alias = "drmFormatModifierPlaneCount")]
    drm_format_modifier_plane_count: u32,
    #[doc(alias = "pPlaneLayouts")]
    plane_layouts: *const SubresourceLayout,
}
///# [VkImageDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkImageDrmFormatModifierPropertiesEXT.md")]
#[doc(alias = "VkImageDrmFormatModifierPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "drmFormatModifier")]
    drm_format_modifier: u64,
}
///# [VkDrmFormatModifierPropertiesList2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesList2EXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkDrmFormatModifierPropertiesList2EXT.md")]
#[doc(alias = "VkDrmFormatModifierPropertiesList2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesList2EXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "drmFormatModifierCount")]
    drm_format_modifier_count: u32,
    #[doc(alias = "pDrmFormatModifierProperties")]
    drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
}
///# [VkDrmFormatModifierProperties2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierProperties2EXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/VkDrmFormatModifierProperties2EXT.md")]
#[doc(alias = "VkDrmFormatModifierProperties2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierProperties2EXT {
    #[doc(alias = "drmFormatModifier")]
    drm_format_modifier: u64,
    #[doc(alias = "drmFormatModifierPlaneCount")]
    drm_format_modifier_plane_count: u32,
    #[doc(alias = "drmFormatModifierTilingFeatures")]
    drm_format_modifier_tiling_features: FormatFeatureFlags2,
}
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_image_drm_format_modifier");
///# [vkGetImageDrmFormatModifierPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_image_drm_format_modifier/vkGetImageDrmFormatModifierPropertiesEXT.md")]
#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
pub type FNGetImageDrmFormatModifierPropertiesExt = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> VulkanResultCodes;
