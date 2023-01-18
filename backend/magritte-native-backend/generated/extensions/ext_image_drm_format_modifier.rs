use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Device, FormatFeatureFlags, Image, SharingMode, StructureType,
        SubresourceLayout, VulkanResultCodes,
    },
    vulkan1_3::FormatFeatureFlags2,
};
use std::ffi::CStr;
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
#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
pub type FNGetImageDrmFormatModifierPropertiesExt = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> VulkanResultCodes;
