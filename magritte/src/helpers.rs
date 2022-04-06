//! # Small helpers
//! 

#[cfg(feature = "VK_KHR_swapchain")]
use crate::Unique;

#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::SwapchainImage;

#[cfg(feature = "VK_KHR_swapchain")]
use crate::vulkan1_0::Image;

#[cfg(feature = "VK_KHR_swapchain")]
impl<'a, 'b> Unique<'a, 'b, SwapchainImage> {
    /// Transforms a swapchain image into a regular image
    #[inline]
    pub const fn as_raw_image(&self) -> Image {
        Image(self.this.0)
    }
}