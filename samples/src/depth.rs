use log::{error, info, trace};
use magritte::{
    memory::find_memory_type_index,
    size::Size,
    vulkan1_0::{
        AccessFlags, DependencyFlags, DeviceMemory, Extent3D, Format, Image, ImageAspectFlags, ImageCreateInfo,
        ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView,
        ImageViewCreateInfo, ImageViewType, MemoryAllocateInfo, MemoryPropertyFlags, PipelineStageFlags,
        SampleCountFlagBits, SharingMode, VulkanResultCodes,
    },
    AsRaw, Unique,
};

use crate::{commands::Commands, surface::Surface, vulkan::Vulkan};

/// A simple container for a depth buffer and its memory.
pub struct Depth {
    /// The image backing the depth buffer
    pub image: Unique<Image>,

    /// The view to access the depth buffer
    pub image_view: Unique<ImageView>,

    /// The memory backing the depth buffer
    pub image_memory: Unique<DeviceMemory>,
}

impl Depth {
    /// Creates a new depth buffer
    #[inline]
    pub fn new(
        vulkan: &Vulkan,
        commands: &Commands,
        surface: &Surface,
        msaa: SampleCountFlagBits,
    ) -> Result<Self, VulkanResultCodes> {
        let (image, image_view, image_memory) = Self::create_new_image_view_memory(vulkan, commands, surface, msaa)?;

        Ok(Self {
            image,
            image_view,
            image_memory,
        })
    }

    /// Sizes the depth image
    #[inline]
    pub fn resize(
        &mut self,
        vulkan: &Vulkan,
        commands: &Commands,
        surface: &Surface,
        msaa: SampleCountFlagBits,
    ) -> Result<(), VulkanResultCodes> {
        let (image, image_view, image_memory) = Self::create_new_image_view_memory(vulkan, commands, surface, msaa)?;

        self.image_view = image_view;
        self.image_memory = image_memory;
        self.image = image;

        Ok(())
    }

    /// Get a reference to the depth's image.
    #[inline]
    pub fn image(&self) -> &Unique<Image> {
        &self.image
    }

    /// Get a reference to the depth's image view.
    #[inline]
    pub fn image_view(&self) -> &Unique<ImageView> {
        &self.image_view
    }

    /// Get a reference to the depth's image memory.
    #[inline]
    pub fn image_memory(&self) -> &Unique<DeviceMemory> {
        &self.image_memory
    }

    fn create_new_image_view_memory(
        vulkan: &Vulkan,
        commands: &Commands,
        surface: &Surface,
        msaa: SampleCountFlagBits,
    ) -> Result<(Unique<Image>, Unique<ImageView>, Unique<DeviceMemory>), VulkanResultCodes> {
        // First we get the memory properties, we will use this when allocating our image
        let memory_properties = unsafe { vulkan.physical_device().get_physical_device_memory_properties() };

        // We need to create an Image, that being a Vulkan handle of an image, for this we need:
        // - Type type of image (2-dimensional in this case)
        // - The format (depth with 16-bit resolution, could also use 24-bit or 32-bit)
        // - The size as a 3D image (the 2D size with a depth of 1)
        // - The number of mip levels (only one)
        // - The number of array layers (only one)
        // - The number of samples for MSAA
        // - The tiling mode (optimal is preferred if it's supported)
        // - The usage (as a depth buffer attachment)
        // - The sharing mode (exclusive because only one queue will access this image at a time)
        let image_create_info = ImageCreateInfo::default()
            .with_image_type(ImageType::_2D)
            .with_format(Format::D16_UNORM)
            .with_extent(Extent3D {
                width: surface.extent().width(),
                height: surface.extent().height(),
                depth: 1,
            })
            .with_mip_levels(1)
            .with_array_layers(1)
            .with_samples(msaa)
            .with_tiling(ImageTiling::OPTIMAL)
            .with_usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .with_sharing_mode(SharingMode::EXCLUSIVE);

        // Here we create the image
        let (image, _) = unsafe { vulkan.device().create_image(&image_create_info, None)? };

        info!("Created the depth image: {:?}", image.as_raw());

        // We want to know how much space we need to allocate the image.
        // We can do this by hand but it is **very** tedious, so instead Vulkan
        // provides a function to quickly get the memory requirements.
        let memory_requirements = unsafe { vulkan.device().get_image_memory_requirements(image.as_raw()) };

        // `Size` is a little helper for pretty-priting memory sized!
        // Let's give it a try!
        info!(
            "Got the memory requirements: size = {}, alignment = {}, memory type = {:032b}",
            Size::from(memory_requirements.size()),
            memory_requirements.alignment(),
            memory_requirements.memory_type_bits()
        );

        // Here by using the requirements and the properties, we try and find the
        // right memory type in which we want to allocate our memory.
        // We also tell it that we want the memory to be allocated on the GPU (local to the device).
        // This is a simple helper that Magritte provides.
        let memory_index = if let Some(index) = find_memory_type_index(
            &memory_requirements,
            &memory_properties,
            MemoryPropertyFlags::DEVICE_LOCAL,
        ) {
            index
        } else {
            error!("Could not find suitable memory");

            return Err(VulkanResultCodes::ERROR_OUT_OF_DEVICE_MEMORY);
        };

        // Here, we prepare where and how much memory we want to allocate using the previously
        // obtained values
        let image_allocate_info = MemoryAllocateInfo::default()
            .with_allocation_size(memory_requirements.size())
            .with_memory_type_index(memory_index);

        // Now, we allocate the memory
        let (image_memory, _) = unsafe { vulkan.device().allocate_memory(&image_allocate_info, None)? };

        // Here we bind the memory, this means that the image must now live longer than its memory!
        // ??? We need to ensure this is the case.
        unsafe {
            vulkan
                .device()
                .bind_image_memory(image.as_raw(), image_memory.as_raw(), 0)?;
        }

        // TODO: explain this more.
        commands.record_and_submit_setup(&[], &[], &[], |cmd| {
            // Here we create a barrier to initialize our depth buffer.
            let layout_transition_barriers = ImageMemoryBarrier::default()
                .with_image(image.as_raw())
                .with_dst_access_mask(
                    AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                )
                .with_new_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .with_old_layout(ImageLayout::UNDEFINED)
                .with_subresource_range(
                    ImageSubresourceRange::default()
                        .with_aspect_mask(ImageAspectFlags::DEPTH)
                        .with_layer_count(1)
                        .with_level_count(1),
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

            trace!("Recorded depth initialization commands");

            Ok(())
        })?;

        let image_view_info = ImageViewCreateInfo::default()
            .with_subresource_range(
                ImageSubresourceRange::default()
                    .with_aspect_mask(ImageAspectFlags::DEPTH)
                    .with_level_count(1)
                    .with_layer_count(1),
            )
            .with_image(image.as_raw())
            .with_format(image_create_info.format())
            .with_view_type(ImageViewType::_2D);

        let (image_view, _) = unsafe { image.create_image_view(&image_view_info, None)? };

        Ok((image, image_view, image_memory))
    }
}
