//! # Small helpers

#[cfg(feature = "VK_KHR_swapchain")]
use std::sync::atomic::AtomicBool;

#[cfg(feature = "VK_KHR_swapchain")]
use crate::{
    extensions::khr_swapchain::{SwapchainImage, SwapchainImageView},
    vulkan1_0::{Image, ImageView},
    Handle, Unique,
};

#[cfg(feature = "VK_KHR_swapchain")]
impl Unique<SwapchainImage> {
    /// Transforms a swapchain image into a regular image
    #[inline]
    pub const fn as_raw_image(&self) -> Image {
        Image(self.this.0)
    }
}

#[cfg(feature = "VK_KHR_swapchain")]
impl Unique<SwapchainImageView> {
    /// Transforms a swapchain image into a regular image
    #[inline]
    pub const fn as_raw_image_view(&self) -> ImageView {
        ImageView(self.this.0)
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
