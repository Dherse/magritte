use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkHeadlessSurfaceCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct HeadlessSurfaceCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: HeadlessSurfaceCreateFlagsEXT,
}
impl Default for HeadlessSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::HeadlessSurfaceCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_headless_surface::{
    HeadlessSurfaceCreateFlagsEXT, EXT_HEADLESS_SURFACE_EXTENSION_NAME, EXT_HEADLESS_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateHeadlessSurfaceEXT")]
pub type FNCreateHeadlessSurfaceExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const HeadlessSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
