//!# [VK_EXT_metal_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_metal_surface/VK_EXT_metal_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkMetalSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_metal_surface/VkMetalSurfaceCreateInfoEXT.md")]
#[doc(alias = "VkMetalSurfaceCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: MetalSurfaceCreateFlagsEXT,
    #[doc(alias = "pLayer")]
    layer: *const CaMetalLayer,
}
///# [CAMetalLayer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/CAMetalLayer.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_metal_surface/CAMetalLayer.md")]
#[doc(alias = "CAMetalLayer")]
pub type CaMetalLayer = std::ffi::c_void;
#[doc(alias = "VkMetalSurfaceCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetalSurfaceCreateFlagsEXT(u32);
impl MetalSurfaceCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_METAL_SURFACE_SPEC_VERSION")]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_METAL_SURFACE_EXTENSION_NAME")]
pub const EXT_METAL_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_metal_surface");
///# [vkCreateMetalSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMetalSurfaceEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_metal_surface/vkCreateMetalSurfaceEXT.md")]
#[doc(alias = "vkCreateMetalSurfaceEXT")]
pub type FNCreateMetalSurfaceExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MetalSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;