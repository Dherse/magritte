use std::{
    ffi::{c_void, CStr},
    sync::atomic::AtomicBool,
};

use magritte::{
    core::MAX_MEMORY_HEAPS,
    vulkan1_0::{
        Buffer, BufferCreateInfo, Device, DeviceSize, Image, ImageCreateInfo, MemoryPropertyFlags, MemoryRequirements,
        PhysicalDevice, PhysicalDeviceMemoryProperties, PhysicalDeviceProperties, VulkanResultCodes,
    },
    AsRaw, DeviceExtensions, Handle, Unique,
};
use smallvec::SmallVec;

use crate::{
    buffer::{BufferUsage, VmaBuffer},
    defragmentation_context::{DefragmentationContext, DefragmentationContextHandle},
    ffi::{
        vmaAllocateMemory, vmaAllocateMemoryForBuffer, vmaAllocateMemoryForImage, vmaAllocateMemoryPages,
        vmaBeginDefragmentation, vmaBuildStatsString, vmaCalculateStatistics, vmaCheckCorruption, vmaCreatePool,
        vmaDestroyAllocator, vmaFindMemoryTypeIndex, vmaFindMemoryTypeIndexForBufferInfo,
        vmaFindMemoryTypeIndexForImageInfo, vmaFreeStatsString, vmaGetAllocatorInfo, vmaGetHeapBudgets,
        vmaGetMemoryProperties, vmaGetMemoryTypeProperties, vmaGetPhysicalDeviceProperties, vmaSetCurrentFrameIndex,
        AllocationCreateInfo, AllocationInfo, AllocatorCreateInfo, AllocatorInfo, Budget, DefragmentationInfo,
        TotalStatistics, VulkanFunctions,
    },
    flags::AllocatorCreateFlags,
    image::{ImageUsage, VmaImage},
    pool::Pool,
    Allocation, AllocationCreateFlags, AsCStr, DefragmentationFlags, MemoryCorruptionState, PoolCreateInfo,
};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Allocator(pub *mut ());

impl Allocator {
    /// Creates a new allocator.
    ///
    /// The parameters passed to VMA are mostly auto-generated. Only `heap_block_size` and
    /// `heap_size_limits` can still be manually specified as the other ones are deduced from
    /// the state of the [device] and [instance].
    ///
    /// # Heap block size
    /// Preferred size of a single [device memory] block to be allocated from large heaps > 1 GiB.
    ///
    /// # Heap size limits
    /// A slice of limits on maximum number of bytes that can be allocated out of particular Vulkan
    /// memory heap. The number of heaps must be strictly equal to the number of heaps returned
    /// by [`get_physical_device_memory_properties`].
    ///
    /// [device]: magritte::vulkan1_0::Device
    /// [instance]: magritte::vulkan1_0::Instance
    /// [device memory]: magritte::vulkan1_0::DeviceMemory
    /// [`get_physical_device_memory_properties`]: magritte::vulkan1_0::PhysicalDevice::get_physical_device_memory_properties
    pub fn new(
        device: &Unique<Device>,
        heap_block_size: Option<u64>,
        heap_size_limits: Option<&[u64]>,
    ) -> Result<Unique<Self>, VulkanResultCodes> {
        let memory_properties = unsafe { device.physical_device().get_physical_device_memory_properties() };

        if let Some(heap_size_limits) = heap_size_limits {
            assert_eq!(
                memory_properties.memory_heap_count,
                heap_size_limits.len() as u32,
                "The number of heap size limits and heap size must match"
            );
        }

        let functions = VulkanFunctions::new(device.instance().vtable(), device.vtable());

        let extensions = device.metadata();
        let mut flags = AllocatorCreateFlags::empty();
        if extensions.khr_dedicated_allocation() {
            flags |= AllocatorCreateFlags::DEDICATED_ALLOCATION;
        }

        if extensions.khr_bind_memory2() {
            flags |= AllocatorCreateFlags::BIND_MEMORY2;
        }

        if extensions.ext_memory_budget() {
            flags |= AllocatorCreateFlags::EXT_MEMORY_BUDGET;
        }

        if extensions.amd_device_coherent_memory() {
            flags |= AllocatorCreateFlags::AMD_DEVICE_COHERENT_MEMORY;
        }

        if extensions.khr_buffer_device_address() {
            flags |= AllocatorCreateFlags::BUFFER_DEVICE_ADDRESS;
        }

        if extensions.ext_memory_priority() {
            flags |= AllocatorCreateFlags::EXT_MEMORY_PRIORITY;
        }

        let create_info = AllocatorCreateInfo {
            flags,
            physicalDevice: device.physical_device().as_raw(),
            device: device.as_raw(),
            preferredLargeHeapBlockSize: heap_block_size.unwrap_or_default(),
            pAllocationCallbacks: std::ptr::null(),
            pDeviceMemoryCallbacks: std::ptr::null(),
            pHeapSizeLimit: heap_size_limits.map(|s| s.as_ptr()).unwrap_or_else(std::ptr::null),
            pVulkanFunctions: &functions,
            instance: device.instance().as_raw(),
            vulkanApiVersion: extensions.version().into(),
            pTypeExternalMemoryHandleTypes: std::ptr::null(),
        };

        let mut allocator = Self::null();

        let code = unsafe { crate::ffi::vmaCreateAllocator(&create_info, &mut allocator) };

        if code != VulkanResultCodes::SUCCESS {
            return Err(code);
        }

        Ok(unsafe { Unique::new(device, allocator, ()) })
    }

    /// Returns information about existing [`Allocator`] - handle to Vulkan device
    /// etc.
    pub fn allocator_info(self: &Unique<Allocator>) -> AllocatorInfo {
        let mut out = unsafe { std::mem::zeroed::<AllocatorInfo>() };

        unsafe {
            vmaGetAllocatorInfo(self.as_raw(), &mut out);
        }

        out
    }

    /// [`PhysicalDeviceProperties`] are fetched from physicalDevice by the allocator.
    /// You can access it here, without fetching it again on your own.
    pub fn physical_device_properties<'this>(self: &'this Unique<Allocator>) -> &'this PhysicalDeviceProperties {
        let mut out = std::ptr::null();

        unsafe {
            vmaGetPhysicalDeviceProperties(self.as_raw(), &mut out);

            &*out
        }
    }

    /// [`PhysicalDeviceMemoryProperties`] are fetched from physicalDevice by the allocator.
    /// You can access it here, without fetching it again on your own.
    pub fn physical_device_meory_properties<'this>(
        self: &'this Unique<Allocator>,
    ) -> &'this PhysicalDeviceMemoryProperties {
        let mut out = std::ptr::null();

        unsafe {
            vmaGetMemoryProperties(self.as_raw(), &mut out);

            &*out
        }
    }

    /// Given Memory Type Index, returns Property Flags of this memory type.
    pub fn memory_type_properties(self: &Unique<Allocator>, index: u32) -> MemoryPropertyFlags {
        let mut out = MemoryPropertyFlags::empty();

        unsafe {
            vmaGetMemoryTypeProperties(self.as_raw(), index, &mut out);
        }

        out
    }

    /// Sets index of the current frame.
    /// This function must be used if you make allocations with
    /// [`AllocationCreateFlags::CAN_BECOME_LOST`] and [`AllocationCreateFlags::
    /// CAN_MAKE_OTHER_LOST`] flags to inform the allocator when a new frame begins. Allocations
    /// queried using [`Allocator::allocation_info`] cannot become lost in the current frame.
    pub fn set_current_frame_index(self: &Unique<Allocator>, frame_index: u32) {
        unsafe {
            vmaSetCurrentFrameIndex(self.as_raw(), frame_index);
        }
    }

    /// Retrieves statistics from current state of the Allocator.
    ///
    /// This function is called "calculate" not "get" because it has to traverse all
    /// internal data structures, so it may be quite slow. Use it for debugging purposes.
    /// For faster but more brief statistics suitable to be called every frame or every allocation,
    /// use [`Self::get_heap_budgets`].
    ///
    /// Note that when using allocator from multiple threads, returned information may immediately
    /// become outdated.
    pub fn calculate_statistics(self: &Unique<Allocator>) -> TotalStatistics {
        let mut out = unsafe { std::mem::zeroed() };

        unsafe {
            vmaCalculateStatistics(self.as_raw(), &mut out);
        }

        out
    }

    /// Retrieves information about current memory usage and budget for all memory heaps.
    ///
    ///
    /// This function is called "get" not "calculate" because it is very fast, suitable to be called
    /// every frame or every allocation. For more detailed statistics use
    /// [`Self::calculate_statistics`].
    ///
    /// Note that when using allocator from multiple threads, returned information may immediately
    /// become outdated.
    pub fn heap_budgets(self: &Unique<Allocator>) -> SmallVec<[Budget; MAX_MEMORY_HEAPS as usize]> {
        let mut out = SmallVec::from_elem(
            unsafe { std::mem::zeroed() },
            self.physical_device_meory_properties().memory_heap_count as usize,
        );

        unsafe {
            vmaGetHeapBudgets(self.as_raw(), out.as_mut_ptr());
        }

        out
    }

    /// Helps to find memory type index, given `memory_type_bits` and [`AllocationCreateInfo`].
    ///
    /// This algorithm tries to find a memory type that:
    ///
    /// - Is allowed by `memory_type_bits`.
    /// - Contains all the flags from [`AllocationCreateInfo::required_flags`].
    /// - Matches intended usage.
    /// - Has as many flags from [`AllocationCreateInfo::preferred_flags`] as possible.
    pub fn find_memory_type_index(
        self: &Unique<Allocator>,
        memory_type_bits: u32,
        allocation_create_info: &AllocationCreateInfo,
    ) -> Option<u32> {
        let mut out = 0;

        let res = unsafe { vmaFindMemoryTypeIndex(self.as_raw(), memory_type_bits, allocation_create_info, &mut out) };

        match res {
            VulkanResultCodes::SUCCESS => Some(out),
            VulkanResultCodes::ERROR_FEATURE_NOT_PRESENT => None,
            other => panic!("unexpected result: {}", other),
        }
    }

    /// Helps to find `memory_type_index`, given [`BufferCreateInfo`] and [`AllocationCreateInfo`].
    ///
    /// It can be useful e.g. to determine value to be used as
    /// [`PoolCreateInfo::memory_type_index`]. It internally creates a temporary, dummy buffer
    /// that never has memory bound.
    pub fn find_memory_type_index_for_buffer_info<'lt>(
        self: &Unique<Allocator>,
        buffer_create_info: &BufferCreateInfo<'lt>,
        allocation_create_info: &AllocationCreateInfo,
    ) -> Option<u32> {
        let mut out = 0;

        let res = unsafe {
            vmaFindMemoryTypeIndexForBufferInfo(self.as_raw(), buffer_create_info, allocation_create_info, &mut out)
        };

        match res {
            VulkanResultCodes::SUCCESS => Some(out),
            VulkanResultCodes::ERROR_FEATURE_NOT_PRESENT => None,
            other => panic!("unexpected result: {}", other),
        }
    }

    /// Helps to find `memory_type_index`, given [`ImageCreateInfo`] and [`AllocationCreateInfo`].
    ///
    /// It can be useful e.g. to determine value to be used as
    /// [`PoolCreateInfo::memory_type_index`]. It internally creates a temporary, dummy buffer
    /// that never has memory bound.
    pub fn find_memory_type_index_for_image_info<'lt>(
        self: &Unique<Allocator>,
        image_create_info: &ImageCreateInfo<'lt>,
        allocation_create_info: &AllocationCreateInfo,
    ) -> Option<u32> {
        let mut out = 0;

        let res = unsafe {
            vmaFindMemoryTypeIndexForImageInfo(self.as_raw(), image_create_info, allocation_create_info, &mut out)
        };

        match res {
            VulkanResultCodes::SUCCESS => Some(out),
            VulkanResultCodes::ERROR_FEATURE_NOT_PRESENT => None,
            other => panic!("unexpected result: {}", other),
        }
    }

    /// Allocates Vulkan device memory and creates [`Pool`] object.
    pub fn create_pool(
        self: &Unique<Allocator>,
        pool_create_info: &PoolCreateInfo,
    ) -> Result<Unique<Pool>, VulkanResultCodes> {
        let mut out = Pool::null();

        let res = unsafe { vmaCreatePool(self.as_raw(), pool_create_info, &mut out) };

        match res {
            VulkanResultCodes::SUCCESS => Ok(unsafe { Unique::new(self, out, ()) }),
            other => Err(other),
        }
    }

    /// General purpose memory allocation.
    ///
    /// This will always **ignore** the [`AllocationCreateInfo::pool`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory`] function instead.
    pub fn allocate_memory(
        self: &Unique<Allocator>,
        memory_requirements: &MemoryRequirements,
        create_info: &AllocationCreateInfo,
    ) -> Result<(Unique<Allocation>, AllocationInfo), VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = Pool::null();

        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let res = unsafe {
            vmaAllocateMemory(
                self.as_raw(),
                memory_requirements,
                &create_info,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok((
                unsafe { Unique::new(self, allocation, (None, allocation_info, AtomicBool::new(false))) },
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
    /// This will always **ignore** the [`AllocationCreateInfo::pool`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory_pages`] function
    /// instead.
    pub fn allocate_memory_pages(
        self: &Unique<Allocator>,
        memory_requirements: &MemoryRequirements,
        create_info: &AllocationCreateInfo,
        allocation_count: usize,
    ) -> Result<SmallVec<[(Unique<Allocation>, AllocationInfo); 8]>, VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = Pool::null();

        let mut allocations = SmallVec::<[_; 8]>::from_elem(Allocation::null(), allocation_count);
        let mut allocation_infos = SmallVec::<[_; 8]>::from_elem(unsafe { std::mem::zeroed() }, allocation_count);

        let res = unsafe {
            vmaAllocateMemoryPages(
                self.as_raw(),
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
                        unsafe { Unique::new(self, alloc, (None, allocation_info, AtomicBool::new(false))) },
                        allocation_info,
                    )
                })
                .collect()),
            other => Err(other),
        }
    }

    /// Allocates memory suitable for given [`Buffer`].
    ///
    /// It only creates #Allocation. To bind the memory to the buffer, use
    /// [`Self::bind_buffer_memory`].
    ///
    /// This is a special-purpose function. In most cases you should use [`Self::create_buffer`].
    ///
    /// This will always **ignore** the [`AllocationCreateInfo::pool`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory_for_buffer`] function
    /// instead.
    pub fn allocate_memory_for_buffer(
        self: &Unique<Allocator>,
        buffer: &Unique<Buffer>,
        create_info: &AllocationCreateInfo,
    ) -> Result<(Unique<Allocation>, AllocationInfo), VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = Pool::null();

        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let res = unsafe {
            vmaAllocateMemoryForBuffer(
                self.as_raw(),
                buffer.as_raw(),
                &create_info,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok((
                unsafe { Unique::new(self, allocation, (None, allocation_info, AtomicBool::new(false))) },
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
    /// This will always **ignore** the [`AllocationCreateInfo::pool`].
    /// If you want to allocate from a pool, use the [`Pool::allocate_memory_for_image`] function
    /// instead.
    pub fn allocate_memory_for_image(
        self: &Unique<Allocator>,
        image: &Unique<Image>,
        create_info: &AllocationCreateInfo,
    ) -> Result<(Unique<Allocation>, AllocationInfo), VulkanResultCodes> {
        let mut create_info = *create_info;
        create_info.pool = Pool::null();

        let mut allocation = Allocation::null();
        let mut allocation_info = unsafe { std::mem::zeroed() };

        let res = unsafe {
            vmaAllocateMemoryForImage(
                self.as_raw(),
                image.as_raw(),
                &create_info,
                &mut allocation,
                &mut allocation_info,
            )
        };

        match res {
            VulkanResultCodes::SUCCESS => Ok((
                unsafe { Unique::new(self, allocation, (None, allocation_info, AtomicBool::new(false))) },
                allocation_info,
            )),
            other => Err(other),
        }
    }

    /// Checks magic number in margins around all allocations in given memory types (in both
    /// default and custom pools) in search for corruptions.
    pub fn check_corruption(self: &Unique<Allocator>, memory_type_bits: u32) -> MemoryCorruptionState {
        let res = unsafe { vmaCheckCorruption(self.as_raw(), memory_type_bits) };

        match res {
            VulkanResultCodes::ERROR_FEATURE_NOT_PRESENT => MemoryCorruptionState::Disabled,
            VulkanResultCodes::SUCCESS => MemoryCorruptionState::NoCorruption,
            VulkanResultCodes::ERROR_UNKNOWN => MemoryCorruptionState::Corrupted,
            other => unreachable!("unexpected vulkan result: {}", other),
        }
    }

    /// Begins defragmentation process.
    /// For more information about defragmentation, see documentation chapter:
    /// [Defragmentation](https://gpuopen-librariesandsdks.github.io/VulkanMemoryAllocator/html/defragmentation.html).
    pub fn begin_defragmentation(
        self: &Unique<Allocator>,
        flags: DefragmentationFlags,
        max_bytes_per_pass: Option<DeviceSize>,
        max_allocations_per_pass: Option<u32>,
    ) -> Result<DefragmentationContext, VulkanResultCodes> {
        let info = DefragmentationInfo {
            flags,
            pool: Pool::null(),
            max_bytes_per_pass: max_bytes_per_pass.unwrap_or_default(),
            max_allocations_per_pass: max_allocations_per_pass.unwrap_or_default(),
        };

        let mut out = DefragmentationContextHandle::null();

        let res = unsafe { vmaBeginDefragmentation(self.as_raw(), &info, &mut out) };

        match res {
            VulkanResultCodes::SUCCESS => Ok(unsafe { DefragmentationContext::new(self.clone(), None, out) }),
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
        self: &Unique<Allocator>,
        buffer_create_info: &BufferCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        usage: BufferUsage,
        memory_type_bits: Option<u32>,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<VmaBuffer, VulkanResultCodes> {
        VmaBuffer::new(
            self,
            buffer_create_info,
            flags,
            usage,
            memory_type_bits,
            priority,
            user_data,
            None,
        )
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
        self: &Unique<Allocator>,
        buffer_create_info: &BufferCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        usage: BufferUsage,
        memory_type_bits: Option<u32>,
        alignment: DeviceSize,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<VmaBuffer, VulkanResultCodes> {
        VmaBuffer::new(
            self,
            buffer_create_info,
            flags,
            usage,
            memory_type_bits,
            priority,
            user_data,
            Some(alignment),
        )
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
        self: &Unique<Allocator>,
        image_create_info: &ImageCreateInfo<'lt>,
        flags: AllocationCreateFlags,
        usage: ImageUsage,
        memory_type_bits: Option<u32>,
        priority: Option<f32>,
        user_data: Option<*mut c_void>,
    ) -> Result<VmaImage, VulkanResultCodes> {
        VmaImage::new(
            self,
            image_create_info,
            flags,
            usage,
            memory_type_bits,
            priority,
            user_data,
        )
    }

    /// Builds and returns statistics as a string in JSON format.
    pub fn stats_string(self: &Unique<Allocator>, detailed: bool) -> String {
        let mut stats = std::ptr::null_mut();
        unsafe {
            vmaBuildStatsString(self.as_raw(), &mut stats, detailed as u8 as u32);

            let out = CStr::from_ptr(stats).to_string_lossy().to_string();

            vmaFreeStatsString(self.as_raw(), stats);

            out
        }
    }

    /// Enables the maximum number device extensions supported by both VMA and the physical
    /// device. This means that as many VMA features as possible will be available.
    pub fn enable_extensions(
        physical_device: &Unique<PhysicalDevice>,
        extensions: &mut DeviceExtensions,
        device_address: bool,
    ) -> Result<(), VulkanResultCodes> {
        const DEDICATED_ALLOCATION: &str = "VK_KHR_dedicated_allocation";
        const BIND_MEMORY2: &str = "VK_KHR_bind_memory2";
        const MEMORY_BUDGET: &str = "VK_EXT_memory_budget";
        const DEVICE_COHERENT_MEMORY: &str = "VK_AMD_device_coherent_memory";
        const BUFFER_DEVICE_ADDRESS: &str = "VK_KHR_buffer_device_address";
        const MEMORY_PRIORITY: &str = "VK_EXT_memory_priority";

        let extension_list = unsafe {
            physical_device
                .enumerate_device_extension_properties(None, None)
                .result()?
        };

        let mut sum = 0;
        for extension in extension_list {
            *extensions = match extension.extension_name().as_cstr().to_str().unwrap() {
                DEDICATED_ALLOCATION => {
                    sum += 1;
                    extensions.enable_khr_dedicated_allocation()
                },
                BIND_MEMORY2 => {
                    sum += 1;
                    extensions.enable_khr_bind_memory2()
                },
                MEMORY_BUDGET => {
                    sum += 1;
                    extensions.enable_ext_memory_budget()
                },
                DEVICE_COHERENT_MEMORY => {
                    sum += 1;
                    extensions.enable_amd_device_coherent_memory()
                },
                BUFFER_DEVICE_ADDRESS if device_address => {
                    sum += 1;
                    extensions.enable_khr_buffer_device_address()
                },
                BUFFER_DEVICE_ADDRESS if !device_address => {
                    sum += 1;
                    continue;
                },
                MEMORY_PRIORITY => {
                    sum += 1;
                    extensions.enable_ext_memory_priority()
                },
                _ => continue,
            };

            if sum == 6 {
                break;
            }
        }

        Ok(())
    }

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
}

unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

impl Default for Allocator {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Allocator {
    type Parent = Unique<Device>;
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
        vmaDestroyAllocator(self.as_raw())
    }

    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
