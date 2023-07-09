use crate::native::{
    extensions::{
        khr_display::SurfaceTransformFlagsKHR,
        khr_surface::{CompositeAlphaFlagsKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR},
    },
    vulkan1_0::{BaseOutStructure, Extent2D, ImageUsageFlags, PhysicalDevice, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkSurfaceCapabilities2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilities2EXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minImageCount")]
    pub min_image_count: u32,
    #[doc(alias = "maxImageCount")]
    pub max_image_count: u32,
    #[doc(alias = "currentExtent")]
    pub current_extent: Extent2D,
    #[doc(alias = "minImageExtent")]
    pub min_image_extent: Extent2D,
    #[doc(alias = "maxImageExtent")]
    pub max_image_extent: Extent2D,
    #[doc(alias = "maxImageArrayLayers")]
    pub max_image_array_layers: u32,
    #[doc(alias = "supportedTransforms")]
    pub supported_transforms: SurfaceTransformFlagsKHR,
    #[doc(alias = "currentTransform")]
    pub current_transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "supportedCompositeAlpha")]
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    #[doc(alias = "supportedUsageFlags")]
    pub supported_usage_flags: ImageUsageFlags,
    #[doc(alias = "supportedSurfaceCounters")]
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}
impl Default for SurfaceCapabilities2EXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SurfaceCapabilities2Ext,
            p_next: unsafe { std::mem::zeroed() },
            min_image_count: unsafe { std::mem::zeroed() },
            max_image_count: unsafe { std::mem::zeroed() },
            current_extent: unsafe { std::mem::zeroed() },
            min_image_extent: unsafe { std::mem::zeroed() },
            max_image_extent: unsafe { std::mem::zeroed() },
            max_image_array_layers: unsafe { std::mem::zeroed() },
            supported_transforms: unsafe { std::mem::zeroed() },
            current_transform: unsafe { std::mem::zeroed() },
            supported_composite_alpha: unsafe { std::mem::zeroed() },
            supported_usage_flags: unsafe { std::mem::zeroed() },
            supported_surface_counters: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_display_surface_counter::{
    SurfaceCounterFlagBitsEXT, SurfaceCounterFlagsEXT, EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME,
    EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
pub type FNGetPhysicalDeviceSurfaceCapabilities2Ext = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> VulkanResultCodes;
