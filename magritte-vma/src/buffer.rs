use std::{ffi::c_void, sync::atomic::AtomicBool};

use magritte::{
    vulkan1_0::{Buffer as VkBuffer, BufferCreateInfo, DeviceSize, MemoryPropertyFlags, VulkanResultCodes},
    AsRaw, Unique,
};

use crate::{
    ffi::{vmaCreateBuffer, vmaCreateBufferWithAlignment, vmaDestroyBuffer, AllocationInfo},
    Allocation, AllocationCreateFlags, AllocationCreateInfo, Allocator, MemoryUsage, Pool,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BufferUsage {
    /// Determined automatically
    Auto,

    /// Based on its usage
    Usage(MemoryUsage),

    /// Based on a set of flags
    Flags {
        required: MemoryPropertyFlags,
        preferred: MemoryPropertyFlags,
    },
}

impl Default for BufferUsage {
    fn default() -> Self {
        BufferUsage::Auto
    }
}

pub struct VmaBuffer {
    allocator: Unique<Allocator>,
    pool: Option<Unique<Pool>>,
    allocation: Unique<Allocation>,
    allocation_info: AllocationInfo,
    buffer: Unique<VkBuffer>,
}

impl AsRaw for VmaBuffer {
    type Raw = VkBuffer;

    fn as_raw(&self) -> Self::Raw {
        self.buffer.as_raw()
    }
}

impl Drop for VmaBuffer {
    fn drop(&mut self) {
        unsafe {
            vmaDestroyBuffer(self.allocator.as_raw(), self.buffer.as_raw(), self.allocation.as_raw());
        }
    }
}

impl VmaBuffer {
    pub(crate) fn new<'lt>(
        allocator: &Unique<Allocator>,
        buffer_create_info: &BufferCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        usage: BufferUsage,
        memory_type_bits: Option<u32>,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
        min_alignment: Option<DeviceSize>,
    ) -> Result<Self, VulkanResultCodes> {
        let mut buffer = VkBuffer::null();
        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let (usage, required_flags, preferred_flags) = match usage {
            BufferUsage::Auto => (
                MemoryUsage::AUTO,
                MemoryPropertyFlags::empty(),
                MemoryPropertyFlags::empty(),
            ),
            BufferUsage::Usage(usage) => (usage, MemoryPropertyFlags::empty(), MemoryPropertyFlags::empty()),
            BufferUsage::Flags { required, preferred } => (MemoryUsage::UNKNOWN, required, preferred),
        };

        let allocation_create_info = AllocationCreateInfo {
            flags,
            usage,
            required_flags,
            preferred_flags,
            memory_type_bits: memory_type_bits.unwrap_or_default(),
            pool: Pool::null(),
            user_data: user_data.unwrap_or_else(std::ptr::null_mut),
            priority: priority.unwrap_or_default(),
        };

        let res = unsafe {
            if let Some(min_alignment) = min_alignment {
                vmaCreateBufferWithAlignment(
                    allocator.as_raw(),
                    buffer_create_info,
                    &allocation_create_info,
                    min_alignment,
                    &mut buffer,
                    &mut allocation,
                    &mut allocation_info,
                )
            } else {
                vmaCreateBuffer(
                    allocator.as_raw(),
                    buffer_create_info,
                    &allocation_create_info,
                    &mut buffer,
                    &mut allocation,
                    &mut allocation_info,
                )
            }
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok(Self {
                allocator: allocator.clone(),
                pool: None,
                allocation: unsafe {
                    Unique::new(allocator, allocation, (None, allocation_info, AtomicBool::new(true)))
                },
                allocation_info,
                buffer: unsafe { Unique::new(allocator.parent(), buffer, AtomicBool::new(true)) },
            }),
            other => Err(other),
        }
    }

    pub(crate) fn new_pool<'lt>(
        allocator: &Unique<Allocator>,
        buffer_create_info: &BufferCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        pool: &Unique<Pool>,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
        min_alignment: Option<DeviceSize>,
    ) -> Result<Self, VulkanResultCodes> {
        let mut buffer = VkBuffer::null();
        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let allocation_create_info = AllocationCreateInfo {
            flags,
            usage: MemoryUsage::UNKNOWN,
            required_flags: MemoryPropertyFlags::empty(),
            preferred_flags: MemoryPropertyFlags::empty(),
            memory_type_bits: 0,
            pool: pool.as_raw(),
            user_data: user_data.unwrap_or_else(std::ptr::null_mut),
            priority: priority.unwrap_or_default(),
        };

        let res = unsafe {
            if let Some(min_alignment) = min_alignment {
                vmaCreateBufferWithAlignment(
                    allocator.as_raw(),
                    buffer_create_info,
                    &allocation_create_info,
                    min_alignment,
                    &mut buffer,
                    &mut allocation,
                    &mut allocation_info,
                )
            } else {
                vmaCreateBuffer(
                    allocator.as_raw(),
                    buffer_create_info,
                    &allocation_create_info,
                    &mut buffer,
                    &mut allocation,
                    &mut allocation_info,
                )
            }
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok(Self {
                allocator: allocator.clone(),
                pool: None,
                allocation: unsafe {
                    Unique::new(allocator, allocation, (None, allocation_info, AtomicBool::new(true)))
                },
                allocation_info,
                buffer: unsafe { Unique::new(allocator.parent(), buffer, AtomicBool::new(true)) },
            }),
            other => Err(other),
        }
    }

    /// Get a reference to the buffer's allocator.
    pub fn allocator(&self) -> &Unique<Allocator> {
        &self.allocator
    }

    /// Get a reference to the buffer's pool.
    pub fn pool(&self) -> Option<&Unique<Pool>> {
        self.pool.as_ref()
    }

    /// Get a reference to the buffer's allocation.
    pub fn allocation(&self) -> &Unique<Allocation> {
        &self.allocation
    }

    /// Get a reference to the buffer's allocation info.
    pub fn allocation_info(&self) -> AllocationInfo {
        self.allocation_info
    }

    /// Get a reference to the buffer's buffer.
    pub fn buffer(&self) -> &Unique<VkBuffer> {
        &self.buffer
    }
}
