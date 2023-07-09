use crate::native::{
    extensions::khr_swapchain::{SwapchainCreateInfoKHR, SwapchainKHR},
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Bool32, Device, Rect2D, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkDisplayPresentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPresentInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcRect")]
    pub src_rect: Rect2D,
    #[doc(alias = "dstRect")]
    pub dst_rect: Rect2D,
    pub persistent: Bool32,
}
impl Default for DisplayPresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayPresentInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            src_rect: unsafe { std::mem::zeroed() },
            dst_rect: unsafe { std::mem::zeroed() },
            persistent: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_display_swapchain::{
    KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME, KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION,
};
#[doc(alias = "vkCreateSharedSwapchainsKHR")]
pub type FNCreateSharedSwapchainsKhr = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_create_infos: *const SwapchainCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_swapchains: *mut SwapchainKHR,
) -> VulkanResultCodes;
