use std::{marker::PhantomData, mem::size_of};

use bytemuck::{cast_slice, Pod, Zeroable};
use magritte::vulkan1_0::{BufferCreateInfo, BufferUsageFlags, MemoryPropertyFlags, SharingMode, VulkanResultCodes};
use magritte_vma::VmaBuffer;

use crate::vulkan::Vulkan;

pub struct Buffer<C: Pod + Zeroable> {
    pub buffer: VmaBuffer,

    pub _phantom: PhantomData<C>,
}

impl<C> Buffer<C>
where
    C: Pod + Zeroable,
{
    /// Creates a new buffer
    #[inline]
    pub fn new(vulkan: &Vulkan, usage: BufferUsageFlags, data: &[C]) -> Result<Self, VulkanResultCodes> {
        // We compute the number of bytes we need
        let size = (size_of::<C>() * data.len()) as u64;

        // Then, we need to gather the information for creating the buffer,
        // in our case:
        //  - the size in bytes (the length of the data multiplied by the size of each element)
        //  - the usage
        //  - the fact that this buffer will only get accessed from a single queue
        let buffer_info = BufferCreateInfo::default()
            .with_size(size)
            .with_usage(usage)
            .with_sharing_mode(SharingMode::EXCLUSIVE);

        let buffer = vulkan.allocator().create_buffer(
            &buffer_info,
            magritte_vma::AllocationCreateFlags::empty(),
            magritte_vma::BufferUsage::Flags {
                required: MemoryPropertyFlags::HOST_VISIBLE,
                preferred: MemoryPropertyFlags::DEVICE_LOCAL | MemoryPropertyFlags::HOST_COHERENT,
            },
            None,
            None,
            None,
        )?;

        buffer
            .allocation()
            .map_range(0..size as usize)
            .expect("Failed to map")
            .copy_from_slice(cast_slice(data));

        Ok(Self {
            buffer,
            _phantom: PhantomData,
        })
    }

    /// Get a reference to the buffer's buffer.
    pub fn buffer(&self) -> &VmaBuffer {
        &self.buffer
    }
}
