//! # Small helpers
//! 

#[cfg(feature = "VK_KHR_swapchain")]
use crate::Unique;

#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::{SwapchainImage, SwapchainImageView};

#[cfg(feature = "VK_KHR_swapchain")]
use crate::vulkan1_0::{ImageView, Image};

#[cfg(feature = "VK_KHR_swapchain")]
impl<'a> Unique<'a, SwapchainImage> {
    /// Transforms a swapchain image into a regular image
    #[inline]
    pub const fn as_raw_image(&self) -> Image {
        Image(self.this.0)
    }
}

#[cfg(feature = "VK_KHR_swapchain")]
impl<'a> Unique<'a, ImageView> {
    pub fn to_swapchain_view<'new, 'b>(self, swapchain: &'new Unique<'b, SwapchainImage>) -> Unique<'new, SwapchainImageView> where 'a: 'b, 'b: 'new {
        unsafe {
            Unique {
                parent: std::mem::transmute(swapchain),
                vtable: (),
                metadata: true,
                this: SwapchainImageView(self.disable_drop().this.0)
            }
        }
    }
}