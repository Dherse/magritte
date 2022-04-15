use std::ffi::c_void;

use magritte::{
    vulkan1_0::{Image as VkImage, ImageCreateInfo, VulkanResultCodes, MemoryPropertyFlags},
    AsRaw, Unique,
};

use crate::{
    ffi::{vmaCreateImage, AllocationInfo, vmaDestroyImage},
    Allocation, AllocationCreateInfo, Allocator, Pool, AllocationCreateFlags, MemoryUsage,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageUsage {
    /// Determined automatically
    Auto,

    /// Based on its usage
    Usage(MemoryUsage),

    /// Based on a set of flags
    Flags {
        required: MemoryPropertyFlags,
        preferred: MemoryPropertyFlags,
    }
}

impl Default for ImageUsage {
    fn default() -> Self {
        ImageUsage::Auto
    }
}

pub struct VmaImage {
    allocator: Unique<Allocator>,
    pool: Option<Unique<Pool>>,
    allocation: Allocation,
    allocation_info: AllocationInfo,
    image: VkImage,
}

impl AsRaw for VmaImage {
    type Raw = VkImage;

    fn as_raw(&self) -> Self::Raw {
        self.image
    }
}

impl Drop for VmaImage {
    fn drop(&mut self) {
        unsafe {
            vmaDestroyImage(self.allocator.as_raw(), self.image, self.allocation);
        }
    }
}

impl VmaImage {
    pub(crate) fn new<'lt>(
        allocator: &Unique<Allocator>,
        image_create_info: &ImageCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        usage: ImageUsage,
        memory_type_bits: Option<u32>,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<Self, VulkanResultCodes> {
        let mut image = VkImage::null();
        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let(usage, required_flags, preferred_flags) = match usage {
            ImageUsage::Auto => (MemoryUsage::AUTO, MemoryPropertyFlags::empty(), MemoryPropertyFlags::empty()),
            ImageUsage::Usage(usage) => (usage, MemoryPropertyFlags::empty(), MemoryPropertyFlags::empty()),
            ImageUsage::Flags { required, preferred } => (MemoryUsage::UNKNOWN, required, preferred),
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
            vmaCreateImage(
                allocator.as_raw(),
                image_create_info,
                &allocation_create_info,
                &mut image,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok(Self {
                allocator: allocator.clone(),
                pool: None,
                allocation,
                allocation_info,
                image,
            }),
            other => Err(other)
        }
    }

    pub(crate) fn new_pool<'lt>(
        allocator: &Unique<Allocator>,
        image_create_info: &ImageCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        pool: &Unique<Pool>,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<Self, VulkanResultCodes> {
        let mut image = VkImage::null();
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
            vmaCreateImage(
                allocator.as_raw(),
                image_create_info,
                &allocation_create_info,
                &mut image,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok(Self {
                allocator: allocator.clone(),
                pool: Some(pool.clone()),
                allocation,
                allocation_info,
                image,
            }),
            other => Err(other)
        }
    }

    /// Get a reference to the image's allocator.
    pub fn allocator(&self) -> &Unique<Allocator> {
        &self.allocator
    }

    /// Get a reference to the image's pool.
    pub fn pool(&self) -> Option<&Unique<Pool>> {
        self.pool.as_ref()
    }

    /// Get a copy of the image's allocation.
    pub fn allocation(&self) -> Allocation {
        self.allocation
    }

    /// Get a reference to the image's allocation info.
    pub fn allocation_info(&self) -> AllocationInfo {
        self.allocation_info
    }

    /// Get a copy of the image's image.
    pub fn image(&self) -> VkImage {
        self.image
    }
}
