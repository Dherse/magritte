use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkAndroidSurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AndroidSurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: AndroidSurfaceCreateFlagsKHR,
    pub window: *mut ANativeWindow,
}
impl Default for AndroidSurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AndroidSurfaceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            window: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_android_surface::{
    ANativeWindow, AndroidSurfaceCreateFlagsKHR, KHR_ANDROID_SURFACE_EXTENSION_NAME, KHR_ANDROID_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateAndroidSurfaceKHR")]
pub type FNCreateAndroidSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const AndroidSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
