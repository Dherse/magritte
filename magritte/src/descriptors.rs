use crate::{vulkan1_0::{Device, DescriptorPoolCreateFlags, DescriptorPool, DescriptorPoolSize, DescriptorType, DescriptorPoolCreateInfo}, Unique, VulkanResult, vulkan1_3::DescriptorPoolInlineUniformBlockCreateInfo, Chain, Version};


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DescriptorCounts {
    pub sampler: u32,
    pub combined_image_sampler: u32,
    pub sampled_image: u32,
    pub storage_image: u32,
    pub uniform_texel_buffer: u32,
    pub storage_texel_buffer: u32,
    pub uniform_buffer: u32,
    pub storage_buffer: u32,
    pub uniform_buffer_dynamic: u32,
    pub storage_buffer_dynamic: u32,
    pub input_attachment: u32,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub acceleration_structure: u32,
    pub inline_uniform_block_bindings: u32,
}

macro_rules! desc_set {
    (
        $this:expr;
        $($(#[$outer:meta])* $name:ident),*
    ) => {
        $(
            $(#[$outer])*
            pub fn $name(&mut self, count: u32) -> &mut Self {
                self.$name += count;
                self
            }
        )*
    };
}

impl DescriptorCounts {
    desc_set! {
        self;
        sampler, combined_image_sampler, sampled_image, storage_image,
        uniform_texel_buffer, storage_texel_buffer, uniform_buffer,
        storage_buffer, uniform_buffer_dynamic, storage_buffer_dynamic,
        input_attachment, #[cfg(feature = "VK_KHR_acceleration_structure")] acceleration_structure, inline_uniform_block_bindings
    }
}

macro_rules! desc_init {
    (
        $descriptor_count:expr;
        $array:expr;
        $len: ident;
        $($(#[$outer:meta])* $name:ident -> $ty:ident),*
    ) => {
        $(
            $(#[$outer])*
            if $descriptor_count.sampler > 0 {
                $array[$len].type_ = DescriptorType::$ty;
                $array[$len].descriptor_count = $descriptor_count.$name;
                $len += 1;
            }
        )*
    };
}

impl DescriptorPool {
    pub unsafe fn create(
        device: &Unique<Device>,
        descriptor_count: DescriptorCounts,
        max_sets: u32,
        flags: DescriptorPoolCreateFlags,
    ) -> VulkanResult<Unique<Self>> {
        let mut descriptors = [DescriptorPoolSize::default(); if cfg!(feature = "VK_KHR_acceleration_structure") { 12 } else { 11 }];
        let mut len = 0;

        desc_init! {
            descriptor_count; descriptors; len;
            sampler -> SAMPLER,
            combined_image_sampler -> COMBINED_IMAGE_SAMPLER,
            sampled_image -> SAMPLED_IMAGE,
            storage_image -> STORAGE_IMAGE,
            uniform_texel_buffer -> UNIFORM_TEXEL_BUFFER,
            storage_texel_buffer -> STORAGE_TEXEL_BUFFER,
            uniform_buffer -> UNIFORM_BUFFER,
            storage_buffer -> STORAGE_BUFFER,
            uniform_buffer_dynamic -> UNIFORM_BUFFER_DYNAMIC,
            storage_buffer_dynamic -> STORAGE_BUFFER_DYNAMIC,
            input_attachment -> INPUT_ATTACHMENT,
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            acceleration_structure -> ACCELERATION_STRUCTURE
        }

        let mut inline_block_info = DescriptorPoolInlineUniformBlockCreateInfo::default()
            .with_max_inline_uniform_block_bindings(descriptor_count.inline_uniform_block_bindings);

        let info = DescriptorPoolCreateInfo::default()
            .with_flags(flags)
            .with_max_sets(max_sets)
            .with_pool_sizes(&descriptors[..len]);

        
        let info = if descriptor_count.inline_uniform_block_bindings > 0 {
            #[cfg(feature = "VK_EXT_inline_uniform_block")]
            if device.instance().metadata().version() >= Version::VULKAN1_3 {
                info.chain(&mut inline_block_info)
            } else if device.metadata().ext_inline_uniform_block {
                info.chain(&mut inline_block_info)
            } else {
                info
            }

            #[cfg(not(feature = "VK_EXT_inline_uniform_block"))]
            if device.instance().metadata().version() >= Version::VULKAN1_3 {
                info.chain(&mut inline_block_info)
            } else {
                info
            }
        } else {
            info
        };

        device.create_descriptor_pool(&info, None)
    }
}