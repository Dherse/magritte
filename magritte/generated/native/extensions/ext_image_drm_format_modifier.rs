pub use crate::common::extensions::ext_image_drm_format_modifier::{
    DrmFormatModifierProperties2EXT, DrmFormatModifierPropertiesEXT,
};
use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Device, Image, SharingMode, StructureType, SubresourceLayout, VulkanResultCodes,
};
#[doc(alias = "VkDrmFormatModifierPropertiesListEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "drmFormatModifierCount")]
    pub drm_format_modifier_count: u32,
    #[doc(alias = "pDrmFormatModifierProperties")]
    pub drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
}
impl Default for DrmFormatModifierPropertiesListEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DrmFormatModifierPropertiesListExt,
            p_next: unsafe { std::mem::zeroed() },
            drm_format_modifier_count: unsafe { std::mem::zeroed() },
            drm_format_modifier_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImageDrmFormatModifierInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
    #[doc(alias = "sharingMode")]
    pub sharing_mode: SharingMode,
    #[doc(alias = "queueFamilyIndexCount")]
    pub queue_family_index_count: u32,
    #[doc(alias = "pQueueFamilyIndices")]
    pub queue_family_indices: *const u32,
}
impl Default for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceImageDrmFormatModifierInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            drm_format_modifier: unsafe { std::mem::zeroed() },
            sharing_mode: unsafe { std::mem::zeroed() },
            queue_family_index_count: unsafe { std::mem::zeroed() },
            queue_family_indices: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageDrmFormatModifierListCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "drmFormatModifierCount")]
    pub drm_format_modifier_count: u32,
    #[doc(alias = "pDrmFormatModifiers")]
    pub drm_format_modifiers: *const u64,
}
impl Default for ImageDrmFormatModifierListCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageDrmFormatModifierListCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            drm_format_modifier_count: unsafe { std::mem::zeroed() },
            drm_format_modifiers: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageDrmFormatModifierExplicitCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
    #[doc(alias = "drmFormatModifierPlaneCount")]
    pub drm_format_modifier_plane_count: u32,
    #[doc(alias = "pPlaneLayouts")]
    pub plane_layouts: *const SubresourceLayout,
}
impl Default for ImageDrmFormatModifierExplicitCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageDrmFormatModifierExplicitCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            drm_format_modifier: unsafe { std::mem::zeroed() },
            drm_format_modifier_plane_count: unsafe { std::mem::zeroed() },
            plane_layouts: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageDrmFormatModifierPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
}
impl Default for ImageDrmFormatModifierPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageDrmFormatModifierPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            drm_format_modifier: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDrmFormatModifierPropertiesList2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DrmFormatModifierPropertiesList2EXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "drmFormatModifierCount")]
    pub drm_format_modifier_count: u32,
    #[doc(alias = "pDrmFormatModifierProperties")]
    pub drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
}
impl Default for DrmFormatModifierPropertiesList2EXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DrmFormatModifierPropertiesList2Ext,
            p_next: unsafe { std::mem::zeroed() },
            drm_format_modifier_count: unsafe { std::mem::zeroed() },
            drm_format_modifier_properties: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_image_drm_format_modifier::{
    EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME, EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION,
};
#[doc(alias = "vkGetImageDrmFormatModifierPropertiesEXT")]
pub type FNGetImageDrmFormatModifierPropertiesExt = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> VulkanResultCodes;
