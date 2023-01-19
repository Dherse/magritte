//!# [VK_EXT_physical_device_drm](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_physical_device_drm.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_physical_device_drm/VK_EXT_physical_device_drm.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDrmPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_physical_device_drm/VkPhysicalDeviceDrmPropertiesEXT.md")]
#[doc(alias = "VkPhysicalDeviceDrmPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDrmPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "hasPrimary")]
    has_primary: Bool32,
    #[doc(alias = "hasRender")]
    has_render: Bool32,
    #[doc(alias = "primaryMajor")]
    primary_major: i64,
    #[doc(alias = "primaryMinor")]
    primary_minor: i64,
    #[doc(alias = "renderMajor")]
    render_major: i64,
    #[doc(alias = "renderMinor")]
    render_minor: i64,
}
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION")]
pub const EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME")]
pub const EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_physical_device_drm");
