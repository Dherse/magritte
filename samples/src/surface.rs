use log::{info, warn};
use magritte::{
    extensions::{
        khr_display::SurfaceTransformFlagsKHR,
        khr_surface::{CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR},
        khr_swapchain::{SwapchainCreateInfoKHR, SwapchainImage, SwapchainImageView, SwapchainKHR},
    },
    vulkan1_0::{
        ComponentMapping, ComponentSwizzle, Extent2D, Format, ImageAspectFlags, ImageSubresourceRange, ImageUsageFlags,
        ImageViewCreateInfo, ImageViewType, Semaphore, SharingMode, VulkanResultCodes,
    },
    AsRaw, SmallVec, Unique,
};

use crate::vulkan::Vulkan;

/// Helper for dealing with the surface and the swapchain
pub struct Surface {
    /// The surface we will draw on
    pub surface: Unique<SurfaceKHR>,

    /// The swapchain we will draw with
    pub swapchain: Option<Unique<SwapchainKHR>>,

    /// The swapchain images
    pub swapchain_images: SmallVec<Unique<SwapchainImage>>,

    /// The swapchain image views
    pub swapchain_image_views: SmallVec<Unique<SwapchainImageView>>,

    /// The size of the surface
    pub extent: Extent2D,

    /// The format of the surface
    pub format: Format,
}

impl Drop for Surface {
    fn drop(&mut self) {
        // First we destroy the views
        self.swapchain_image_views.clear();

        // Then the associated images
        self.swapchain_images.clear();

        // Then the swapchain itself
        // This simply ensures that the swapchain is destroyed before the rest of this structure.
        self.swapchain.take();
    }
}

impl Surface {
    /// Creates a new easier to use wrapper for surfaces and swapchains
    pub fn new(vulkan: &Vulkan, surface: Unique<SurfaceKHR>) -> Result<Self, VulkanResultCodes> {
        // First we want to list the supported format to find the one we want to use.
        let (mut surface_formats, _) = unsafe {
            vulkan
                .physical_device()
                .get_physical_device_surface_formats_khr(Some(surface.as_raw()), None)?
        };

        // We will use the first format regardless.
        // Note that there are some formats in Vulkan that are required, you could also use one of those.
        // There are also ways of getting the "optimal" format for the surface, you may want to take a look
        // at those.
        let surface_format = surface_formats.remove(0);

        info!("Using format: {:?}", surface_format);

        // Now we will get the capabilities of the surface to know what resolution it supports,
        // the number of frames it must contain and all sorts of information.
        // Have a look at `SurfaceCapabilitiesKHR` to see all that it can do.
        let (surface_capabilities, _) = unsafe {
            vulkan
                .physical_device()
                .get_physical_device_surface_capabilities_khr(surface.as_raw())?
        };

        // We get the number of images we will request
        let mut desired_image_count = surface_capabilities.min_image_count() + 1;
        if surface_capabilities.max_image_count() > 0 && desired_image_count > surface_capabilities.max_image_count() {
            desired_image_count = surface_capabilities.max_image_count();
        }

        info!("Will use {} images", desired_image_count);

        // We get the surface resolution using the current size.
        // The Vulkan spec states that if it's equal to [`std::u32::MAX`] then it has not been
        // set yet. Otherwise, we use the current window size.
        let surface_resolution = match surface_capabilities.current_extent().width() {
            std::u32::MAX => Extent2D {
                width: 1920,
                height: 1080,
            },
            _ => surface_capabilities.current_extent(),
        };

        info!("Window size: {:?}", surface_resolution);

        // We get the current window transform (i.e whether it's rotated or flipped or not)
        // In most cases, this is going to be [`SurfaceTransformFlagBitsKHR::IDENTITY`].
        let pre_transform = if surface_capabilities
            .supported_transforms()
            .contains(SurfaceTransformFlagsKHR::IDENTITY)
        {
            SurfaceTransformFlagBitsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform()
        };

        info!("Transform: {:?}", pre_transform);

        // Now we get the different presentation modes the surface supports.
        // Think of this like a more advanced version of V-Sync, you can control
        // the kind of V-Sync that happens, how and when from a single flag.
        let (present_modes, _) = unsafe {
            vulkan
                .physical_device()
                .get_physical_device_surface_present_modes_khr(Some(surface.as_raw()), None)?
        };

        // We select a present mode, ideally [`PresentModeKHR::MAILBOX`] but [`PresentModeKHR::FIFO`]
        // otherwise. The Vulkan spec states that [`PresentModeKHR::FIFO`] is always available!
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == PresentModeKHR::MAILBOX)
            .unwrap_or(PresentModeKHR::FIFO);

        // Here, we gather all of the information required to create the swapchain:
        // - The surface the swapchain will "act" on.
        // - The minimum number of frames we want
        // - The color space (based on the obtained capabilities)
        // - The color format (based on the obtained capabilities)
        // - The image size
        // - The image usage (we will use it as the output of a fragment shader so it's a color attachment)
        // - We want exclusive access (i.e from one queue at a time)
        // - The transform
        // - The way we want to composite alpha (i.e turn transparency into color)
        // - Whether Vulkan can discard pixels outside of the rendering area
        // - The number of array layers (one in this case)
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .set_surface(surface.as_raw())
            .set_min_image_count(desired_image_count)
            .set_image_color_space(surface_format.color_space())
            .set_image_format(surface_format.format())
            .set_image_extent(surface_resolution)
            .set_image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .set_image_sharing_mode(SharingMode::EXCLUSIVE)
            .set_pre_transform(pre_transform)
            .set_composite_alpha(CompositeAlphaFlagBitsKHR::OPAQUE)
            .set_present_mode(present_mode)
            .set_clipped(true)
            .set_image_array_layers(1);

        // Finally, we create the swapchain itself
        // ⚠ Note that here, things get a bit tricky, the swapchain **must** live longer than the
        // surface it was created for. **You** need to ensure this! Magritte currently cannot do
        // it. Probably in the future.
        // ⚠ Note that the surface we are refering to is the one in the `SwapchainCreateInfoKHR`.
        // TODO: use metadata to implement double lifetimes.
        let (swapchain, _) = unsafe { vulkan.device().create_swapchain_khr(&swapchain_create_info, None)? };

        info!("We have created the swapchain: {:?}", swapchain.as_raw());

        // Now we get the images (allocated by Vulkan) that are associated with the swapchain
        let (swapchain_images, _) = unsafe { swapchain.get_swapchain_images_khr(swapchain.as_raw(), None)? };

        info!("We have {} swapchain images", swapchain_images.len());

        // Now we need to create the swapchain image views
        // For this we need the following for each image:
        // - The view type, that is, how we think the image will be layed out (in 2D in our case)
        // - The format (essentially, how many bytes per pixel and the color depth of each pixel)
        // - How to map the components, you can exchange red and blue for example and look at the result
        // - What parts of the image we want the view to point to, in this case:
        //  - the color information
        //  - at the first mipmap level (since we have only one anyway)
        //  - in the first array layer (since we have only one anyway)
        let swapchain_image_views = swapchain_images
            .iter()
            .map(|image| {
                let create_view_info = ImageViewCreateInfo::default()
                    .set_view_type(ImageViewType::_2_D)
                    .set_format(surface_format.format)
                    .set_components(ComponentMapping {
                        r: ComponentSwizzle::R,
                        g: ComponentSwizzle::G,
                        b: ComponentSwizzle::B,
                        a: ComponentSwizzle::A,
                    })
                    .set_subresource_range(ImageSubresourceRange {
                        aspect_mask: ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .set_image(image.as_raw_image());

                unsafe { image.create_swapchain_image_view(&create_view_info, None).result() }
            })
            .collect::<Result<SmallVec<_>, _>>()?;

        Ok(Self {
            surface,
            swapchain: Some(swapchain),
            swapchain_images,
            swapchain_image_views,
            extent: surface_resolution,
            format: surface_format.format(),
        })
    }

    /// Acquires the next image
    #[inline]
    pub fn acquire_next_image(&self, semaphore: &Unique<Semaphore>) -> Result<usize, VulkanResultCodes> {
        let (present_index, result) = unsafe {
            self.swapchain().device().acquire_next_image_khr(
                self.swapchain().as_raw(),
                Some(std::u64::MAX),
                Some(semaphore.as_raw()),
                None,
            )?
        };

        if result == VulkanResultCodes::SUBOPTIMAL_KHR {
            warn!("Suboptimal swapchain");
        }

        Ok(present_index as usize)
    }

    /// Get a reference to the surface's surface.
    #[inline]
    pub fn surface(&self) -> &Unique<SurfaceKHR> {
        &self.surface
    }

    /// Get a reference to the surface's swapchain.
    #[inline]
    pub fn swapchain(&self) -> &Unique<SwapchainKHR> {
        self.swapchain.as_ref().unwrap()
    }

    /// Get a reference to the surface's swapchain images.
    #[inline]
    pub fn swapchain_images(&self) -> &SmallVec<Unique<SwapchainImage>> {
        &self.swapchain_images
    }

    /// Get a reference to the surface's swapchain image views.
    #[inline]
    pub fn swapchain_image_views(&self) -> &SmallVec<Unique<SwapchainImageView>> {
        &self.swapchain_image_views
    }

    /// Get a reference to the surface's extent.
    #[inline]
    pub fn extent(&self) -> Extent2D {
        self.extent
    }

    /// Gets the number of images in the swapchain
    #[inline]
    pub fn image_count(&self) -> usize {
        self.swapchain_images().len()
    }

    /// Get a reference to the surface's format.
    pub fn format(&self) -> Format {
        self.format
    }
}
