use std::{
    ffi::{c_void, CStr},
    ops::{Bound, Deref, DerefMut, RangeBounds},
    sync::atomic::{AtomicBool, Ordering},
};

use log::error;
use magritte::{
    vulkan1_0::{Buffer, BufferCreateInfo, DeviceSize, Image, ImageCreateInfo, MemoryPropertyFlags, VulkanResultCodes},
    AsRaw, Handle, Unique,
};

use crate::{
    allocator::Allocator,
    ffi::{
        vmaBindBufferMemory, vmaBindBufferMemory2, vmaBindImageMemory, vmaBindImageMemory2, vmaCreateAliasingBuffer,
        vmaCreateAliasingImage, vmaFlushAllocation, vmaFreeMemory, vmaGetAllocationInfo,
        vmaGetAllocationMemoryProperties, vmaInvalidateAllocation, vmaMapMemory, vmaSetAllocationName,
        vmaSetAllocationUserData, vmaUnmapMemory, AllocationInfo,
    },
    pool::Pool,
};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Allocation(pub *mut ());

impl Allocation {
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

    /// Returns current information about specified allocation.
    ///
    /// Current parameters of given allocation are returned.
    ///
    /// Although this function doesn't lock any mutex, so it should be quite efficient, you should
    /// avoid calling it too often. You can retrieve same AllocationInfo structure while
    /// creating your resource, from function [`Allocator::create_buffer`],
    /// [`Allocator::create_image`]. You can remember it if you are sure parameters don't change
    /// (e.g. due to defragmentation).
    pub fn info(self: &Unique<Allocation>) -> AllocationInfo {
        let mut out = unsafe { std::mem::zeroed() };

        unsafe {
            vmaGetAllocationInfo(self.parent().as_raw(), self.as_raw(), &mut out);
        }

        out
    }

    /// Sets `user_data` in given allocation to new value.
    ///
    /// The value of pointer `user_data` is copied to allocation's `user_data`.
    /// It is opaque, so you can use it however you want - e.g.
    /// as a pointer, ordinal number or some handle to you own data.
    pub fn set_user_data(self: &Unique<Allocation>, user_data: *mut c_void) {
        unsafe {
            vmaSetAllocationUserData(self.parent().as_raw(), self.as_raw(), user_data);
        }
    }

    /// Sets name of an allocation.
    ///
    /// Note that this function makes an internal copy of the string.
    pub fn set_name(self: &Unique<Allocation>, name: &CStr) {
        unsafe {
            vmaSetAllocationName(self.parent().as_raw(), self.as_raw(), name.as_ptr());
        }
    }

    /// Given an allocation, returns Property Flags of its memory type.
    ///
    /// This is just a convenience function. Same information can be obtained using
    /// [`Self::info`] & [`Allocator::memory_properties`].
    pub fn memory_properties(self: &Unique<Allocation>) -> MemoryPropertyFlags {
        let mut out = MemoryPropertyFlags::empty();

        unsafe {
            vmaGetAllocationMemoryProperties(self.parent().as_raw(), self.as_raw(), &mut out);
        }

        out
    }

    /// Maps the entire range into the CPU's address space.
    ///
    /// Automatically invalidates the GPU caches upon mapping if necessary in order to make
    /// all GPU-side writes available to the CPU. This can be disabled using
    /// [`Self::map_no_invalidate`].
    pub fn map<'this>(self: &'this Unique<Allocation>) -> Option<Mapped<'this>> {
        self.map_range(..)
    }

    /// Maps a range into the CPU's address space.
    ///
    /// Automatically invalidates the GPU caches upon mapping if necessary in order to make
    /// all GPU-side writes available to the CPU. This can be disabled using
    /// [`Self::map_no_invalidate`].
    pub fn map_range<'this, R>(self: &'this Unique<Allocation>, range: R) -> Option<Mapped<'this>>
    where
        R: RangeBounds<usize>,
    {
        let min = match range.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n + 1,
            Bound::Unbounded => 0,
        };

        let max = match range.end_bound() {
            Bound::Included(n) => *n + 1,
            Bound::Excluded(n) => *n,
            Bound::Unbounded => self.metadata().1.size as usize,
        };

        assert!(min <= self.metadata().1.size as usize);
        assert!(max <= self.metadata().1.size as usize);
        assert!(min <= max);

        let mut pointer = std::ptr::null_mut();

        let host_coherent = self.memory_properties().contains(MemoryPropertyFlags::HOST_COHERENT);
        let len = max - min;

        if !host_coherent {
            let ret = unsafe { vmaInvalidateAllocation(self.parent().as_raw(), self.as_raw(), min as _, len as _) };

            if ret != VulkanResultCodes::SUCCESS {
                return None;
            }
        }

        let ret = unsafe { vmaMapMemory(self.parent().as_raw(), self.as_raw(), &mut pointer) };

        match ret {
            VulkanResultCodes::SUCCESS => Some(Mapped {
                allocation: self,
                pointer,
                len,
                host_coherent,
            }),
            _ => None,
        }
    }

    /// Binds buffer to the allocation.
    ///
    /// Binds specified buffer to region of memory represented by specified allocation.
    /// Gets [`DeviceMemory`] handle and offset from the allocation.
    /// If you want to create a buffer, allocate memory for it and bind them together separately,
    /// you should use this function for binding instead of standard `vkBindBufferMemory()`,
    /// because it ensures proper synchronization so that when a `VkDeviceMemory` object is used by
    /// multiple
    /// allocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple
    /// threads simultaneously
    /// (which is illegal in Vulkan).
    ///
    /// It is recommended to use function [`Allocator::create_buffer`] or [`Pool::create_buffer`]
    /// instead of this one.
    ///
    /// [`DeviceMemory`]: magritte::vulkan1_0::DeviceMemory
    pub fn bind_buffer_memory(self: &Unique<Allocation>, buffer: &Unique<Buffer>) -> Result<(), VulkanResultCodes> {
        let res = unsafe { vmaBindBufferMemory(self.parent().as_raw(), self.as_raw(), buffer.as_raw()) };

        if res == VulkanResultCodes::SUCCESS {
            Ok(())
        } else {
            Err(res)
        }
    }

    /// Binds buffer to the allocation with additional parameters.
    ///
    /// This function is similar to [`Self::bind_buffer_memory`], but it provides additional
    /// parameters.
    ///
    /// If `next` is not null, it must be a valid pointer chain for [`BindBufferMemoryInfo`].
    ///
    /// # Safety
    /// Cannot verify the safety of the pointer chain given by `next`.
    ///
    /// [`BindBufferMemoryInfo`]: magritte::vulkan1_1::BindBufferMemoryInfo
    pub unsafe fn bind_buffer_memory2(
        self: &Unique<Allocation>,
        offset: DeviceSize,
        buffer: &Unique<Buffer>,
        next: *const c_void,
    ) -> Result<(), VulkanResultCodes> {
        let res = vmaBindBufferMemory2(self.parent().as_raw(), self.as_raw(), offset, buffer.as_raw(), next);

        if res == VulkanResultCodes::SUCCESS {
            Ok(())
        } else {
            Err(res)
        }
    }

    /// Binds image to the allocation.
    ///
    /// Binds specified image to region of memory represented by specified allocation.
    /// Gets [`DeviceMemory`] handle and offset from the allocation.
    /// If you want to create a image, allocate memory for it and bind them together separately,
    /// you should use this function for binding instead of standard `vkBindBufferMemory()`,
    /// because it ensures proper synchronization so that when a `VkDeviceMemory` object is used by
    /// multiple
    /// allocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple
    /// threads simultaneously
    /// (which is illegal in Vulkan).
    ///
    /// It is recommended to use function [`Allocator::create_image`] or [`Pool::create_image`]
    /// instead of this one.
    ///
    /// [`DeviceMemory`]: magritte::vulkan1_0::DeviceMemory
    pub fn bind_image_memory(self: &Unique<Allocation>, image: &Unique<Image>) -> Result<(), VulkanResultCodes> {
        let res = unsafe { vmaBindImageMemory(self.parent().as_raw(), self.as_raw(), image.as_raw()) };

        if res == VulkanResultCodes::SUCCESS {
            Ok(())
        } else {
            Err(res)
        }
    }

    /// Binds image to the allocation with additional parameters.
    ///
    /// This function is similar to [`Self::bind_image_memory`], but it provides additional
    /// parameters.
    ///
    /// If `next` is not null, it must be a valid pointer chain for [`BindImageMemoryInfo`].
    ///
    /// # Safety
    /// Cannot verify the safety of the pointer chain given by `next`.
    ///
    /// [`BindImageMemoryInfo`]: magritte::vulkan1_1::BindImageMemoryInfo
    pub unsafe fn bind_image_memory2(
        self: &Unique<Allocation>,
        offset: DeviceSize,
        image: &Unique<Image>,
        next: *const c_void,
    ) -> Result<(), VulkanResultCodes> {
        let res = vmaBindImageMemory2(self.parent().as_raw(), self.as_raw(), offset, image.as_raw(), next);

        if res == VulkanResultCodes::SUCCESS {
            Ok(())
        } else {
            Err(res)
        }
    }

    /// Creates a new `Buffer`, binds already created memory for it.
    ///
    /// This function automatically:
    ///
    /// - Creates buffer.
    /// - Binds the buffer with the supplied memory.
    pub fn create_aliasing_buffer<'lt>(
        self: &Unique<Allocation>,
        buffer_create_info: &BufferCreateInfo<'lt>,
    ) -> Result<Unique<Buffer>, VulkanResultCodes> {
        let mut out = Buffer::null();

        let res =
            unsafe { vmaCreateAliasingBuffer(self.parent().as_raw(), self.as_raw(), buffer_create_info, &mut out) };

        match res {
            VulkanResultCodes::SUCCESS => {
                Ok(unsafe { Unique::new(self.parent().parent(), out, AtomicBool::default()) })
            },
            other => Err(other),
        }
    }

    /// Function similar to [`Self::create_aliasing_buffer`].
    pub fn create_aliasing_image<'lt>(
        self: &Unique<Allocation>,
        image_create_info: &ImageCreateInfo<'lt>,
    ) -> Result<Unique<Image>, VulkanResultCodes> {
        let mut out = Image::null();

        let res = unsafe { vmaCreateAliasingImage(self.parent().as_raw(), self.as_raw(), image_create_info, &mut out) };

        match res {
            VulkanResultCodes::SUCCESS => {
                Ok(unsafe { Unique::new(self.parent().parent(), out, AtomicBool::default()) })
            },
            other => Err(other),
        }
    }
}

unsafe impl Send for Allocation {}
impl Default for Allocation {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Allocation {
    type Parent = Unique<Allocator>;
    type VTable = ();
    type Metadata = (Option<Unique<Pool>>, AllocationInfo, AtomicBool);
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
        if !self.metadata().2.load(Ordering::Acquire) {
            vmaFreeMemory(self.parent().as_raw(), self.as_raw())
        }
    }

    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {
        ()
    }
}

pub struct Mapped<'a> {
    allocation: &'a Unique<Allocation>,
    pointer: *mut c_void,
    len: usize,
    host_coherent: bool,
}

impl<'a> Mapped<'a> {
    /// Flushes the entire mapped range
    ///
    /// This makes any writes to `self` visible to the GPU, this is only useful if the
    /// memory is not [`MemoryPropertyFlags::HOST_COHERENT`].
    pub fn flush(&mut self) -> Result<(), VulkanResultCodes> {
        self.flush_range(..)
    }

    /// Flushes the given range.
    ///
    /// This makes any writes to `self` visible to the GPU, this is only useful if the
    /// memory is not [`MemoryPropertyFlags::HOST_COHERENT`].
    pub fn flush_range<R>(&mut self, range: R) -> Result<(), VulkanResultCodes>
    where
        R: RangeBounds<usize>,
    {
        let min = match range.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n + 1,
            Bound::Unbounded => 0,
        };

        let max = match range.end_bound() {
            Bound::Included(n) => *n + 1,
            Bound::Excluded(n) => *n,
            Bound::Unbounded => self.len,
        };

        assert!(min <= self.len);
        assert!(max <= self.len);
        assert!(min <= max);

        let ret = unsafe {
            vmaFlushAllocation(
                self.allocation.parent().as_raw(),
                self.allocation.as_raw(),
                min as _,
                (max - min) as _,
            )
        };

        match ret {
            VulkanResultCodes::SUCCESS => Ok(()),
            other => Err(other),
        }
    }

    /// Invalidate the entire mapped range
    ///
    /// This makes any writes on the GPU visible to `self`, this is only useful if the
    /// memory is not [`MemoryPropertyFlags::HOST_COHERENT`].
    pub fn invalidate(&mut self) -> Result<(), VulkanResultCodes> {
        self.invalidate_range(..)
    }

    /// Invalidate the give range
    ///
    /// This makes any writes on the GPU visible to `self`, this is only useful if the
    /// memory is not [`MemoryPropertyFlags::HOST_COHERENT`].
    pub fn invalidate_range<R>(&mut self, range: R) -> Result<(), VulkanResultCodes>
    where
        R: RangeBounds<usize>,
    {
        let min = match range.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n + 1,
            Bound::Unbounded => 0,
        };

        let max = match range.end_bound() {
            Bound::Included(n) => *n + 1,
            Bound::Excluded(n) => *n,
            Bound::Unbounded => self.len,
        };

        assert!(min <= self.len);
        assert!(max <= self.len);
        assert!(min <= max);

        let ret = unsafe {
            vmaInvalidateAllocation(
                self.allocation.parent().as_raw(),
                self.allocation.as_raw(),
                min as _,
                (max - min) as _,
            )
        };

        match ret {
            VulkanResultCodes::SUCCESS => Ok(()),
            other => Err(other),
        }
    }
}

impl<'a> Deref for Mapped<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.pointer.cast(), self.len) }
    }
}

impl<'a> DerefMut for Mapped<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.pointer.cast(), self.len) }
    }
}

impl<'a> Drop for Mapped<'a> {
    fn drop(&mut self) {
        if self.host_coherent {
            let ret = unsafe {
                vmaFlushAllocation(
                    self.allocation.parent().as_raw(),
                    self.allocation.as_raw(),
                    0,
                    self.len as _,
                )
            };

            if ret != VulkanResultCodes::SUCCESS {
                let name_ptr = self.allocation.info().name;
                let name = if name_ptr.is_null() {
                    "no name"
                } else {
                    unsafe { CStr::from_ptr(name_ptr).to_str().expect("invalid UTF-8 string") }
                };

                error!("failed to flush allocation: {}, reason: {:?}", name, ret);
            }
        }

        unsafe {
            vmaUnmapMemory(self.allocation.parent().as_raw(), self.allocation.as_raw());
        }
    }
}
