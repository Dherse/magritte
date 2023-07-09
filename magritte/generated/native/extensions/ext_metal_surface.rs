use crate::native::{
    extensions::khr_surface::SurfaceKHR,
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Instance, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkMetalSurfaceCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: MetalSurfaceCreateFlagsEXT,
    #[doc(alias = "pLayer")]
    pub layer: *const CaMetalLayer,
}
impl Default for MetalSurfaceCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::MetalSurfaceCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            layer: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_metal_surface::{
    CaMetalLayer, MetalSurfaceCreateFlagsEXT, EXT_METAL_SURFACE_EXTENSION_NAME, EXT_METAL_SURFACE_SPEC_VERSION,
};
#[doc(alias = "vkCreateMetalSurfaceEXT")]
pub type FNCreateMetalSurfaceExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MetalSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
