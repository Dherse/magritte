use log::{info, error, trace};
use magritte::{vulkan1_0::{DeviceMemory, ImageView, Image, VulkanResultCodes, ImageCreateInfo, ImageType, Format, SampleCountFlagBits, ImageTiling, ImageUsageFlags, SharingMode, Extent3D, MemoryPropertyFlags, MemoryAllocateInfo, AccessFlags, ImageSubresourceRange, ImageAspectFlags, ImageLayout, ImageMemoryBarrier, PipelineStageFlags, DependencyFlags, ImageViewCreateInfo, ImageViewType}, Unique, AsRaw, size::Size, memory::find_memory_type_index};

use crate::{vulkan::Vulkan, surface::Surface, commands::Commands};


/// A simple container for a depth buffer and its memory.
pub struct Depth {
    /// The image backing the depth buffer
    pub image: Unique<Image>,

    /// The view to access the depth buffer
    pub image_view: Unique<ImageView>,

    /// The memory backing the depth buffer
    pub image_memory: Unique<DeviceMemory>
}

impl Depth {
    /// Creates a new depth buffer
    #[inline]
    pub fn new(vulkan: &Vulkan, commands: &Commands, surface: &Surface) -> Result<Self, VulkanResultCodes> {
        let (image, image_view, image_memory) = Self::create_new_image_view_memory(vulkan, commands, surface)?;

        Ok(Self {
            image,
            image_view,
            image_memory,
        })
    }

    /// Sizes the depth image
    #[inline]
    pub fn resize(&mut self, vulkan: &Vulkan, commands: &Commands, surface: &Surface) -> Result<(), VulkanResultCodes> {
        let (image, image_view, image_memory) = Self::create_new_image_view_memory(vulkan, commands, surface)?;

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
        surface: &Surface
    ) -> Result<(Unique<Image>, Unique<ImageView>, Unique<DeviceMemory>), VulkanResultCodes> {
        // First we get the memory properties, we will use this when allocating our image 
        let memory_properties = unsafe {
            vulkan.physical_device().get_physical_device_memory_properties()
        };

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
            .set_image_type(ImageType::_2_D)
            .set_format(Format::D16_UNORM)
            .set_extent(Extent3D {
                width: surface.extent().width(),
                height: surface.extent().height(),
                depth: 1,
            })
            .set_mip_levels(1)
            .set_array_layers(1)
            .set_samples(SampleCountFlagBits::_1)
            .set_tiling(ImageTiling::OPTIMAL)
            .set_usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .set_sharing_mode(SharingMode::EXCLUSIVE);

        // Here we create the image
        let (image, _) = unsafe {
            vulkan.device().create_image(&image_create_info, None)?
        };

        info!("Created the depth image: {:?}", image.as_raw());

        // We want to know how much space we need to allocate the image.
        // We can do this by hand but it is **very** tedious, so instead Vulkan
        // provides a function to quickly get the memory requirements.
        let memory_requirements = unsafe {
            vulkan.device().get_image_memory_requirements(image.as_raw())
        };

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
            .set_allocation_size(memory_requirements.size())
            .set_memory_type_index(memory_index);

        // Now, we allocate the memory
        let (image_memory, _) = unsafe {
            vulkan.device().allocate_memory(&image_allocate_info, None)?
        };

        // Here we bind the memory, this means that the image must now live longer than its memory!
        // ⚠ We need to ensure this is the case.
        unsafe {
            vulkan.device().bind_image_memory(image.as_raw(), image_memory.as_raw(), 0)?;
        }

        // TODO: explain this more.
        commands.record_and_submit_setup(&[], &[], &[], |cmd| {
            // Here we create a barrier to initialize our depth buffer.
            let layout_transition_barriers = ImageMemoryBarrier::default()
                .set_image(image.as_raw())
                .set_dst_access_mask(
                    AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                        | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                )
                .set_new_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                .set_old_layout(ImageLayout::UNDEFINED)
                .set_subresource_range(
                   ImageSubresourceRange::default()
                        .set_aspect_mask(ImageAspectFlags::DEPTH)
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
                    std::slice::from_ref(&layout_transition_barriers)
                );
            }

            trace!("Recorded depth initialization commands");

            Ok(())
        })?;

        let image_view_info = ImageViewCreateInfo::default()
            .set_subresource_range(
                ImageSubresourceRange::default()
                    .set_aspect_mask(ImageAspectFlags::DEPTH)
                    .set_level_count(1)
                    .set_layer_count(1),
            )
            .set_image(image.as_raw())
            .set_format(image_create_info.format())
            .set_view_type(ImageViewType::_2_D);

        let (image_view, _) = unsafe {
            image.create_image_view(&image_view_info, None)?
        };

        Ok((
            image,
            image_view,
            image_memory,
        ))
    }
}