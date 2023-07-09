use crate::native::{
    extensions::khr_surface::{
        ColorSpaceKHR, CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR,
    },
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Device, Extent2D, Fence, Format, Image, ImageUsageFlags, Queue,
        Semaphore, SharingMode, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkSwapchainCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    #[doc(alias = "minImageCount")]
    pub min_image_count: u32,
    #[doc(alias = "imageFormat")]
    pub image_format: Format,
    #[doc(alias = "imageColorSpace")]
    pub image_color_space: ColorSpaceKHR,
    #[doc(alias = "imageExtent")]
    pub image_extent: Extent2D,
    #[doc(alias = "imageArrayLayers")]
    pub image_array_layers: u32,
    #[doc(alias = "imageUsage")]
    pub image_usage: ImageUsageFlags,
    #[doc(alias = "imageSharingMode")]
    pub image_sharing_mode: SharingMode,
    #[doc(alias = "queueFamilyIndexCount")]
    pub queue_family_index_count: u32,
    #[doc(alias = "pQueueFamilyIndices")]
    pub queue_family_indices: *const u32,
    #[doc(alias = "preTransform")]
    pub pre_transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "compositeAlpha")]
    pub composite_alpha: CompositeAlphaFlagBitsKHR,
    #[doc(alias = "presentMode")]
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    #[doc(alias = "oldSwapchain")]
    pub old_swapchain: SwapchainKHR,
}
impl Default for SwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::SwapchainCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            surface: unsafe { std::mem::zeroed() },
            min_image_count: unsafe { std::mem::zeroed() },
            image_format: unsafe { std::mem::zeroed() },
            image_color_space: unsafe { std::mem::zeroed() },
            image_extent: unsafe { std::mem::zeroed() },
            image_array_layers: unsafe { std::mem::zeroed() },
            image_usage: unsafe { std::mem::zeroed() },
            image_sharing_mode: unsafe { std::mem::zeroed() },
            queue_family_index_count: unsafe { std::mem::zeroed() },
            queue_family_indices: unsafe { std::mem::zeroed() },
            pre_transform: unsafe { std::mem::zeroed() },
            composite_alpha: unsafe { std::mem::zeroed() },
            present_mode: unsafe { std::mem::zeroed() },
            clipped: unsafe { std::mem::zeroed() },
            old_swapchain: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPresentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreCount")]
    pub wait_semaphore_count: u32,
    #[doc(alias = "pWaitSemaphores")]
    pub wait_semaphores: *const Semaphore,
    #[doc(alias = "swapchainCount")]
    pub swapchain_count: u32,
    #[doc(alias = "pSwapchains")]
    pub swapchains: *const SwapchainKHR,
    #[doc(alias = "pImageIndices")]
    pub image_indices: *const u32,
    #[doc(alias = "pResults")]
    pub results: *mut VulkanResultCodes,
}
impl Default for PresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PresentInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            wait_semaphore_count: unsafe { std::mem::zeroed() },
            wait_semaphores: unsafe { std::mem::zeroed() },
            swapchain_count: unsafe { std::mem::zeroed() },
            swapchains: unsafe { std::mem::zeroed() },
            image_indices: unsafe { std::mem::zeroed() },
            results: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSwapchainKHR")]
#[repr(transparent)]
pub struct SwapchainKHR(u64);
impl SwapchainKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for SwapchainKHR {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::khr_swapchain::{
    SwapchainCreateFlagBitsKHR, SwapchainCreateFlagsKHR, KHR_SWAPCHAIN_EXTENSION_NAME, KHR_SWAPCHAIN_SPEC_VERSION,
};
#[doc(alias = "vkCreateSwapchainKHR")]
pub type FNCreateSwapchainKhr = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SwapchainCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_swapchain: *mut SwapchainKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroySwapchainKHR")]
pub type FNDestroySwapchainKhr =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetSwapchainImagesKHR")]
pub type FNGetSwapchainImagesKhr = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut Image,
) -> VulkanResultCodes;
#[doc(alias = "vkAcquireNextImageKHR")]
pub type FNAcquireNextImageKhr = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    p_image_index: *mut u32,
) -> VulkanResultCodes;
#[doc(alias = "vkQueuePresentKHR")]
pub type FNQueuePresentKhr =
    unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR) -> VulkanResultCodes;
