use std::{ffi::{CStr, c_void}, ops::Not, os::raw::c_char};

use magritte::{
    vulkan1_0::{Buffer, DeviceSize, Image, MemoryRequirements, VulkanResultCodes, BufferCreateInfo, ImageCreateInfo},
    AsRaw, Handle, Unique,
};
use smallvec::SmallVec;

use crate::{
    allocator::Allocator,
    defragmentation_context::DefragmentationContextHandle,
    ffi::{
        vmaAllocateMemory, vmaAllocateMemoryForBuffer, vmaAllocateMemoryForImage, vmaAllocateMemoryPages,
        vmaBeginDefragmentation, vmaCalculatePoolStatistics, vmaCheckPoolCorruption, vmaDestroyPool, vmaGetPoolName,
        vmaGetPoolStatistics, vmaSetPoolName, AllocationInfo, DefragmentationInfo, DetailedStatistics, Statistics,
    },
    Allocation, AllocationCreateInfo, DefragmentationContext, DefragmentationFlags, buffer::VmaBuffer, AllocationCreateFlags, image::VmaImage,
};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Pool(pub *mut ());
impl Pool {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }

    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> *mut () {
        self.0
    }

    /// Retrieves statistics of existing [`Pool`] object.
    pub fn statistics(self: &Unique<Pool>) -> Statistics {
        let mut out = unsafe { std::mem::zeroed() };

        unsafe {
            vmaGetPoolStatistics(self.parent().as_raw(), self.as_raw(), &mut out);
        }

        out
    }

    /// Retrieves detailed statistics of existing [`Pool`] object.
    pub fn calculate_statistics(self: &Unique<Pool>) -> DetailedStatistics {
        let mut out = unsafe { std::mem::zeroed() };

        unsafe {
            vmaCalculatePoolStatistics(self.parent().as_raw(), self.as_raw(), &mut out);
        }

        out
    }

    /// Checks magic number in margins around all allocations in given memory pool in search
    /// for corruptions.
    ///
    /// Corruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to
    /// nonzero,
    /// `VMA_DEBUG_MARGIN` is defined to nonzero and the pool is created in memory type that is
    /// `HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/debugging_memory_usage.html#debugging_memory_usage_corruption_detection).
    pub fn check_corruption(self: &Unique<Pool>) -> MemoryCorruptionState {
        let res = unsafe { vmaCheckPoolCorruption(self.parent().as_raw(), self.as_raw()) };

        match res {
            VulkanResultCodes::ERROR_FEATURE_NOT_PRESENT => MemoryCorruptionState::Disabled,
            VulkanResultCodes::SUCCESS => MemoryCorruptionState::NoCorruption,
            VulkanResultCodes::ERROR_UNKNOWN => MemoryCorruptionState::Corrupted,
            other => unreachable!("unexpected vulkan result: {}", other),
        }
    }

    /// Retrieves name of a custom pool.
    ///
    /// Note that this function returns an internal copy of the string.
    pub fn name(self: &Unique<Pool>) -> Option<String> {
        let mut res: *const c_char = std::ptr::null();

        unsafe {
            vmaGetPoolName(self.parent().as_raw(), self.as_raw(), &mut res);
        }

        res.is_null()
            .not()
            .then(|| unsafe { CStr::from_ptr(res).to_string_lossy().to_string() })
    }

    /// Sets name of a custom pool.
    ///
    /// Note that this function makes an internal copy of the string.
    pub fn set_name(self: &Unique<Pool>, name: &CStr) {
        unsafe {
            vmaSetPoolName(self.parent().as_raw(), self.as_raw(), name.as_ptr());
        }
    }

    /// General purpose memory allocation.
    ///
    /// This will always **set** the [`AllocationCreateInfo::pool`] to [`Self::as_raw`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory`] function instead.
    pub fn allocate_memory(
        self: &Unique<Pool>,
        memory_requirements: &MemoryRequirements,
        create_info: &AllocationCreateInfo,
    ) -> Result<(Unique<Allocation>, AllocationInfo), VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = self.as_raw();

        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let res = unsafe {
            vmaAllocateMemory(
                self.parent().as_raw(),
                memory_requirements,
                &create_info,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok((
                unsafe { Unique::new(self.parent(), allocation, (Some(self.clone()), allocation_info)) },
                allocation_info,
            )),
            other => Err(other),
        }
    }

    /// General purpose memory allocationfor multiple allocation objects at once.
    ///
    /// Word "pages" is just a suggestion to use this function to allocate pieces of memory needed
    /// for sparse binding. It is just a general purpose allocation function able to make
    /// multiple allocations at once. It may be internally optimized to be more efficient than
    /// calling [`Self::allocate_memory`] `allocation_count` times.
    ///
    /// This will always **set** the [`AllocationCreateInfo::pool`] to [`Self::as_raw`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory`] function instead.
    pub fn allocate_memory_pages(
        self: &Unique<Pool>,
        memory_requirements: &MemoryRequirements,
        create_info: &AllocationCreateInfo,
        allocation_count: usize,
    ) -> Result<SmallVec<[(Unique<Allocation>, AllocationInfo); 8]>, VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = self.as_raw();

        let mut allocations = SmallVec::<[_; 8]>::from_elem(Allocation::null(), allocation_count);
        let mut allocation_infos = SmallVec::<[_; 8]>::from_elem(unsafe { std::mem::zeroed() }, allocation_count);

        let res = unsafe {
            vmaAllocateMemoryPages(
                self.parent().as_raw(),
                memory_requirements,
                &create_info,
                allocation_count,
                allocations.as_mut_ptr(),
                allocation_infos.as_mut_ptr(),
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok(allocations
                .into_iter()
                .zip(allocation_infos.into_iter())
                .map(|(alloc, allocation_info)| {
                    (
                        unsafe { Unique::new(self.parent(), alloc, (Some(self.clone()), allocation_info)) },
                        allocation_info,
                    )
                })
                .collect()),
            other => Err(other),
        }
    }

    /// Allocates memory suitable for given [`Buffer`].
    ///
    /// It only creates [`Allocation`]. To bind the memory to the buffer, use
    /// [`Self::bind_buffer_memory`].
    ///
    /// This is a special-purpose function. In most cases you should use [`Self::create_buffer`].
    ///
    /// This will always **set** the [`AllocationCreateInfo::pool`] to [`Self::as_raw`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory_for_buffer`] function
    /// instead.
    pub fn allocate_memory_for_buffer(
        self: &Unique<Pool>,
        buffer: &Unique<Buffer>,
        create_info: &AllocationCreateInfo,
    ) -> Result<(Unique<Allocation>, AllocationInfo), VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = self.as_raw();

        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let res = unsafe {
            vmaAllocateMemoryForBuffer(
                self.parent().as_raw(),
                buffer.as_raw(),
                &create_info,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok((
                unsafe { Unique::new(self.parent(), allocation, (Some(self.clone()), allocation_info)) },
                allocation_info,
            )),
            other => Err(other),
        }
    }

    /// Allocates memory suitable for given [`Image`].
    ///
    /// It only creates [`Allocation`]. To bind the memory to the image, use
    /// [`Self::bind_image_memory`].
    ///
    /// This is a special-purpose function. In most cases you should use [`Self::create_image`].
    ///
    /// This will always **set** the [`AllocationCreateInfo::pool`] to [`Self::as_raw`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory_for_image`] function
    /// instead.
    pub fn allocate_memory_for_image(
        self: &Unique<Pool>,
        image: &Unique<Image>,
        create_info: &AllocationCreateInfo,
    ) -> Result<(Unique<Allocation>, AllocationInfo), VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = self.as_raw();

        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let res = unsafe {
            vmaAllocateMemoryForImage(
                self.parent().as_raw(),
                image.as_raw(),
                &create_info,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok((
                unsafe { Unique::new(self.parent(), allocation, (Some(self.clone()), allocation_info)) },
                allocation_info,
            )),
            other => Err(other),
        }
    }

    /// Begins defragmentation process.
    /// For more information about defragmentation, see documentation chapter:
    /// [Defragmentation](https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/defragmentation.html).
    pub fn begin_defragmentation(
        self: &Unique<Pool>,
        flags: DefragmentationFlags,
        max_bytes_per_pass: Option<DeviceSize>,
        max_allocations_per_pass: Option<u32>,
    ) -> Result<DefragmentationContext, VulkanResultCodes> {
        let info = DefragmentationInfo {
            flags,
            pool: self.as_raw(),
            max_bytes_per_pass: max_bytes_per_pass.unwrap_or_default(),
            max_allocations_per_pass: max_allocations_per_pass.unwrap_or_default(),
        };

        let mut out = DefragmentationContextHandle::null();

        let res = unsafe { vmaBeginDefragmentation(self.parent().as_raw(), &info, &mut out) };

        match res {
            VulkanResultCodes::SUCCESS => {
                Ok(unsafe { DefragmentationContext::new(self.parent().clone(), Some(self.clone()), out) })
            },
            other => Err(other),
        }
    }

    /// Creates a new [`VmaBuffer`], allocates and binds memory for it.
    ///
    /// This function automatically:
    /// - Creates buffer.
    /// - Allocates appropriate memory for it.
    /// - Binds the buffer with the memory.
    ///
    /// If any of these operations fail, buffer and allocation are not created,
    /// and it returns an [`Err`].
    pub fn create_buffer<'lt>(
        self: &Unique<Pool>,
        buffer_create_info: &BufferCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<VmaBuffer, VulkanResultCodes> {
        VmaBuffer::new_pool(self.parent(), buffer_create_info, flags, self, priority, user_data, None)
    }

    /// Creates a new [`VmaBuffer`] with additional minimum alignment.
    ///
    /// This function automatically:
    /// - Creates buffer.
    /// - Allocates appropriate memory for it.
    /// - Binds the buffer with the memory.
    ///
    /// If any of these operations fail, buffer and allocation are not created,
    /// and it returns an [`Err`].
    pub fn create_buffer_with_alignment<'lt>(
        self: &Unique<Pool>,
        buffer_create_info: &BufferCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        alignment: DeviceSize,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<VmaBuffer, VulkanResultCodes> {
        VmaBuffer::new_pool(self.parent(), buffer_create_info, flags, self, priority, user_data, Some(alignment))
    }

    /// Creates a new [`VmaImage`], allocates and binds memory for it.
    ///
    /// This function automatically:
    /// - Creates image.
    /// - Allocates appropriate memory for it.
    /// - Binds the image with the memory.
    ///
    /// If any of these operations fail, image and allocation are not created,
    /// and it returns an [`Err`].
    pub fn create_image<'lt>(
        self: &Unique<Pool>,
        image_create_info: &ImageCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<VmaImage, VulkanResultCodes> {
        VmaImage::new_pool(self.parent(), image_create_info, flags, self, priority, user_data)
    }
}

unsafe impl Send for Pool {}
unsafe impl Sync for Pool {}

impl Default for Pool {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Pool {
    type Parent = Unique<Allocator>;
    type VTable = ();
    type Metadata = ();
    type Storage = *mut ();

    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }

    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }

    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        vmaDestroyPool(self.parent().as_raw(), self.as_raw());
    }

    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {
        ()
    }
}

/// The state of the memory corruption of a pool
pub enum MemoryCorruptionState {
    /// Memory corruption is disabled
    Disabled,

    /// There is no corruption
    NoCorruption,

    /// There is corruption
    Corrupted,
}
