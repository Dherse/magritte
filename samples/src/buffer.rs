use std::{marker::PhantomData, mem::size_of};

use bytemuck::{Pod, Zeroable, cast_slice};
use log::{info, error};
use magritte::{vulkan1_0::{Buffer as VkBuffer, DeviceMemory, VulkanResultCodes, BufferCreateInfo, BufferUsageFlags, SharingMode, MemoryPropertyFlags, MemoryAllocateInfo}, Unique, AsRaw, size::Size, memory::find_memory_type_index};

use crate::vulkan::Vulkan;

pub struct Buffer<C: Pod + Zeroable> {
    /// The buffer
    pub buffer: Unique<VkBuffer>,

    /// The memory backing the buffer
    pub buffer_memory: Unique<DeviceMemory>,

    pub _phantom: PhantomData<C>,
}

impl<C> Buffer<C> where C: Pod + Zeroable {
    /// Creates a new buffer
    #[inline]
    pub fn new(
        vulkan: &Vulkan, 
        usage: BufferUsageFlags,
        data: &[C]
    ) -> Result<Self, VulkanResultCodes> {
        // First we get the memory properties, we will use this when allocating our image 
        let memory_properties = unsafe {
            vulkan.physical_device().get_physical_device_memory_properties()
        };

        // We compute the number of bytes we need
        let size = (size_of::<C>() * data.len()) as u64;

        // Then, we need to gather the information for creating the buffer,
        // in our case:
        //  - the size in bytes (the length of the data multiplied by the size of each element)
        //  - the usage
        //  - the fact that this buffer will only get accessed from a single queue
        let buffer_info = BufferCreateInfo::default()
            .set_size(size)
            .set_usage(usage)
            .set_sharing_mode(SharingMode::EXCLUSIVE);

        // We create the buffer handle
        let (buffer, _) = unsafe {
            vulkan.device().create_buffer(&buffer_info, None)?
        };

        info!(
            "Created buffer: {:?} of size: {} and usage: {:?}",
            buffer.as_raw(),
            Size::from(size),
            usage
        );

        // We get the memory requirements for this buffer
        let memory_requirements = unsafe {
            vulkan.device().get_buffer_memory_requirements(buffer.as_raw())
        };

        info!(
            "Got the memory requirements: size = {}, alignment = {}, memory type = {:032b}",
            Size::from(memory_requirements.size()),
            memory_requirements.alignment(),
            memory_requirements.memory_type_bits()
        );

        // We use a helper function to figure out where to put allocate the buffer.
        // Here, we request specifically that the memory be:
        // - visible by the host (CPU), that means we can read and write to it
        // - that the memory be coherent, that means we don't need to flush any caches.
        //
        // On some cards, there is what is called the Base Address Register (BAR) which
        // is a region of memory (typically 256 MiB) that is accessible to the CPU directly.
        // This memory region is deal because it is **on** the GPU and still fully accessible
        // to the CPU, this is why we first try to find that memory, then we try to find
        // memory with looser requirements
        let memory_index = if let Some(index) = find_memory_type_index(
            &memory_requirements,
            &memory_properties,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT | MemoryPropertyFlags::DEVICE_LOCAL,
        ).or_else(|| find_memory_type_index(
            &memory_requirements,
            &memory_properties,
            MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
        )) {
            index
        } else {
            error!("Could not find suitable memory");

            return Err(VulkanResultCodes::ERROR_OUT_OF_DEVICE_MEMORY);
        };

        info!(
            "Got the memory index: {} with the flags: {:?}", 
            memory_index,
            memory_properties.memory_types()[memory_index as usize].property_flags
        );

        // Here, we prepare where and how much memory we want to allocate using the previously
        // obtained values
        let allocate_info = MemoryAllocateInfo::default()
            .set_allocation_size(memory_requirements.size)
            .set_memory_type_index(memory_index);

        // Now, we allocate the memory
        let (buffer_memory, _) = unsafe {
            vulkan.device().allocate_memory(&allocate_info, None)?
        };

        // We map the memory into the CPU address space, thuse we get a mutable pointer to the memory.
        let mut ptr = std::ptr::null_mut();
        unsafe {
            vulkan.device().map_memory(buffer_memory.as_raw(), 0, size, Default::default(), &mut ptr).result()?;
        }

        // We check if the pointer is null for improved safety.
        if ptr.is_null() {
            error!("Did not map successfully");
            
            return Err(VulkanResultCodes::ERROR_INITIALIZATION_FAILED);
        }

        // We make it have the shortest lifetime possible to avoid any potential errors!
        {
            // We map the pointer into a slice of the right slice.
            let byte_slice: &mut [u8] = unsafe {
                std::slice::from_raw_parts_mut(ptr.cast(), size as usize)
            };
    
            // Here, we copy the data inside of the slice.
            // We use [`bytemuck::cast_slice`] to transform the slice of `C` into a slice of [`u8`]
            byte_slice.copy_from_slice(cast_slice(data));
        }

        // We unmap the memory
        unsafe {
            vulkan.device().unmap_memory(buffer_memory.as_raw());
        }

        // Here we bind the memory, this means that the image must now live longer than its memory!
        // ⚠ We need to ensure this is the case.
        unsafe {
            vulkan.device().bind_buffer_memory(buffer.as_raw(), buffer_memory.as_raw(), 0)?;
        }

        Ok(Self {
            buffer,
            buffer_memory,
            _phantom: PhantomData,
        })
    }

    /// Get a reference to the buffer's buffer.
    pub fn buffer(&self) -> &Unique<VkBuffer> {
        &self.buffer
    }

    /// Get a reference to the buffer's buffer memory.
    pub fn buffer_memory(&self) -> &Unique<DeviceMemory> {
        &self.buffer_memory
    }
}