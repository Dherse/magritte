//! # Small helpers

use std::sync::atomic::AtomicBool;

use crate::Handle;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::Unique;

#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::{SwapchainImage, SwapchainImageView};

#[cfg(feature = "VK_KHR_swapchain")]
use crate::vulkan1_0::{Image, ImageView};

#[cfg(feature = "VK_KHR_swapchain")]
impl Unique<SwapchainImage> {
    /// Transforms a swapchain image into a regular image
    #[inline]
    pub const fn as_raw_image(&self) -> Image {
        Image(self.this.0)
    }
}

#[cfg(feature = "VK_KHR_swapchain")]
impl Unique<ImageView> {
    pub fn to_swapchain_view(self, swapchain: &Unique<SwapchainImage>) -> Unique<SwapchainImageView> {
        self.disable_drop();

        unsafe {
            Unique::new(
                swapchain,
                self.this.coerce::<SwapchainImageView>(),
                AtomicBool::new(false),
            )
        }
    }
}
