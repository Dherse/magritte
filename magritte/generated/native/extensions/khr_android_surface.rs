//!# [VK_KHR_android_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_android_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_android_surface/VK_KHR_android_surface.md")]
use crate::{
    cstr,
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkAndroidSurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_android_surface/VkAndroidSurfaceCreateInfoKHR.md")]
#[doc(alias = "VkAndroidSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: AndroidSurfaceCreateFlagsKHR,
    window: *mut ANativeWindow,
}
///# [ANativeWindow](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/ANativeWindow.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_android_surface/ANativeWindow.md")]
pub type ANativeWindow = std::ffi::c_void;
#[doc(alias = "VkAndroidSurfaceCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AndroidSurfaceCreateFlagsKHR(u32);
impl AndroidSurfaceCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_KHR_ANDROID_SURFACE_SPEC_VERSION")]
pub const KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
#[doc(alias = "VK_KHR_ANDROID_SURFACE_EXTENSION_NAME")]
pub const KHR_ANDROID_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_android_surface");
///# [vkCreateAndroidSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_android_surface/vkCreateAndroidSurfaceKHR.md")]
#[doc(alias = "vkCreateAndroidSurfaceKHR")]
pub type FNCreateAndroidSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const AndroidSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
