//!# [VK_EXT_headless_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_headless_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_headless_surface/VK_EXT_headless_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkHeadlessSurfaceCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_headless_surface/VkHeadlessSurfaceCreateInfoEXT.md")]
#[doc(alias = "VkHeadlessSurfaceCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: HeadlessSurfaceCreateFlagsEXT,
}
#[doc(alias = "VkHeadlessSurfaceCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeadlessSurfaceCreateFlagsEXT(u32);
impl HeadlessSurfaceCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_SPEC_VERSION")]
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME")]
pub const EXT_HEADLESS_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_headless_surface");
///# [vkCreateHeadlessSurfaceEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_headless_surface/vkCreateHeadlessSurfaceEXT.md")]
#[doc(alias = "vkCreateHeadlessSurfaceEXT")]
pub type FNCreateHeadlessSurfaceExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const HeadlessSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
