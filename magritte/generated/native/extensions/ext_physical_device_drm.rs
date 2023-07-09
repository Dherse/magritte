use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceDrmPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDrmPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "hasPrimary")]
    pub has_primary: Bool32,
    #[doc(alias = "hasRender")]
    pub has_render: Bool32,
    #[doc(alias = "primaryMajor")]
    pub primary_major: i64,
    #[doc(alias = "primaryMinor")]
    pub primary_minor: i64,
    #[doc(alias = "renderMajor")]
    pub render_major: i64,
    #[doc(alias = "renderMinor")]
    pub render_minor: i64,
}
impl Default for PhysicalDeviceDrmPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDrmPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            has_primary: unsafe { std::mem::zeroed() },
            has_render: unsafe { std::mem::zeroed() },
            primary_major: unsafe { std::mem::zeroed() },
            primary_minor: unsafe { std::mem::zeroed() },
            render_major: unsafe { std::mem::zeroed() },
            render_minor: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_physical_device_drm::{
    EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME, EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION,
};
