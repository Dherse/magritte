use log::{info, trace};
use magritte::{
    vulkan1_0::{
         DependencyFlags, Extent3D, Format, Image, ImageAspectFlags, ImageCreateInfo,
        ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView,
        ImageViewCreateInfo, ImageViewType, PipelineStageFlags,
        SampleCountFlagBits, SharingMode, VulkanResultCodes,
    },
    AsRaw, Unique,
};
use magritte_vma::{Allocator, AllocationCreateFlags, ImageUsage, VmaImage};

use crate::{commands::Commands, surface::Surface, vulkan::Vulkan};

/// A simple container for a render target and its memory.
pub struct RenderTarget {
    /// The image backing the render target
    pub image: VmaImage,

    /// The view to access the render target
    pub image_view: Unique<ImageView>,

    /// The allocator from which this render target is allocated
    pub allocator: Unique<Allocator>,
}

impl RenderTarget {
    /// Creates a new render target
    #[inline]
    pub fn new(
        vulkan: &Vulkan,
        commands: &Commands,
        surface: &Surface,
        allocator: &Unique<Allocator>,
        format: Format,
        msaa: SampleCountFlagBits,
    ) -> Result<Self, VulkanResultCodes> {
        let (image, image_view) =
            Self::create_new_image_view_memory(vulkan, commands, surface, allocator, format, msaa)?;

        Ok(Self {
            image,
            image_view,
            allocator: allocator.clone(),
        })
    }

    /// Sizes the render target image
    #[inline]
    pub fn resize(
        &mut self,
        vulkan: &Vulkan,
        commands: &Commands,
        surface: &Surface,
        format: Format,
        msaa: SampleCountFlagBits,
    ) -> Result<(), VulkanResultCodes> {
        let (image, image_view) =
            Self::create_new_image_view_memory(vulkan, commands, surface, self.allocator(), format, msaa)?;

        self.image_view = image_view;
        self.image = image;

        Ok(())
    }

    /// Get a reference to the render target's image.
    #[inline]
    pub fn image(&self) -> &Unique<Image> {
        self.vma_image().image()
    }

    /// Get a reference to the render target's VMA image.
    #[inline]
    pub fn vma_image(&self) -> &VmaImage {
        &self.image
    }

    /// Get a reference to the render target's image view.
    #[inline]
    pub fn image_view(&self) -> &Unique<ImageView> {
        &self.image_view
    }

    /// Get a reference to the render target's allocator.
    #[must_use]
    pub fn allocator(&self) -> &Unique<Allocator> {
        &self.allocator
    }

    fn create_new_image_view_memory(
        vulkan: &Vulkan,
        commands: &Commands,
        surface: &Surface,
        allocator: &Unique<Allocator>,
        format: Format,
        msaa: SampleCountFlagBits,
    ) -> Result<(VmaImage, Unique<ImageView>), VulkanResultCodes> {
        // First we get the memory properties, we will use this when allocating our image
        let memory_properties = unsafe { vulkan.physical_device().get_physical_device_memory_properties() };

        // We need to create an Image, that being a Vulkan handle of an image, for this we need:
        // - Type type of image (2-dimensional in this case)
        // - The format
        // - The size as a 3D image (the 2D size with a depth of 1)
        // - The number of mip levels (only one)
        // - The number of array layers (only one)
        // - The number of samples for MSAA
        // - The tiling mode (optimal is preferred if it's supported)
        // - The usage (as a render target attachment)
        // - The sharing mode (exclusive because only one queue will access this image at a time)
        let image_create_info = ImageCreateInfo::default()
            .set_image_type(ImageType::_2D)
            .set_format(format)
            .set_extent(Extent3D {
                width: surface.extent().width(),
                height: surface.extent().height(),
                depth: 1,
            })
            .set_mip_levels(1)
            .set_array_layers(1)
            .set_samples(msaa)
            .set_tiling(ImageTiling::OPTIMAL)
            .set_usage(ImageUsageFlags::TRANSIENT_ATTACHMENT | ImageUsageFlags::COLOR_ATTACHMENT)
            .set_sharing_mode(SharingMode::EXCLUSIVE);

        let image = allocator.create_image(
            &image_create_info, 
            AllocationCreateFlags::empty(), 
            ImageUsage::Auto, 
            None, 
            None, 
            None
        )?;

        info!("Created the render target image: {:?}", image.as_raw());

        // TODO: explain this more.
        commands.record_and_submit_setup(&[], &[], &[], |cmd| {
            // Here we create a barrier to initialize our render target.
            let layout_transition_barriers = ImageMemoryBarrier::default()
                .set_image(image.image().as_raw())
                .set_new_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .set_old_layout(ImageLayout::UNDEFINED)
                .set_subresource_range(
                    ImageSubresourceRange::default()
                        .set_aspect_mask(ImageAspectFlags::COLOR)
                        .set_layer_count(1)
                        .set_level_count(1),
                );

            // Here we record the barrier inside of the command buffer.
            unsafe {
                cmd.cmd_pipeline_barrier(
                    PipelineStageFlags::BOTTOM_OF_PIPE,
                    PipelineStageFlags::LATE_FRAGMENT_TESTS,
                    DependencyFlags::empty(),
                    &[],
                    &[],
                    std::slice::from_ref(&layout_transition_barriers),
                );
            }

            trace!("Recorded image initialization commands");

            Ok(())
        })?;

        let image_view_info = ImageViewCreateInfo::default()
            .set_subresource_range(
                ImageSubresourceRange::default()
                    .set_aspect_mask(ImageAspectFlags::COLOR)
                    .set_level_count(1)
                    .set_layer_count(1),
            )
            .set_image(image.image().as_raw())
            .set_format(image_create_info.format())
            .set_view_type(ImageViewType::_2D);

        let (image_view, _) = unsafe { image.image().create_image_view(&image_view_info, None)? };

        Ok((image, image_view))
    }
}
