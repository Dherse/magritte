bitflags::bitflags! {
    #[repr(transparent)]
    pub struct AllocatorCreateFlags: u32 {
        const EXTERNALLY_SYNCHRONIZED = 0x00000001;
        const DEDICATED_ALLOCATION = 0x00000002;
        const BIND_MEMORY2 = 0x00000004;
        const EXT_MEMORY_BUDGET = 0x00000008;
        const AMD_DEVICE_COHERENT_MEMORY = 0x00000010;
        const BUFFER_DEVICE_ADDRESS = 0x00000020;
        const EXT_MEMORY_PRIORITY = 0x00000040;
    }
}

bitflags::bitflags! {
    /// Flags to be passed as [`AllocationCreateInfo::flags`].
    ///
    /// [`AllocationCreateInfo::flags`]: crate::AllocationCreateInfo::flags
    #[repr(transparent)]
    pub struct AllocationCreateFlags: u32 {
        /// Set this flag if the allocation should have its own memory block.
        ///
        /// Use it for special, big resources, like fullscreen images used as attachments.
        const DEDICATED_MEMORY = 0x00000001;

        /// Set this flag to only try to allocate from existing [`DeviceMemory`] blocks and never create new such block.
        ///
        /// If new allocation cannot be placed in any of the existing blocks, allocation
        /// fails with [`VK_ERROR_OUT_OF_DEVICE_MEMORY`] error.
        /// You should not use [`Self::DEDICATED_MEMORY`] and [`Self::NEVER_ALLOCATE`] at the same time. It makes no sense.
        ///
        /// [`DeviceMemory`]: magritte::vulkan1_0::DeviceMemory
        /// [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]: magritte::vulkan1_0::VulkanResultCodes::ERROR_OUT_OF_DEVICE_MEMORY
        const NEVER_ALLOCATE = 0x00000002;

        /// Set this flag to use a memory that will be persistently mapped and retrieve pointer to it.
        ///
        /// It is valid to use this flag for allocation made from memory type that is not
        /// `HOST_VISIBLE`. This flag is then ignored and memory is not mapped. This is
        /// useful if you need an allocation that is efficient to use on GPU
        /// (`DEVICE_LOCAL`) and still want to map it directly if possible on platforms that
        /// support it (e.g. Intel GPU).
        const MAPPED = 0x00000004;

        #[doc(hidden)]
        #[deprecated = "Preserved for backward compatibility withing VMA"]
        const USER_DATA_COPY_STRING = 0x00000020;

        /// Allocation will be created from upper stack in a double stack pool.
        ///
        /// This flag is only allowed for custom pools created with [`PoolCreateFlags::LINEAR_ALGORITHM`] flag.
        const UPPER_ADDRESS = 0x00000040;

        /// Create both buffer/image and allocation, but don't bind them together.
        /// It is useful when you want to bind yourself to do some more advanced binding, e.g. using some extensions.
        /// The flag is meaningful only with functions that bind by default: vmaCreateBuffer(), vmaCreateImage().
        /// Otherwise it is ignored.
        ///
        ///
        /// If you want to make sure the new buffer/image is not tied to the new memory allocation
        /// through [`MemoryDedicatedAllocateInfo`] structure in case the allocation ends up in its own memory block,
        /// use also flag [`Self::CAN_ALIAS`].
        ///
        /// [`MemoryDedicatedAllocateInfo`]: magritte::vulkan1_1::MemoryDedicatedAllocateInfo
        const DONT_BIND = 0x00000080;

        /// Create allocation only if additional device memory required for it, if any, won't exceed
        /// memory budget. Otherwise return [`VK_ERROR_OUT_OF_DEVICE_MEMORY`].
        ///
        /// [`VK_ERROR_OUT_OF_DEVICE_MEMORY`]: magritte::vulkan1_0::VulkanResultCodes::ERROR_OUT_OF_DEVICE_MEMORY
        const WITHIN_BUDGET = 0x00000100;

        /// Set this flag if the allocated memory will have aliasing resources.
        ///
        /// Usage of this flag prevents supplying [`MemoryDedicatedAllocateInfo`] when [`Self::DEDICATED_MEMORY`] is specified.
        /// Otherwise created dedicated memory will not be suitable for aliasing resources, resulting in Vulkan Validation Layer errors.
        ///
        /// [`MemoryDedicatedAllocateInfo`]: magritte::vulkan1_1::MemoryDedicatedAllocateInfo
        const CAN_ALIAS = 0x00000200;

        /// Requests possibility to map the allocation (using vmaMapMemory() or [`Self::MAPPED`]).

        /// - If you use [`MemoryUsage::AUTO`] or other `USAGE_AUTO*` value,
        ///   you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.
        /// - If you use other value of [`MemoryUsage`], this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.
        ///
        /// This includes allocations created in [`custom memory pools`].
        /// Declares that mapped memory will only be written sequentially, e.g. using `memcpy()` or a loop writing number-by-number,
        /// never read or accessed randomly, so a memory type can be selected that is uncached and write-combined.
        ///
        /// ⚠ Violating this declaration may work correctly, but will likely be very slow.
        /// Watch out for implicit reads introduced by doing e.g. `pMappedData[i] += x;`
        /// Better prepare your data in a local variable and `memcpy()` it to the mapped pointer all at once.
        ///
        /// [`custom memory pools`]: crate::Pool
        /// [`MemoryUsage`]: crate::MemoryUsage
        /// [`MemoryUsage::AUTO`]: crate::MemoryUsage::AUTO
        const HOST_ACCESS_SEQUENTIAL_WRITE = 0x00000400;

        /// Requests possibility to map the allocation (using vmaMapMemory() or [`Self::MAPPED`]).
        ///
        /// - If you use [`MemoryUsage::AUTO`] or other `USAGE_AUTO*` value,
        /// you must use this flag to be able to map the allocation. Otherwise, mapping is incorrect.
        /// - If you use other value of [`MemoryUsage`], this flag is ignored and mapping is always possible in memory types that are `HOST_VISIBLE`.
        ///   This includes allocations created in [`custom memory pools`].
        ///
        /// Declares that mapped memory can be read, written, and accessed in random order,
        /// so a `HOST_CACHED` memory type is required.
        ///
        /// [`custom memory pools`]: crate::Pool
        /// [`MemoryUsage`]: crate::MemoryUsage
        /// [`MemoryUsage::AUTO`]: crate::MemoryUsage::AUTO
        const HOST_ACCESS_RANDOM = 0x00000800;

        /// Together with [`Self::HOST_ACCESS_SEQUENTIAL_WRITE`] or [`Self::HOST_ACCESS_RANDOM`],
        /// it says that despite request for host access, a not-`HOST_VISIBLE` memory type can be selected
        /// if it may improve performance.
        ///
        /// By using this flag, you declare that you will check if the allocation ended up in a `HOST_VISIBLE` memory type
        /// (e.g. using [`Allocation::memory_properties`]) and if not, you will create some "staging" buffer and
        /// issue an explicit transfer to write/read your data.
        ///
        /// To prepare for this possibility, don't forget to add appropriate flags like
        /// [`BufferUsageFlags::TRANSFER_DST`], [`BufferUsageFlags::TRANSFER_SRC`] to the parameters of created buffer or image.
        ///
        /// [`Allocation::memory_properties`]: crate::Allocation::memory_properties
        /// [`BufferUsageFlags::TRANSFER_DST`]: magritte::vulkan1_0::BufferUsageFlags::TRANSFER_DST
        /// [`BufferUsageFlags::TRANSFER_SRC`]: magritte::vulkan1_0::BufferUsageFlags::TRANSFER_SRC
        const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD = 0x00001000;

        /// Allocation strategy that chooses smallest possible free range for the allocation
        /// to minimize memory usage and fragmentation, possibly at the expense of allocation time.
        const STRATEGY_MIN_MEMORY = 0x00010000;

        /// Allocation strategy that chooses first suitable free range for the allocation -
        /// not necessarily in terms of the smallest offset but the one that is easiest and fastest to find
        /// to minimize allocation time, possibly at the expense of allocation quality.
        const STRATEGY_MIN_TIME = 0x00010000;

        /// Allocation strategy that chooses always the lowest offset in available space.
        /// This is not the most efficient strategy but achieves highly packed data.
        /// Used internally by defragmentation, not recomended in typical usage.
        const STRATEGY_MIN_OFFSET = 0x00010000;

        /// Alias to [`Self::STRATEGY_MIN_MEMORY`]
        const STRATEGY_BEST_FIT = Self::STRATEGY_MIN_MEMORY.bits();

        /// Alias to [`Self::STRATEGY_MIN_TIME`]
        const STRATEGY_FIRST_FIT = Self::STRATEGY_MIN_TIME.bits();
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct PoolCreateFlags: u32 {
        const IGNORE_BUFFER_IMAGE_GRANULARITY = 0x00000001;
        const LINEAR_ALGORITHM = 0x00000002;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct DefragmentationFlags: u32 {
        /// Use simple but fast algorithm for defragmentation.
        ///
        /// May not achieve best results but will require least time to compute and least allocations to copy.
        const ALGORITHM_FAST = 0x00000001;

        /// Default defragmentation algorithm, applied also when no `ALGORITHM` flag is specified.
        ///
        /// Offers a balance between defragmentation quality and the amount of allocations and bytes that need to be moved.
        const ALGORITHM_BALANCED = 0x00000002;

        /// Perform full defragmentation of memory.
        ///
        /// Can result in notably more time to compute and allocations to copy, but will achieve best memory packing.
        const ALGORITHM_FULL = 0x00000004;

        /// Use the most roboust algorithm at the cost of time to compute and number of copies to make.
        /// Only available when bufferImageGranularity is greater than 1, since it aims to reduce
        /// alignment issues between different types of resources.
        /// Otherwise falls back to same behavior as [`Self::ALGORITHM_FULL`].
        const ALGORITHM_EXTENSIVE = 0x00000020;
    }
}
