//!# [VK_MVK_macos_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_MVK_macos_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/mvk_macos_surface/VK_MVK_macos_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkMacOSSurfaceCreateInfoMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html)
# [doc = include_str ! ("../../../../doc/extensions/mvk_macos_surface/VkMacOSSurfaceCreateInfoMVK.md")]
#[doc(alias = "VkMacOSSurfaceCreateInfoMVK")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MacOsSurfaceCreateInfoMVK {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: MacOsSurfaceCreateFlagsMVK,
    #[doc(alias = "pView")]
    view: *const std::ffi::c_void,
}
#[doc(alias = "VkMacOSSurfaceCreateFlagsMVK")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MacOsSurfaceCreateFlagsMVK(u32);
impl MacOsSurfaceCreateFlagsMVK {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_MVK_MACOS_SURFACE_SPEC_VERSION")]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_MVK_MACOS_SURFACE_EXTENSION_NAME")]
pub const MVK_MACOS_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_MVK_macos_surface");
///# [vkCreateMacOSSurfaceMVK](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateMacOSSurfaceMVK.html)
# [doc = include_str ! ("../../../../doc/extensions/mvk_macos_surface/vkCreateMacOSSurfaceMVK.md")]
#[doc(alias = "vkCreateMacOSSurfaceMVK")]
pub type FNCreateMacOsSurfaceMvk = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MacOsSurfaceCreateInfoMVK,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
