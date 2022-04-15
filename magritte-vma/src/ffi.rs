#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ::std::os::raw::{c_char, c_uint, c_void};

use magritte::{
    vulkan1_0::{
        Bool32, Buffer, BufferCreateInfo, Device, DeviceMemory, DeviceSize, DeviceVTable, FNAllocateMemory,
        FNBindBufferMemory, FNBindImageMemory, FNCmdCopyBuffer, FNCreateBuffer, FNCreateImage, FNDestroyBuffer,
        FNDestroyImage, FNFlushMappedMemoryRanges, FNFreeMemory, FNGetBufferMemoryRequirements, FNGetDeviceProcAddr,
        FNGetImageMemoryRequirements, FNGetInstanceProcAddr, FNGetPhysicalDeviceMemoryProperties,
        FNGetPhysicalDeviceProperties, FNInvalidateMappedMemoryRanges, FNMapMemory, FNUnmapMemory, Image,
        ImageCreateInfo, Instance, InstanceVTable, MemoryPropertyFlags, MemoryRequirements, PhysicalDevice,
        PhysicalDeviceMemoryProperties, PhysicalDeviceProperties, VulkanResultCodes,
    },
    vulkan1_1::{
        ExternalMemoryHandleTypeFlags as ExternalMemoryHandleTypeFlagsKHR, FNBindBufferMemory2, FNBindImageMemory2,
        FNGetBufferMemoryRequirements2, FNGetImageMemoryRequirements2, FNGetPhysicalDeviceMemoryProperties2,
    },
    vulkan1_3::{FNGetDeviceBufferMemoryRequirements, FNGetDeviceImageMemoryRequirements},
};

use crate::{
    allocation::Allocation,
    allocator::Allocator,
    defragmentation_context::DefragmentationContextHandle,
    flags::{AllocationCreateFlags, AllocatorCreateFlags, DefragmentationFlags, PoolCreateFlags},
    pool::Pool,
};

type Result = VulkanResultCodes;
type AllocationCallbacks = magritte::vulkan1_0::AllocationCallbacks<'static>;

/// All the Vulkan function used by VMA
#[repr(C)]
pub struct VulkanFunctions {
    pub get_instance_proc_addr: FNGetInstanceProcAddr,
    pub get_device_proc_addr: FNGetDeviceProcAddr,
    pub get_physical_device_properties: FNGetPhysicalDeviceProperties,
    pub get_physical_device_memory_properties: FNGetPhysicalDeviceMemoryProperties,
    pub allocate_memory: FNAllocateMemory,
    pub free_memory: FNFreeMemory,
    pub map_memory: FNMapMemory,
    pub unmap_memory: FNUnmapMemory,
    pub flush_mapped_memory_ranges: FNFlushMappedMemoryRanges,
    pub invalidate_mapped_memory_ranges: FNInvalidateMappedMemoryRanges,
    pub bind_buffer_memory: FNBindBufferMemory,
    pub bind_image_memory: FNBindImageMemory,
    pub get_buffer_memory_requirements: FNGetBufferMemoryRequirements,
    pub get_image_memory_requirements: FNGetImageMemoryRequirements,
    pub create_buffer: FNCreateBuffer,
    pub destroy_buffer: FNDestroyBuffer,
    pub create_image: FNCreateImage,
    pub destroy_image: FNDestroyImage,
    pub cmd_copy_buffer: FNCmdCopyBuffer,
    pub get_buffer_memory_requirements2: FNGetBufferMemoryRequirements2,
    pub get_image_memory_requirements2: FNGetImageMemoryRequirements2,
    pub bind_buffer_memory2: FNBindBufferMemory2,
    pub bind_image_memory2: FNBindImageMemory2,
    pub get_physical_device_memory_properties2: FNGetPhysicalDeviceMemoryProperties2,
    pub get_device_buffer_memory_requirements: FNGetDeviceBufferMemoryRequirements,
    pub get_device_image_memory_requirements: FNGetDeviceImageMemoryRequirements,
}

impl VulkanFunctions {
    pub fn new(instance: &InstanceVTable, device: &DeviceVTable) -> Self {
        Self {
            get_instance_proc_addr: None,
            get_device_proc_addr: instance.vulkan1_0().get_device_proc_addr(),
            get_physical_device_properties: instance.vulkan1_0().get_physical_device_properties(),
            get_physical_device_memory_properties: instance.vulkan1_0().get_physical_device_memory_properties(),
            allocate_memory: device.vulkan1_0().allocate_memory(),
            free_memory: device.vulkan1_0().free_memory(),
            map_memory: device.vulkan1_0().map_memory(),
            unmap_memory: device.vulkan1_0().unmap_memory(),
            flush_mapped_memory_ranges: device.vulkan1_0().flush_mapped_memory_ranges(),
            invalidate_mapped_memory_ranges: device.vulkan1_0().invalidate_mapped_memory_ranges(),
            bind_buffer_memory: device.vulkan1_0().bind_buffer_memory(),
            bind_image_memory: device.vulkan1_0().bind_image_memory(),
            get_buffer_memory_requirements: device.vulkan1_0().get_buffer_memory_requirements(),
            get_image_memory_requirements: device.vulkan1_0().get_image_memory_requirements(),
            create_buffer: device.vulkan1_0().create_buffer(),
            destroy_buffer: device.vulkan1_0().destroy_buffer(),
            create_image: device.vulkan1_0().create_image(),
            destroy_image: device.vulkan1_0().destroy_image(),
            cmd_copy_buffer: device.vulkan1_0().cmd_copy_buffer(),
            get_buffer_memory_requirements2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.get_buffer_memory_requirements2())
                .or_else(|| {
                    device
                        .khr_get_memory_requirements2()
                        .and_then(|vtable| vtable.get_buffer_memory_requirements2_khr())
                }),
            get_image_memory_requirements2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.get_image_memory_requirements2())
                .or_else(|| {
                    device
                        .khr_get_memory_requirements2()
                        .and_then(|vtable| vtable.get_image_memory_requirements2_khr())
                }),
            bind_buffer_memory2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.bind_buffer_memory2())
                .or_else(|| {
                    device
                        .khr_bind_memory2()
                        .and_then(|vtable| vtable.bind_buffer_memory2_khr())
                }),
            bind_image_memory2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.bind_image_memory2())
                .or_else(|| {
                    device
                        .khr_bind_memory2()
                        .and_then(|vtable| vtable.bind_image_memory2_khr())
                }),
            get_physical_device_memory_properties2: instance
                .vulkan1_1()
                .and_then(|vtable| vtable.get_physical_device_memory_properties2())
                .or_else(|| {
                    instance
                        .khr_get_physical_device_properties2()
                        .and_then(|vtable| vtable.get_physical_device_memory_properties2_khr())
                }),
            get_device_buffer_memory_requirements: device
                .vulkan1_3()
                .and_then(|vtable| vtable.get_device_buffer_memory_requirements())
                .or_else(|| {
                    device
                        .khr_maintenance4()
                        .and_then(|vtable| vtable.get_device_buffer_memory_requirements_khr())
                }),
            get_device_image_memory_requirements: device
                .vulkan1_3()
                .and_then(|vtable| vtable.get_device_image_memory_requirements())
                .or_else(|| {
                    device
                        .khr_maintenance4()
                        .and_then(|vtable| vtable.get_device_image_memory_requirements_khr())
                }),
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryUsage(c_uint);

/// Intended usage of the allocated memory.
impl MemoryUsage {
    /// No intended memory usage specified.
    /// Use other members of [`AllocationCreateInfo`] to specify your requirements.
    pub const UNKNOWN: Self = Self(0);

    /// Prefers [`MemoryPropertyFlags::DEVICE_LOCAL`].
    ///
    /// [`MemoryPropertyFlags::DEVICE_LOCAL`]: magritte::vulkan1_0::MemoryPropertyFlags::DEVICE_LOCAL
    #[deprecated = "Preserved for backward compatibility inside of VMA"]
    pub const GPU_ONLY: Self = Self(2);

    /// Guarantees [`MemoryPropertyFlags::HOST_VISIBLE`] and [`MemoryPropertyFlags::HOST_COHERENT`].
    ///
    /// [`MemoryPropertyFlags::HOST_VISIBLE`]: magritte::vulkan1_0::MemoryPropertyFlags::HOST_VISIBLE
    /// [`MemoryPropertyFlags::HOST_COHERENT`]: magritte::vulkan1_0::MemoryPropertyFlags::HOST_COHERENT
    #[deprecated = "Preserved for backward compatibility inside of VMA"]
    pub const CPU_ONLY: Self = Self(2);

    /// Guarantees [`MemoryPropertyFlags::HOST_VISIBLE`] and [`MemoryPropertyFlags::DEVICE_LOCAL`].
    ///
    /// [`MemoryPropertyFlags::HOST_VISIBLE`]: magritte::vulkan1_0::MemoryPropertyFlags::HOST_VISIBLE
    /// [`MemoryPropertyFlags::DEVICE_LOCAL`]: magritte::vulkan1_0::MemoryPropertyFlags::DEVICE_LOCAL
    #[deprecated = "Preserved for backward compatibility inside of VMA"]
    pub const CPU_TO_GPU: Self = Self(3);

    /// Guarantees [`MemoryPropertyFlags::HOST_VISIBLE`] and [`MemoryPropertyFlags::HOST_CACHED`].
    ///
    /// [`MemoryPropertyFlags::HOST_VISIBLE`]: magritte::vulkan1_0::MemoryPropertyFlags::HOST_VISIBLE
    /// [`MemoryPropertyFlags::HOST_CACHED`]: magritte::vulkan1_0::MemoryPropertyFlags::HOST_CACHED
    #[deprecated = "Preserved for backward compatibility inside of VMA"]
    pub const GPU_TO_CPU: Self = Self(4);

    /// Prefers not [`MemoryPropertyFlags::DEVICE_LOCAL`].
    ///
    /// [`MemoryPropertyFlags::DEVICE_LOCAL`]: magritte::vulkan1_0::MemoryPropertyFlags::DEVICE_LOCAL
    #[deprecated = "Preserved for backward compatibility inside of VMA"]
    pub const CPU_COPY: Self = Self(5);

    /// Lazily allocated GPU memory having [`MemoryPropertyFlags::LAZILY_ALLOCATED`].
    /// Exists mostly on mobile platforms. Using it on desktop PC or other GPUs with no such memory
    /// type present will fail the allocation.
    ///
    /// Usage: Memory for transient attachment images (color attachments, depth attachments etc.),
    /// created with [`ImageUsageFlags::TRANSIENT_ATTACHMENT`].
    ///
    /// Allocations with this usage are always created as dedicated - it implies
    /// #VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.
    ///
    /// [`MemoryPropertyFlags::LAZILY_ALLOCATED`]: magritte::vulkan1_0::MemoryPropertyFlags::LAZILY_ALLOCATED
    /// [`ImageUsageFlags::TRANSIENT_ATTACHMENT`]: magritte::vulkan1_0::ImageUsageFlags::TRANSIENT_ATTACHMENT
    pub const GPU_LAZILY_ALLOCATED: Self = Self(6);

    /// Selects best memory type automatically.
    /// This flag is recommended for most common use cases.
    ///
    /// When using this flag, if you want to map the allocation (using vmaMapMemory() or
    /// [`AllocationCreateFlags::MAPPED`]),
    /// you must pass one of the flags: [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE`] or
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM`] in [`AllocationCreateInfo::flags`].
    ///
    /// It can be used only with functions that let the library know `VkBufferCreateInfo` or
    /// `VkImageCreateInfo`, e.g. [`Allocator::create_buffer`], [`Allocator::create_image`],
    /// [`Allocator::find_memory_type_index_for_buffer_info`], [`Allocator::
    /// find_memory_type_index_for_image_info`] and not with generic memory allocation
    /// functions.
    ///
    /// [`AllocationCreateFlags::MAPPED`]: crate::AllocationCreateFlags::MAPPED
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE`]: crate::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM`]: crate::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM
    pub const AUTO: Self = Self(7);

    /// Selects best memory type automatically with preference for GPU (device) memory.
    ///
    /// When using this flag, if you want to map the allocation (using vmaMapMemory() or
    /// [`AllocationCreateFlags::MAPPED`]),
    /// you must pass one of the flags: [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE`] or
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM`] in [`AllocationCreateInfo::flags`].
    ///
    /// It can be used only with functions that let the library know `VkBufferCreateInfo` or
    /// `VkImageCreateInfo`, e.g. [`Allocator::create_buffer`], [`Allocator::create_image`],
    /// [`Allocator::find_memory_type_index_for_buffer_info`], [`Allocator::
    /// find_memory_type_index_for_image_info`] and not with generic memory allocation
    /// functions.
    ///
    /// [`AllocationCreateFlags::MAPPED`]: crate::AllocationCreateFlags::MAPPED
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE`]: crate::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM`]: crate::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM
    pub const PREFER_DEVICE: Self = Self(8);

    /// Selects best memory type automatically with preference for CPU (host) memory.
    ///
    /// When using this flag, if you want to map the allocation (using vmaMapMemory() or
    /// [`AllocationCreateFlags::MAPPED`]),
    /// you must pass one of the flags: [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE`] or
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM`] in [`AllocationCreateInfo::flags`].
    ///
    /// It can be used only with functions that let the library know `VkBufferCreateInfo` or
    /// `VkImageCreateInfo`, e.g. [`Allocator::create_buffer`], [`Allocator::create_image`],
    /// [`Allocator::find_memory_type_index_for_buffer_info`], [`Allocator::
    /// find_memory_type_index_for_image_info`] and not with generic memory allocation
    /// functions.
    ///
    /// [`AllocationCreateFlags::MAPPED`]: crate::AllocationCreateFlags::MAPPED
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE`]: crate::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE
    /// [`AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM`]: crate::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_RANDOM
    pub const PREFER_HOST: Self = Self(9);
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefragmentationMoveOperation(c_uint);

/// Operation performed on single defragmentation move. See structure [`DefragmentationMove`].
impl DefragmentationMoveOperation {
    /// Buffer/image has been recreated at `dstTmpAllocation`, data has been copied, old
    /// buffer/image has been destroyed. `srcAllocation` should be changed to point to the new
    /// place. This is the default value set by vmaBeginDefragmentationPass().
    pub const COPY: Self = Self(0);

    /// Set this value if you cannot move the allocation. New place reserved at `dstTmpAllocation`
    /// will be freed. `srcAllocation` will remain unchanged.
    pub const IGNORE: Self = Self(1);

    /// Set this value if you decide to abandon the allocation and you destroyed the buffer/image.
    /// New place reserved at `dstTmpAllocation` will be freed, along with `srcAllocation`,
    /// which will be destroyed.
    pub const DESTROY: Self = Self(2);
}

/// Callback function called after successful vkAllocateMemory.
pub type PFN_vmaAllocateDeviceMemoryFunction = ::std::option::Option<
    unsafe extern "C" fn(
        allocator: Allocator,
        memoryType: u32,
        memory: DeviceMemory,
        size: DeviceSize,
        pUserData: *mut c_void,
    ),
>;
/// Callback function called before vkFreeMemory.
pub type PFN_vmaFreeDeviceMemoryFunction = ::std::option::Option<
    unsafe extern "C" fn(
        allocator: Allocator,
        memoryType: u32,
        memory: DeviceMemory,
        size: DeviceSize,
        pUserData: *mut c_void,
    ),
>;
/// Set of callbacks that the library will call for `vkAllocateMemory` and `vkFreeMemory`.
///
///Provided for informative purpose, e.g. to gather statistics about number of
///allocations or total amount of memory allocated in Vulkan.
///
///Used in AllocatorCreateInfo::pDeviceMemoryCallbacks.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceMemoryCallbacks {
    /// Optional, can be null.
    pub pfnAllocate: PFN_vmaAllocateDeviceMemoryFunction,
    /// Optional, can be null.
    pub pfnFree: PFN_vmaFreeDeviceMemoryFunction,
    /// Optional, can be null.
    pub pUserData: *mut c_void,
}
/// Description of a Allocator to be created.
#[repr(C)]
pub struct AllocatorCreateInfo {
    /// Flags for created allocator. Use #AllocatorCreateFlagBits enum.
    pub flags: AllocatorCreateFlags,
    /// Vulkan physical device.
    ///** It must be valid throughout whole lifetime of created allocator. */
    pub physicalDevice: PhysicalDevice,
    /// Vulkan device.
    ///** It must be valid throughout whole lifetime of created allocator. */
    pub device: Device,
    /// Preferred size of a single `VkDeviceMemory` block to be allocated from large heaps > 1 GiB.
    /// Optional.
    ///** Set to 0 to use default, which is currently 256 MiB. */
    pub preferredLargeHeapBlockSize: DeviceSize,
    /// Custom CPU memory allocation callbacks. Optional.
    ///** Optional, can be null. When specified, will also be used for all CPU-side memory
    /// allocations. */
    pub pAllocationCallbacks: *const AllocationCallbacks,
    /// Informative callbacks for `vkAllocateMemory`, `vkFreeMemory`. Optional.
    ///** Optional, can be null. */
    pub pDeviceMemoryCallbacks: *const DeviceMemoryCallbacks,
    /// Either null or a pointer to an array of limits on maximum number of bytes that can be
    /// allocated out of particular Vulkan memory heap.
    ///
    ///If not NULL, it must be a pointer to an array of
    ///`VkPhysicalDeviceMemoryProperties::memoryHeapCount` elements, defining limit on
    ///maximum number of bytes that can be allocated out of particular Vulkan memory
    ///heap.
    ///
    ///Any of the elements may be equal to `VK_WHOLE_SIZE`, which means no limit on that
    ///heap. This is also the default in case of `pHeapSizeLimit` = NULL.
    ///
    ///If there is a limit defined for a heap:
    ///
    /// - If user tries to allocate more memory from that heap using this allocator,
    ///the allocation fails with `VK_ERROR_OUT_OF_DEVICE_MEMORY`.
    /// - If the limit is smaller than heap size reported in `VkMemoryHeap::size`, the
    ///value of this limit will be reported instead when using vmaGetMemoryProperties().
    ///
    ///Warning! Using this feature may not be equivalent to installing a GPU with
    ///smaller amount of memory, because graphics driver doesn't necessary fail new
    ///allocations with `VK_ERROR_OUT_OF_DEVICE_MEMORY` result when memory capacity is
    ///exceeded. It may return success and just silently migrate some device memory
    ///blocks to system RAM. This driver behavior can also be controlled using
    ///VK_AMD_memory_overallocation_behavior extension.
    pub pHeapSizeLimit: *const DeviceSize,
    /// Pointers to Vulkan functions. Can be null.
    ///
    ///For details see [Pointers to Vulkan functions](@ref config_Vulkan_functions).
    pub pVulkanFunctions: *const VulkanFunctions,
    /// Handle to Vulkan instance object.
    ///
    ///Starting from version 3.0.0 this member is no longer optional, it must be set!
    pub instance: Instance,
    /// Optional. The highest version of Vulkan that the application is designed to use.
    ///
    ///It must be a value in the format as created by macro `VK_MAKE_VERSION` or a constant like:
    /// `VK_API_VERSION_1_1`, `VK_API_VERSION_1_0`.
    ///The patch version number specified is ignored. Only the major and minor versions are
    /// considered.
    ///It must be less or equal (preferably equal) to value as passed to `vkCreateInstance` as
    /// `VkApplicationInfo::apiVersion`.
    ///Only versions 1.0, 1.1, 1.2, 1.3 are supported by the current implementation.
    ///Leaving it initialized to zero is equivalent to `VK_API_VERSION_1_0`.
    pub vulkanApiVersion: u32,
    /// Either null or a pointer to an array of external memory handle types for each Vulkan
    /// memory type.
    ///
    ///If not NULL, it must be a pointer to an array of
    /// `VkPhysicalDeviceMemoryProperties::memoryTypeCount`
    ///elements, defining external memory handle types of particular Vulkan memory type,
    ///to be passed using `VkExportMemoryAllocateInfoKHR`.
    ///
    ///Any of the elements may be equal to 0, which means not to use
    /// `VkExportMemoryAllocateInfoKHR` on this memory type.
    ///This is also the default in case of `pTypeExternalMemoryHandleTypes` = NULL.
    pub pTypeExternalMemoryHandleTypes: *const ExternalMemoryHandleTypeFlagsKHR,
}
/// Information about existing #Allocator object.
#[repr(C)]
pub struct AllocatorInfo {
    /// Handle to Vulkan instance object.
    ///
    ///This is the same value as has been passed through AllocatorCreateInfo::instance.
    pub instance: Instance,
    /// Handle to Vulkan physical device object.
    ///
    ///This is the same value as has been passed through AllocatorCreateInfo::physicalDevice.
    pub physicalDevice: PhysicalDevice,
    /// Handle to Vulkan device object.
    ///
    ///This is the same value as has been passed through AllocatorCreateInfo::device.
    pub device: Device,
}
/// Calculated statistics of memory usage e.g. in a specific memory type, heap, custom pool,
/// or total.
///
///These are fast to calculate.
///See functions: vmaGetHeapBudgets(), vmaGetPoolStatistics().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statistics {
    /// Number of `VkDeviceMemory` objects - Vulkan memory blocks allocated.
    pub block_count: u32,
    /// Number of #Allocation objects allocated.
    ///
    ///Dedicated allocations have their own blocks, so each one adds 1 to `allocationCount` as well
    /// as `blockCount`.
    pub allocation_count: u32,
    /// Number of bytes allocated in `VkDeviceMemory` blocks.
    ///
    ///\note To avoid confusion, please be aware that what Vulkan calls an "allocation" - a whole
    /// `VkDeviceMemory` object
    ///(e.g. as in `VkPhysicalDeviceLimits::maxMemoryAllocationCount`) is called a "block" in VMA,
    /// while VMA calls
    ///"allocation" a #Allocation object that represents a memory region sub-allocated from such
    /// block, usually for a single buffer or image.
    pub block_bytes: DeviceSize,
    /// Total number of bytes occupied by all #Allocation objects.
    ///
    ///Always less or equal than `blockBytes`.
    ///Difference `(blockBytes - allocationBytes)` is the amount of memory allocated from Vulkan
    ///but unused by any #Allocation.
    pub allocation_bytes: DeviceSize,
}
/// More detailed statistics than #Statistics.
///
///These are slower to calculate. Use for debugging purposes.
///See functions: vmaCalculateStatistics(), vmaCalculatePoolStatistics().
///
///Previous version of the statistics API provided averages, but they have been removed
///because they can be easily calculated as:
///
///\code
///VkDeviceSize allocationSizeAvg = detailedStats.statistics.allocationBytes /
/// detailedStats.statistics.allocationCount;
///VkDeviceSize unusedBytes = detailedStats.statistics.blockBytes -
/// detailedStats.statistics.allocationBytes;
///VkDeviceSize unusedRangeSizeAvg = unusedBytes / detailedStats.unusedRangeCount;
///\endcode
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetailedStatistics {
    /// Basic statistics.
    pub statistics: Statistics,
    /// Number of free ranges of memory between allocations.
    pub unused_range_count: u32,
    /// Smallest allocation size. `VK_WHOLE_SIZE` if there are 0 allocations.
    pub allocation_size_min: DeviceSize,
    /// Largest allocation size. 0 if there are 0 allocations.
    pub allocation_size_max: DeviceSize,
    /// Smallest empty range size. `VK_WHOLE_SIZE` if there are 0 empty ranges.
    pub unused_range_size_min: DeviceSize,
    /// Largest empty range size. 0 if there are 0 empty ranges.
    pub unused_range_size_max: DeviceSize,
}
///  General statistics from current state of the Allocator -
///total memory usage across all memory heaps and types.
///
///These are slower to calculate. Use for debugging purposes.
///See function vmaCalculateStatistics().
#[repr(C)]
pub struct TotalStatistics {
    pub memoryType: [DetailedStatistics; 32usize],
    pub memoryHeap: [DetailedStatistics; 16usize],
    pub total: DetailedStatistics,
}
/// Statistics of current memory usage and available budget for a specific memory heap.
///
///These are fast to calculate.
///See function vmaGetHeapBudgets().
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Budget {
    /// Statistics fetched from the library.
    pub statistics: Statistics,
    /// Estimated current memory usage of the program, in bytes.
    ///
    ///Fetched from system using VK_EXT_memory_budget extension if enabled.
    ///
    ///It might be different than `statistics.blockBytes` (usually higher) due to additional
    /// implicit objects
    ///also occupying the memory, like swapchain, pipelines, descriptor heaps, command buffers, or
    ///`VkDeviceMemory` blocks allocated outside of this library, if any.
    pub usage: DeviceSize,
    /// Estimated amount of memory available to the program, in bytes.
    ///
    ///Fetched from system using VK_EXT_memory_budget extension if enabled.
    ///
    ///It might be different (most probably smaller) than `VkMemoryHeap::size[heapIndex]` due to
    /// factors
    ///external to the program, decided by the operating system.
    ///Difference `budget - usage` is the amount of additional memory that can probably
    ///be allocated without problems. Exceeding the budget may result in various problems.
    pub budget: DeviceSize,
}
/// Parameters of new #Allocation.
///
///To be used with functions like vmaCreateBuffer(), vmaCreateImage(), and many others.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AllocationCreateInfo {
    /// See [`AllocationCreateFlags`].
    pub flags: AllocationCreateFlags,

    /// Intended usage of memory.
    ///
    /// You can leave [`MemoryUsage::UNKNOWN`] if you specify memory requirements in other way.
    ///
    /// If [`Self::pool`] is not null, this member is ignored.
    pub usage: MemoryUsage,

    /// Flags that must be set in a Memory Type chosen for an allocation.
    ///
    /// Leave 0 if you specify memory requirements in other way.
    ///
    /// If [`Self::pool`] is not null, this member is ignored.
    pub required_flags: MemoryPropertyFlags,

    /// Flags that preferably should be set in a memory type chosen for an allocation.
    ///
    /// Set to [`MemoryPropertyFlags::empty`] if no additional flags are preferred.
    ///
    /// If [`Self::pool`] is not null, this member is ignored.
    pub preferred_flags: MemoryPropertyFlags,

    /// Bitmask containing one bit set for every memory type acceptable for this allocation.
    ///
    /// Value 0 is equivalent to [`u32::MAX`] - it means any memory type is accepted if
    /// it meets other requirements specified by this structure, with no further
    /// restrictions on memory type index.
    ///
    /// If [`Self::pool`] is not null, this member is ignored.
    ///
    /// [`u32::MAX`]: std::u32::MAX
    pub memory_type_bits: u32,

    /// Pool that this allocation should be created in.
    ///
    /// Leave [`Pool::null`] to allocate from default pool. If not null, members:
    /// [`Self::usage`], [`Self::required_flags`], [`Self::preferred_flags`],
    /// [`Self::memory_type_bits`] are ignored.
    pub pool: Pool,

    /// Custom general-purpose pointer that will be stored in [`Allocation`], can be read as
    /// [`AllocationInfo::user_data`] and changed using [`Allocation::set_user_data`].
    ///
    /// If [`AllocationCreateFlags::USER_DATA_COPY_STRING`] is used, it must be either
    /// null or pointer to a null-terminated string. The string will be then copied to
    /// internal buffer, so it doesn't need to be valid after allocation call.
    pub user_data: *mut c_void,

    /// A floating-point value between 0.0 and 1.0, indicating the priority of the allocation
    /// relative to other memory allocations.
    ///
    /// It is used only when [`AllocatorCreateFlags::EXT_MEMORY_PRIORITY`] flag was used during
    /// creation of the #Allocator object
    /// and this allocation ends up as dedicated or is explicitly forced as dedicated using
    /// [`AllocatorCreateFlags::DEDICATED_ALLOCATION`].
    /// Otherwise, it has the priority of a memory block where it is placed and this variable is
    /// ignored.
    pub priority: f32,
}

impl Default for AllocationCreateInfo {
    fn default() -> Self {
        Self {
            flags: AllocationCreateFlags::empty(),
            usage: MemoryUsage::UNKNOWN,
            required_flags: MemoryPropertyFlags::empty(),
            preferred_flags: MemoryPropertyFlags::empty(),
            memory_type_bits: std::u32::MAX,
            pool: Pool::null(),
            user_data: std::ptr::null_mut(),
            priority: 0.0,
        }
    }
}

/// Describes parameter of created #Pool.
#[repr(C)]
pub struct PoolCreateInfo {
    /// Vulkan memory type index to allocate this pool from.
    pub memory_type_index: u32,

    /// Use combination of #PoolCreateFlagBits.
    pub flags: PoolCreateFlags,
    /// Size of a single `VkDeviceMemory` block to be allocated as part of this pool, in
    /// bytes. Optional.
    ///
    ///Specify nonzero to set explicit, constant size of memory blocks used by this
    ///pool.
    ///
    ///Leave 0 to use default and let the library manage block sizes automatically.
    ///Sizes of particular blocks may vary.
    ///In this case, the pool will also support dedicated allocations.
    pub block_size: DeviceSize,
    /// Minimum number of blocks to be always allocated in this pool, even if they stay
    /// empty.
    ///
    ///Set to 0 to have no preallocated blocks and allow the pool be completely empty.
    pub min_block_count: usize,
    /// Maximum number of blocks that can be allocated in this pool. Optional.
    ///
    ///Set to 0 to use default, which is `SIZE_MAX`, which means no limit.
    ///
    ///Set to same value as PoolCreateInfo::minBlockCount to have fixed amount of memory
    /// allocated
    ///throughout whole lifetime of this pool.
    pub max_block_count: usize,
    /// A floating-point value between 0 and 1, indicating the priority of the allocations in
    /// this pool relative to other memory allocations.
    ///
    ///It is used only when #VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT flag was used during
    /// creation of the #Allocator object.
    ///Otherwise, this variable is ignored.
    pub priority: f32,
    /// Additional minimum alignment to be used for all allocations created from this pool.
    /// Can be 0.
    ///
    ///Leave 0 (default) not to impose any additional alignment. If not 0, it must be a power of
    /// two.
    ///It can be useful in cases where alignment returned by Vulkan by functions like
    /// `vkGetBufferMemoryRequirements` is not enough,
    ///e.g. when doing interop with OpenGL.
    pub min_allocation_alignment: DeviceSize,
    /// Additional `pNext` chain to be attached to `VkMemoryAllocateInfo` used for every
    /// allocation made by this pool. Optional.
    ///
    ///Optional, can be null. If not null, it must point to a `pNext` chain of structures that can
    /// be attached to `VkMemoryAllocateInfo`.
    ///It can be useful for special needs such as adding `VkExportMemoryAllocateInfoKHR`.
    ///Structures pointed by this member must remain alive and unchanged for the whole lifetime of
    /// the custom pool.
    ///
    ///Please note that some structures, e.g. `VkMemoryPriorityAllocateInfoEXT`,
    /// `VkMemoryDedicatedAllocateInfoKHR`,
    ///can be attached automatically by this library when using other, more convenient of its
    /// features.
    pub memory_allocate_next: *mut c_void,
}
/// Parameters of #Allocation objects, that can be retrieved using function
/// vmaGetAllocationInfo().
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllocationInfo {
    /// Memory type index that this allocation was allocated from.
    ///
    ///It never changes.
    pub memory_type: u32,
    /// Handle to Vulkan memory object.
    ///
    ///Same memory object can be shared by multiple allocations.
    ///
    ///It can change after the allocation is moved during \ref defragmentation.
    pub device_memory: DeviceMemory,
    /// Offset in `VkDeviceMemory` object to the beginning of this allocation, in bytes.
    /// `(deviceMemory, offset)` pair is unique to this allocation.
    ///
    ///You usually don't need to use this offset. If you create a buffer or an image together with
    /// the allocation using e.g. function
    ///vmaCreateBuffer(), vmaCreateImage(), functions that operate on these resources refer to the
    /// beginning of the buffer or image,
    ///not entire device memory block. Functions like vmaMapMemory(), vmaBindBufferMemory() also
    /// refer to the beginning of the allocation
    ///and apply this offset automatically.
    ///
    ///It can change after the allocation is moved during \ref defragmentation.
    pub offset: DeviceSize,
    /// Size of this allocation, in bytes.
    ///
    ///It never changes.
    ///
    ///\note Allocation size returned in this variable may be greater than the size
    ///requested for the resource e.g. as `VkBufferCreateInfo::size`. Whole size of the
    ///allocation is accessible for operations on memory e.g. using a pointer after
    ///mapping with vmaMapMemory(), but operations on the resource e.g. using
    ///`vkCmdCopyBuffer` must be limited to the size of the resource.
    pub size: DeviceSize,
    /// Pointer to the beginning of this allocation as mapped data.
    ///
    ///If the allocation hasn't been mapped using vmaMapMemory() and hasn't been
    ///created with #VMA_ALLOCATION_CREATE_MAPPED_BIT flag, this value is null.
    ///
    ///It can change after call to vmaMapMemory(), vmaUnmapMemory().
    ///It can also change after the allocation is moved during \ref defragmentation.
    pub mapped_data: *mut c_void,
    /// Custom general-purpose pointer that was passed as AllocationCreateInfo::pUserData
    /// or set using vmaSetAllocationUserData().
    ///
    ///It can change after call to vmaSetAllocationUserData() for this allocation.
    pub user_data: *mut c_void,
    /// Custom allocation name that was set with vmaSetAllocationName().
    ///
    ///It can change after call to vmaSetAllocationName() for this allocation.
    ///
    ///Another way to set custom name is to pass it in AllocationCreateInfo::pUserData with
    ///additional flag #VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT set [DEPRECATED].
    pub name: *const c_char,
}

unsafe impl Send for AllocationInfo {}
unsafe impl Sync for AllocationInfo {}

/// Parameters for defragmentation.
///
///To be used with function vmaBeginDefragmentation().
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefragmentationInfo {
    /// Use combination of #DefragmentationFlagBits.
    pub flags: DefragmentationFlags,
    /// Custom pool to be defragmented.
    ///
    ///If null then default pools will undergo defragmentation process.
    pub pool: Pool,
    /// Maximum numbers of bytes that can be copied during single pass, while moving
    /// allocations to different places.
    ///
    ///`0` means no limit.
    pub max_bytes_per_pass: DeviceSize,
    /// Maximum number of allocations that can be moved during single pass to a different
    /// place.
    ///
    ///`0` means no limit.
    pub max_allocations_per_pass: u32,
}
/// Single move of an allocation to be done for defragmentation.
#[repr(C)]
pub struct DefragmentationMove {
    /// Operation to be performed on the allocation by vmaEndDefragmentationPass(). Default value is
    /// #VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY. You can modify it.
    pub operation: DefragmentationMoveOperation,
    /// Allocation that should be moved.
    pub srcAllocation: Allocation,
    /// Temporary allocation pointing to destination memory that will replace
    /// `srcAllocation`.
    ///
    ///\warning Do not store this allocation in your data structures! It exists only temporarily,
    /// for the duration of the defragmentation pass,
    ///to be used for binding new buffer/image to the destination memory using e.g.
    /// vmaBindBufferMemory().
    ///vmaEndDefragmentationPass() will destroy it and make `srcAllocation` point to this memory.
    pub dstTmpAllocation: Allocation,
}
/// Parameters for incremental defragmentation steps.
///
///To be used with function vmaBeginDefragmentationPass().
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefragmentationPassMoveInfo {
    /// Number of elements in the `pMoves` array.
    pub move_count: u32,
    /// Array of moves to be performed by the user in the current defragmentation pass.
    ///
    ///Pointer to an array of `moveCount` elements, owned by VMA, created in
    /// vmaBeginDefragmentationPass(), destroyed in vmaEndDefragmentationPass().
    ///
    ///For each element, you should:
    ///
    ///1. Create a new buffer/image in the place pointed by DefragmentationMove::dstMemory +
    /// DefragmentationMove::dstOffset.
    ///2. Copy data from the DefragmentationMove::srcAllocation e.g. using `vkCmdCopyBuffer`,
    /// `vkCmdCopyImage`.
    ///3. Make sure these commands finished executing on the GPU.
    ///4. Destroy the old buffer/image.
    ///
    ///Only then you can finish defragmentation pass by calling vmaEndDefragmentationPass().
    ///After this call, the allocation will point to the new place in memory.
    ///
    ///Alternatively, if you cannot move specific allocation, you can set
    /// DefragmentationMove::operation to #VMA_DEFRAGMENTATION_MOVE_OPERATION_IGNORE.
    ///
    ///Alternatively, if you decide you want to completely remove the allocation:
    ///
    ///1. Destroy its buffer/image.
    ///2. Set DefragmentationMove::operation to #VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY.
    ///
    ///Then, after vmaEndDefragmentationPass() the allocation will be freed.
    pub moves: *mut DefragmentationMove,
}

/// Statistics returned for defragmentation process in function vmaEndDefragmentation().
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefragmentationStats {
    /// Total number of bytes that have been copied while moving allocations to different places.
    pub bytes_moved: DeviceSize,
    /// Total number of bytes that have been released to the system by freeing empty
    /// `VkDeviceMemory` objects.
    pub bytes_freed: DeviceSize,
    /// Number of allocations that have been moved to different places.
    pub allocation_moved: u32,
    /// Number of empty `VkDeviceMemory` objects that have been released to the system.
    pub device_memory_blocks_freed: u32,
}
extern "C" {
    /// Creates #Allocator object.
    pub fn vmaCreateAllocator(pCreateInfo: *const AllocatorCreateInfo, pAllocator: *mut Allocator) -> Result;
}
extern "C" {
    /// Destroys allocator object.
    pub fn vmaDestroyAllocator(allocator: Allocator);
}
extern "C" {
    /// Returns information about existing #Allocator object - handle to Vulkan device
    /// etc.
    ///
    ///It might be useful if you want to keep just the #Allocator handle and fetch other
    /// required handles to
    ///`VkPhysicalDevice`, `VkDevice` etc. every time using this function.
    pub fn vmaGetAllocatorInfo(allocator: Allocator, pAllocatorInfo: *mut AllocatorInfo);
}
extern "C" {
    ///PhysicalDeviceProperties are fetched from physicalDevice by the allocator.
    ///You can access it here, without fetching it again on your own.
    pub fn vmaGetPhysicalDeviceProperties(
        allocator: Allocator,
        ppPhysicalDeviceProperties: *mut *const PhysicalDeviceProperties,
    );
}
extern "C" {
    ///PhysicalDeviceMemoryProperties are fetched from physicalDevice by the allocator.
    ///You can access it here, without fetching it again on your own.
    pub fn vmaGetMemoryProperties(
        allocator: Allocator,
        ppPhysicalDeviceMemoryProperties: *mut *const PhysicalDeviceMemoryProperties,
    );
}
extern "C" {
    ///Given Memory Type Index, returns Property Flags of this memory type.
    ///
    ///This is just a convenience function. Same information can be obtained using
    ///vmaGetMemoryProperties().
    pub fn vmaGetMemoryTypeProperties(allocator: Allocator, memoryTypeIndex: u32, pFlags: *mut MemoryPropertyFlags);
}
extern "C" {
    /// Sets index of the current frame.
    pub fn vmaSetCurrentFrameIndex(allocator: Allocator, frameIndex: u32);
}
extern "C" {
    /// Retrieves statistics from current state of the Allocator.
    ///
    ///This function is called "calculate" not "get" because it has to traverse all
    ///internal data structures, so it may be quite slow. Use it for debugging purposes.
    ///For faster but more brief statistics suitable to be called every frame or every allocation,
    ///use vmaGetHeapBudgets().
    ///
    ///Note that when using allocator from multiple threads, returned information may immediately
    ///become outdated.
    pub fn vmaCalculateStatistics(allocator: Allocator, pStats: *mut TotalStatistics);
}
extern "C" {
    /// Retrieves information about current memory usage and budget for all memory heaps.
    ///
    ///\param allocator
    ///\param[out] pBudgets Must point to array with number of elements at least equal to number of
    /// memory heaps in physical device used.
    ///
    ///This function is called "get" not "calculate" because it is very fast, suitable to be called
    ///every frame or every allocation. For more detailed statistics use vmaCalculateStatistics().
    ///
    ///Note that when using allocator from multiple threads, returned information may immediately
    ///become outdated.
    pub fn vmaGetHeapBudgets(allocator: Allocator, pBudgets: *mut Budget);
}
extern "C" {
    ///Helps to find memoryTypeIndex, given memoryTypeBits and AllocationCreateInfo.
    ///
    ///This algorithm tries to find a memory type that:
    ///
    /// - Is allowed by memoryTypeBits.
    /// - Contains all the flags from pAllocationCreateInfo->requiredFlags.
    /// - Matches intended usage.
    /// - Has as many flags from pAllocationCreateInfo->preferredFlags as possible.
    ///
    ///\return Returns VK_ERROR_FEATURE_NOT_PRESENT if not found. Receiving such result
    ///from this function or any other allocating function probably means that your
    ///device doesn't support any memory type with requested features for the specific
    ///type of resource you want to use it for. Please check parameters of your
    ///resource, like image layout (OPTIMAL versus LINEAR) or mip level count.
    pub fn vmaFindMemoryTypeIndex(
        allocator: Allocator,
        memoryTypeBits: u32,
        pAllocationCreateInfo: *const AllocationCreateInfo,
        pMemoryTypeIndex: *mut u32,
    ) -> Result;
}
extern "C" {
    ///Helps to find memoryTypeIndex, given VkBufferCreateInfo and AllocationCreateInfo.
    ///
    ///It can be useful e.g. to determine value to be used as PoolCreateInfo::memoryTypeIndex.
    ///It internally creates a temporary, dummy buffer that never has memory bound.
    pub fn vmaFindMemoryTypeIndexForBufferInfo(
        allocator: Allocator,
        pBufferCreateInfo: *const BufferCreateInfo,
        pAllocationCreateInfo: *const AllocationCreateInfo,
        pMemoryTypeIndex: *mut u32,
    ) -> Result;
}
extern "C" {
    ///Helps to find memoryTypeIndex, given VkImageCreateInfo and AllocationCreateInfo.
    ///
    ///It can be useful e.g. to determine value to be used as PoolCreateInfo::memoryTypeIndex.
    ///It internally creates a temporary, dummy image that never has memory bound.
    pub fn vmaFindMemoryTypeIndexForImageInfo(
        allocator: Allocator,
        pImageCreateInfo: *const ImageCreateInfo,
        pAllocationCreateInfo: *const AllocationCreateInfo,
        pMemoryTypeIndex: *mut u32,
    ) -> Result;
}
extern "C" {
    /// Allocates Vulkan device memory and creates #Pool object.
    ///
    ///\param allocator Allocator object.
    ///\param pCreateInfo Parameters of pool to create.
    ///\param[out] pPool Handle to created pool.
    pub fn vmaCreatePool(allocator: Allocator, pCreateInfo: *const PoolCreateInfo, pPool: *mut Pool) -> Result;
}
extern "C" {
    /// Destroys #Pool object and frees Vulkan device memory.
    pub fn vmaDestroyPool(allocator: Allocator, pool: Pool);
}
extern "C" {
    /// Retrieves statistics of existing #Pool object.
    ///
    ///\param allocator Allocator object.
    ///\param pool Pool object.
    ///\param[out] pPoolStats Statistics of specified pool.
    pub fn vmaGetPoolStatistics(allocator: Allocator, pool: Pool, pPoolStats: *mut Statistics);
}
extern "C" {
    /// Retrieves detailed statistics of existing #Pool object.
    ///
    ///\param allocator Allocator object.
    ///\param pool Pool object.
    ///\param[out] pPoolStats Statistics of specified pool.
    pub fn vmaCalculatePoolStatistics(allocator: Allocator, pool: Pool, pPoolStats: *mut DetailedStatistics);
}
extern "C" {
    /// Checks magic number in margins around all allocations in given memory pool in search
    /// for corruptions.
    ///
    ///Corruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to
    /// nonzero,
    ///`VMA_DEBUG_MARGIN` is defined to nonzero and the pool is created in memory type that is
    ///`HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref
    /// debugging_memory_usage_corruption_detection).
    ///
    ///Possible return values:
    ///
    /// - `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for specified pool.
    /// - `VK_SUCCESS` - corruption detection has been performed and succeeded.
    /// - `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions
    ///   around one of the allocations.
    ///`VMA_ASSERT` is also fired in that case.
    /// - Other value: Error returned by Vulkan, e.g. memory mapping failure.
    pub fn vmaCheckPoolCorruption(allocator: Allocator, pool: Pool) -> Result;
}
extern "C" {
    /// Retrieves name of a custom pool.
    ///
    ///After the call `ppName` is either null or points to an internally-owned null-terminated
    /// string
    ///containing name of the pool that was previously set. The pointer becomes invalid when the
    /// pool is
    ///destroyed or its name is changed using vmaSetPoolName().
    pub fn vmaGetPoolName(allocator: Allocator, pool: Pool, ppName: *mut *const c_char);
}
extern "C" {
    /// Sets name of a custom pool.
    ///
    ///`pName` can be either null or pointer to a null-terminated string with new name for the
    /// pool.
    ///Function makes internal copy of the string, so it can be changed or freed immediately after
    /// this call.
    pub fn vmaSetPoolName(allocator: Allocator, pool: Pool, pName: *const c_char);
}
extern "C" {
    /// General purpose memory allocation.
    ///
    ///\param allocator
    ///\param pVkMemoryRequirements
    ///\param pCreateInfo
    ///\param[out] pAllocation Handle to allocated memory.
    ///\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later
    /// fetched using function vmaGetAllocationInfo().
    ///
    ///You should free the memory using vmaFreeMemory() or vmaFreeMemoryPages().
    ///
    ///It is recommended to use vmaAllocateMemoryForBuffer(), vmaAllocateMemoryForImage(),
    ///vmaCreateBuffer(), vmaCreateImage() instead whenever possible.
    pub fn vmaAllocateMemory(
        allocator: Allocator,
        pVkMemoryRequirements: *const MemoryRequirements,
        pCreateInfo: *const AllocationCreateInfo,
        pAllocation: *mut Allocation,
        pAllocationInfo: *mut AllocationInfo,
    ) -> Result;
}
extern "C" {
    /// General purpose memory allocation for multiple allocation objects at once.
    ///
    ///\param allocator Allocator object.
    ///\param pVkMemoryRequirements Memory requirements for each allocation.
    ///\param pCreateInfo Creation parameters for each allocation.
    ///\param allocationCount Number of allocations to make.
    ///\param[out] pAllocations Pointer to array that will be filled with handles to created
    /// allocations.
    ///\param[out] pAllocationInfo Optional. Pointer to array that will be filled with parameters
    /// of created allocations.
    ///
    ///You should free the memory using vmaFreeMemory() or vmaFreeMemoryPages().
    ///
    ///Word "pages" is just a suggestion to use this function to allocate pieces of memory needed
    /// for sparse binding.
    ///It is just a general purpose allocation function able to make multiple allocations at once.
    ///It may be internally optimized to be more efficient than calling vmaAllocateMemory()
    /// `allocationCount` times.
    ///
    ///All allocations are made using same parameters. All of them are created out of the same
    /// memory pool and type.
    ///If any allocation fails, all allocations already made within this function call are also
    /// freed, so that when
    ///returned result is not `VK_SUCCESS`, `pAllocation` array is always entirely filled with
    /// `VK_NULL_HANDLE`.
    pub fn vmaAllocateMemoryPages(
        allocator: Allocator,
        pVkMemoryRequirements: *const MemoryRequirements,
        pCreateInfo: *const AllocationCreateInfo,
        allocationCount: usize,
        pAllocations: *mut Allocation,
        pAllocationInfo: *mut AllocationInfo,
    ) -> Result;
}
extern "C" {
    /// Allocates memory suitable for given `VkBuffer`.
    ///
    ///\param allocator
    ///\param buffer
    ///\param pCreateInfo
    ///\param[out] pAllocation Handle to allocated memory.
    ///\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later
    /// fetched using function vmaGetAllocationInfo().
    ///
    ///It only creates #Allocation. To bind the memory to the buffer, use vmaBindBufferMemory().
    ///
    ///This is a special-purpose function. In most cases you should use vmaCreateBuffer().
    ///
    ///You must free the allocation using vmaFreeMemory() when no longer needed.
    pub fn vmaAllocateMemoryForBuffer(
        allocator: Allocator,
        buffer: Buffer,
        pCreateInfo: *const AllocationCreateInfo,
        pAllocation: *mut Allocation,
        pAllocationInfo: *mut AllocationInfo,
    ) -> Result;
}
extern "C" {
    /// Allocates memory suitable for given `VkImage`.
    ///
    ///\param allocator
    ///\param image
    ///\param pCreateInfo
    ///\param[out] pAllocation Handle to allocated memory.
    ///\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later
    /// fetched using function vmaGetAllocationInfo().
    ///
    ///It only creates #Allocation. To bind the memory to the buffer, use vmaBindImageMemory().
    ///
    ///This is a special-purpose function. In most cases you should use vmaCreateImage().
    ///
    ///You must free the allocation using vmaFreeMemory() when no longer needed.
    pub fn vmaAllocateMemoryForImage(
        allocator: Allocator,
        image: Image,
        pCreateInfo: *const AllocationCreateInfo,
        pAllocation: *mut Allocation,
        pAllocationInfo: *mut AllocationInfo,
    ) -> Result;
}
extern "C" {
    /// Frees memory previously allocated using vmaAllocateMemory(),
    /// vmaAllocateMemoryForBuffer(), or vmaAllocateMemoryForImage().
    ///
    ///Passing `VK_NULL_HANDLE` as `allocation` is valid. Such function call is just skipped.
    pub fn vmaFreeMemory(allocator: Allocator, allocation: Allocation);
}
extern "C" {
    /// Returns current information about specified allocation.
    ///
    ///Current paramteres of given allocation are returned in `pAllocationInfo`.
    ///
    ///Although this function doesn't lock any mutex, so it should be quite efficient,
    ///you should avoid calling it too often.
    ///You can retrieve same AllocationInfo structure while creating your resource, from
    /// function
    ///vmaCreateBuffer(), vmaCreateImage(). You can remember it if you are sure parameters don't
    /// change
    ///(e.g. due to defragmentation).
    pub fn vmaGetAllocationInfo(allocator: Allocator, allocation: Allocation, pAllocationInfo: *mut AllocationInfo);
}
extern "C" {
    /// Sets pUserData in given allocation to new value.
    ///
    ///The value of pointer `pUserData` is copied to allocation's `pUserData`.
    ///It is opaque, so you can use it however you want - e.g.
    ///as a pointer, ordinal number or some handle to you own data.
    pub fn vmaSetAllocationUserData(allocator: Allocator, allocation: Allocation, pUserData: *mut c_void);
}
extern "C" {
    /// Sets pName in given allocation to new value.
    ///
    ///`pName` must be either null, or pointer to a null-terminated string. The function
    ///makes local copy of the string and sets it as allocation's `pName`. String
    ///passed as pName doesn't need to be valid for whole lifetime of the allocation -
    ///you can free it after this call. String previously pointed by allocation's
    ///`pName` is freed from memory.
    pub fn vmaSetAllocationName(allocator: Allocator, allocation: Allocation, pName: *const c_char);
}
extern "C" {
    ///Given an allocation, returns Property Flags of its memory type.
    ///
    ///This is just a convenience function. Same information can be obtained using
    ///vmaGetAllocationInfo() + vmaGetMemoryProperties().
    pub fn vmaGetAllocationMemoryProperties(
        allocator: Allocator,
        allocation: Allocation,
        pFlags: *mut MemoryPropertyFlags,
    );
}
extern "C" {
    /// Maps memory represented by given allocation and returns pointer to it.
    ///
    ///Maps memory represented by given allocation to make it accessible to CPU code.
    ///When succeeded, `*ppData` contains pointer to first byte of this memory.
    ///
    ///\warning
    ///If the allocation is part of a bigger `VkDeviceMemory` block, returned pointer is
    ///correctly offsetted to the beginning of region assigned to this particular allocation.
    ///Unlike the result of `vkMapMemory`, it points to the allocation, not to the beginning of the
    /// whole block.
    ///You should not add AllocationInfo::offset to it!
    ///
    ///Mapping is internally reference-counted and synchronized, so despite raw Vulkan
    ///function `vkMapMemory()` cannot be used to map same block of `VkDeviceMemory`
    ///multiple times simultaneously, it is safe to call this function on allocations
    ///assigned to the same memory block. Actual Vulkan memory will be mapped on first
    ///mapping and unmapped on last unmapping.
    ///
    ///If the function succeeded, you must call vmaUnmapMemory() to unmap the
    ///allocation when mapping is no longer needed or before freeing the allocation, at
    ///the latest.
    ///
    ///It also safe to call this function multiple times on the same allocation. You
    ///must call vmaUnmapMemory() same number of times as you called vmaMapMemory().
    ///
    ///It is also safe to call this function on allocation created with
    ///#VMA_ALLOCATION_CREATE_MAPPED_BIT flag. Its memory stays mapped all the time.
    ///You must still call vmaUnmapMemory() same number of times as you called
    ///vmaMapMemory(). You must not call vmaUnmapMemory() additional time to free the
    ///"0-th" mapping made automatically due to #VMA_ALLOCATION_CREATE_MAPPED_BIT flag.
    ///
    ///This function fails when used on allocation made in memory type that is not
    ///`HOST_VISIBLE`.
    ///
    ///This function doesn't automatically flush or invalidate caches.
    ///If the allocation is made from a memory types that is not `HOST_COHERENT`,
    ///you also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan
    /// specification.
    pub fn vmaMapMemory(allocator: Allocator, allocation: Allocation, ppData: *mut *mut c_void) -> Result;
}
extern "C" {
    /// Unmaps memory represented by given allocation, mapped previously using
    /// vmaMapMemory().
    ///
    ///For details, see description of vmaMapMemory().
    ///
    ///This function doesn't automatically flush or invalidate caches.
    ///If the allocation is made from a memory types that is not `HOST_COHERENT`,
    ///you also need to use vmaInvalidateAllocation() / vmaFlushAllocation(), as required by Vulkan
    /// specification.
    pub fn vmaUnmapMemory(allocator: Allocator, allocation: Allocation);
}
extern "C" {
    /// Flushes memory of given allocation.
    ///
    ///Calls `vkFlushMappedMemoryRanges()` for memory associated with given range of given
    /// allocation.
    ///It needs to be called after writing to a mapped memory for memory types that are not
    /// `HOST_COHERENT`.
    ///Unmap operation doesn't do that automatically.
    ///
    /// - `offset` must be relative to the beginning of allocation.
    /// - `size` can be `VK_WHOLE_SIZE`. It means all memory from `offset` the the end of given
    ///   allocation.
    /// - `offset` and `size` don't have to be aligned.
    ///They are internally rounded down/up to multiply of `nonCoherentAtomSize`.
    /// - If `size` is 0, this call is ignored.
    /// - If memory type that the `allocation` belongs to is not `HOST_VISIBLE` or it is
    ///   `HOST_COHERENT`,
    ///this call is ignored.
    ///
    ///Warning! `offset` and `size` are relative to the contents of given `allocation`.
    ///If you mean whole allocation, you can pass 0 and `VK_WHOLE_SIZE`, respectively.
    ///Do not pass allocation's offset as `offset`!!!
    ///
    ///This function returns the `VkResult` from `vkFlushMappedMemoryRanges` if it is
    ///called, otherwise `VK_SUCCESS`.
    pub fn vmaFlushAllocation(
        allocator: Allocator,
        allocation: Allocation,
        offset: DeviceSize,
        size: DeviceSize,
    ) -> Result;
}
extern "C" {
    /// Invalidates memory of given allocation.
    ///
    ///Calls `vkInvalidateMappedMemoryRanges()` for memory associated with given range of given
    /// allocation.
    ///It needs to be called before reading from a mapped memory for memory types that are not
    /// `HOST_COHERENT`.
    ///Map operation doesn't do that automatically.
    ///
    /// - `offset` must be relative to the beginning of allocation.
    /// - `size` can be `VK_WHOLE_SIZE`. It means all memory from `offset` the the end of given
    ///   allocation.
    /// - `offset` and `size` don't have to be aligned.
    ///They are internally rounded down/up to multiply of `nonCoherentAtomSize`.
    /// - If `size` is 0, this call is ignored.
    /// - If memory type that the `allocation` belongs to is not `HOST_VISIBLE` or it is
    ///   `HOST_COHERENT`,
    ///this call is ignored.
    ///
    ///Warning! `offset` and `size` are relative to the contents of given `allocation`.
    ///If you mean whole allocation, you can pass 0 and `VK_WHOLE_SIZE`, respectively.
    ///Do not pass allocation's offset as `offset`!!!
    ///
    ///This function returns the `VkResult` from `vkInvalidateMappedMemoryRanges` if
    ///it is called, otherwise `VK_SUCCESS`.
    pub fn vmaInvalidateAllocation(
        allocator: Allocator,
        allocation: Allocation,
        offset: DeviceSize,
        size: DeviceSize,
    ) -> Result;
}
extern "C" {
    /// Checks magic number in margins around all allocations in given memory types (in both
    /// default and custom pools) in search for corruptions.
    ///
    ///\param allocator
    ///\param memoryTypeBits Bit mask, where each bit set means that a memory type with that index
    /// should be checked.
    ///
    ///Corruption detection is enabled only when `VMA_DEBUG_DETECT_CORRUPTION` macro is defined to
    /// nonzero,
    ///`VMA_DEBUG_MARGIN` is defined to nonzero and only for memory types that are
    ///`HOST_VISIBLE` and `HOST_COHERENT`. For more information, see [Corruption detection](@ref
    /// debugging_memory_usage_corruption_detection).
    ///
    ///Possible return values:
    ///
    /// - `VK_ERROR_FEATURE_NOT_PRESENT` - corruption detection is not enabled for any of specified
    ///   memory types.
    /// - `VK_SUCCESS` - corruption detection has been performed and succeeded.
    /// - `VK_ERROR_UNKNOWN` - corruption detection has been performed and found memory corruptions
    ///   around one of the allocations.
    ///`VMA_ASSERT` is also fired in that case.
    /// - Other value: Error returned by Vulkan, e.g. memory mapping failure.
    pub fn vmaCheckCorruption(allocator: Allocator, memoryTypeBits: u32) -> Result;
}
extern "C" {
    /// Begins defragmentation process.
    ///
    ///\param allocator Allocator object.
    ///\param pInfo Structure filled with parameters of defragmentation.
    ///\param[out] pContext Context object that must be passed to vmaEndDefragmentation() to finish
    /// defragmentation.
    ///\returns
    /// - `VK_SUCCESS` if defragmentation can begin.
    /// - `VK_ERROR_FEATURE_NOT_PRESENT` if defragmentation is not supported.
    ///
    ///For more information about defragmentation, see documentation chapter:
    ///[Defragmentation](@ref defragmentation).
    pub fn vmaBeginDefragmentation(
        allocator: Allocator,
        pInfo: *const DefragmentationInfo,
        pContext: *mut DefragmentationContextHandle,
    ) -> Result;
}
extern "C" {
    /// Ends defragmentation process.
    ///
    ///\param allocator Allocator object.
    ///\param context Context object that has been created by vmaBeginDefragmentation().
    ///\param[out] pStats Optional stats for the defragmentation. Can be null.
    ///
    ///Use this function to finish defragmentation started by vmaBeginDefragmentation().
    pub fn vmaEndDefragmentation(
        allocator: Allocator,
        context: DefragmentationContextHandle,
        pStats: *mut DefragmentationStats,
    );
}
extern "C" {
    /// Starts single defragmentation pass.
    ///
    ///\param allocator Allocator object.
    ///\param context Context object that has been created by vmaBeginDefragmentation().
    ///\param[out] pPassInfo Computed informations for current pass.
    ///\returns
    /// - `VK_SUCCESS` if no more moves are possible. Then you can omit call to
    ///   vmaEndDefragmentationPass() and simply end whole defragmentation.
    /// - `VK_INCOMPLETE` if there are pending moves returned in `pPassInfo`. You need to perform
    ///   them, call vmaEndDefragmentationPass(),
    ///and then preferably try another pass with vmaBeginDefragmentationPass().
    pub fn vmaBeginDefragmentationPass(
        allocator: Allocator,
        context: DefragmentationContextHandle,
        pPassInfo: *mut DefragmentationPassMoveInfo,
    ) -> Result;
}
extern "C" {
    /// Ends single defragmentation pass.
    ///
    ///\param allocator Allocator object.
    ///\param context Context object that has been created by vmaBeginDefragmentation().
    ///\param pPassInfo Computed informations for current pass filled by
    /// vmaBeginDefragmentationPass() and possibly modified by you.
    ///
    ///Returns `VK_SUCCESS` if no more moves are possible or `VK_INCOMPLETE` if more
    /// defragmentations are possible.
    ///
    ///Ends incremental defragmentation pass and commits all defragmentation moves from
    /// `pPassInfo`.
    ///After this call:
    ///
    /// - Allocations at `pPassInfo[i].srcAllocation` that had `pPassInfo[i].operation ==`
    ///   #VMA_DEFRAGMENTATION_MOVE_OPERATION_COPY
    ///(which is the default) will be pointing to the new destination place.
    /// - Allocation at `pPassInfo[i].srcAllocation` that had `pPassInfo[i].operation ==`
    ///   #VMA_DEFRAGMENTATION_MOVE_OPERATION_DESTROY
    ///will be freed.
    ///
    ///If no more moves are possible you can end whole defragmentation.
    pub fn vmaEndDefragmentationPass(
        allocator: Allocator,
        context: DefragmentationContextHandle,
        pPassInfo: *mut DefragmentationPassMoveInfo,
    ) -> Result;
}
extern "C" {
    /// Binds buffer to allocation.
    ///
    ///Binds specified buffer to region of memory represented by specified allocation.
    ///Gets `VkDeviceMemory` handle and offset from the allocation.
    ///If you want to create a buffer, allocate memory for it and bind them together separately,
    ///you should use this function for binding instead of standard `vkBindBufferMemory()`,
    ///because it ensures proper synchronization so that when a `VkDeviceMemory` object is used by
    /// multiple
    ///allocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple
    /// threads simultaneously
    ///(which is illegal in Vulkan).
    ///
    ///It is recommended to use function vmaCreateBuffer() instead of this one.
    pub fn vmaBindBufferMemory(allocator: Allocator, allocation: Allocation, buffer: Buffer) -> Result;
}
extern "C" {
    /// Binds buffer to allocation with additional parameters.
    ///
    ///\param allocator
    ///\param allocation
    ///\param allocationLocalOffset Additional offset to be added while binding, relative to the
    /// beginning of the `allocation`. Normally it should be 0.
    ///\param buffer
    ///\param pNext A chain of structures to be attached to `VkBindBufferMemoryInfoKHR` structure
    /// used internally. Normally it should be null.
    ///
    ///This function is similar to vmaBindBufferMemory(), but it provides additional parameters.
    ///
    ///If `pNext` is not null, #Allocator object must have been created with
    /// #VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT flag
    ///or with AllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call
    /// fails.
    pub fn vmaBindBufferMemory2(
        allocator: Allocator,
        allocation: Allocation,
        allocationLocalOffset: DeviceSize,
        buffer: Buffer,
        pNext: *const c_void,
    ) -> Result;
}
extern "C" {
    /// Binds image to allocation.
    ///
    ///Binds specified image to region of memory represented by specified allocation.
    ///Gets `VkDeviceMemory` handle and offset from the allocation.
    ///If you want to create an image, allocate memory for it and bind them together separately,
    ///you should use this function for binding instead of standard `vkBindImageMemory()`,
    ///because it ensures proper synchronization so that when a `VkDeviceMemory` object is used by
    /// multiple
    ///allocations, calls to `vkBind*Memory()` or `vkMapMemory()` won't happen from multiple
    /// threads simultaneously
    ///(which is illegal in Vulkan).
    ///
    ///It is recommended to use function vmaCreateImage() instead of this one.
    pub fn vmaBindImageMemory(allocator: Allocator, allocation: Allocation, image: Image) -> Result;
}
extern "C" {
    /// Binds image to allocation with additional parameters.
    ///
    ///\param allocator
    ///\param allocation
    ///\param allocationLocalOffset Additional offset to be added while binding, relative to the
    /// beginning of the `allocation`. Normally it should be 0.
    ///\param image
    ///\param pNext A chain of structures to be attached to `VkBindImageMemoryInfoKHR` structure
    /// used internally. Normally it should be null.
    ///
    ///This function is similar to vmaBindImageMemory(), but it provides additional parameters.
    ///
    ///If `pNext` is not null, #Allocator object must have been created with
    /// #VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT flag
    ///or with AllocatorCreateInfo::vulkanApiVersion `>= VK_API_VERSION_1_1`. Otherwise the call
    /// fails.
    pub fn vmaBindImageMemory2(
        allocator: Allocator,
        allocation: Allocation,
        allocationLocalOffset: DeviceSize,
        image: Image,
        pNext: *const c_void,
    ) -> Result;
}
extern "C" {
    /// Creates a new `VkBuffer`, allocates and binds memory for it.
    ///
    ///\param allocator
    ///\param pBufferCreateInfo
    ///\param pAllocationCreateInfo
    ///\param[out] pBuffer Buffer that was created.
    ///\param[out] pAllocation Allocation that was created.
    ///\param[out] pAllocationInfo Optional. Information about allocated memory. It can be later
    /// fetched using function vmaGetAllocationInfo().
    ///
    ///This function automatically:
    ///
    ///-# Creates buffer.
    ///-# Allocates appropriate memory for it.
    ///-# Binds the buffer with the memory.
    ///
    ///If any of these operations fail, buffer and allocation are not created,
    ///returned value is negative error code, `*pBuffer` and `*pAllocation` are null.
    ///
    ///If the function succeeded, you must destroy both buffer and allocation when you
    ///no longer need them using either convenience function vmaDestroyBuffer() or
    ///separately, using `vkDestroyBuffer()` and vmaFreeMemory().
    ///
    ///If #VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT flag was used,
    ///VK_KHR_dedicated_allocation extension is used internally to query driver whether
    ///it requires or prefers the new buffer to have dedicated allocation. If yes,
    ///and if dedicated allocation is possible
    ///(#VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT is not used), it creates dedicated
    ///allocation for this buffer, just like when using
    ///#VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT.
    ///
    ///\note This function creates a new `VkBuffer`. Sub-allocation of parts of one large buffer,
    ///although recommended as a good practice, is out of scope of this library and could be
    /// implemented
    ///by the user as a higher-level logic on top of VMA.
    pub fn vmaCreateBuffer<'lt>(
        allocator: Allocator,
        pBufferCreateInfo: *const BufferCreateInfo<'lt>,
        pAllocationCreateInfo: *const AllocationCreateInfo,
        pBuffer: *mut Buffer,
        pAllocation: *mut Allocation,
        pAllocationInfo: *mut AllocationInfo,
    ) -> Result;
}
extern "C" {
    /// Creates a buffer with additional minimum alignment.
    ///
    ///Similar to vmaCreateBuffer() but provides additional parameter `minAlignment` which allows
    /// to specify custom,
    ///minimum alignment to be used when placing the buffer inside a larger memory block, which may
    /// be needed e.g.
    ///for interop with OpenGL.
    pub fn vmaCreateBufferWithAlignment<'lt>(
        allocator: Allocator,
        pBufferCreateInfo: *const BufferCreateInfo<'lt>,
        pAllocationCreateInfo: *const AllocationCreateInfo,
        minAlignment: DeviceSize,
        pBuffer: *mut Buffer,
        pAllocation: *mut Allocation,
        pAllocationInfo: *mut AllocationInfo,
    ) -> Result;
}
extern "C" {
    /// Creates a new `VkBuffer`, binds already created memory for it.
    ///
    ///\param allocator
    ///\param allocation Allocation that provides memory to be used for binding new buffer to it.
    ///\param pBufferCreateInfo
    ///\param[out] pBuffer Buffer that was created.
    ///
    ///This function automatically:
    ///
    ///-# Creates buffer.
    ///-# Binds the buffer with the supplied memory.
    ///
    ///If any of these operations fail, buffer is not created,
    ///returned value is negative error code and `*pBuffer` is null.
    ///
    ///If the function succeeded, you must destroy the buffer when you
    ///no longer need it using `vkDestroyBuffer()`. If you want to also destroy the corresponding
    ///allocation you can use convenience function vmaDestroyBuffer().
    pub fn vmaCreateAliasingBuffer(
        allocator: Allocator,
        allocation: Allocation,
        pBufferCreateInfo: *const BufferCreateInfo,
        pBuffer: *mut Buffer,
    ) -> Result;
}
extern "C" {
    /// Destroys Vulkan buffer and frees allocated memory.
    ///
    ///This is just a convenience function equivalent to:
    ///
    ///\code
    ///vkDestroyBuffer(device, buffer, allocationCallbacks);
    ///vmaFreeMemory(allocator, allocation);
    ///\endcode
    ///
    ///It it safe to pass null as buffer and/or allocation.
    pub fn vmaDestroyBuffer(allocator: Allocator, buffer: Buffer, allocation: Allocation);
}
extern "C" {
    /// Function similar to vmaCreateBuffer().
    pub fn vmaCreateImage(
        allocator: Allocator,
        pImageCreateInfo: *const ImageCreateInfo,
        pAllocationCreateInfo: *const AllocationCreateInfo,
        pImage: *mut Image,
        pAllocation: *mut Allocation,
        pAllocationInfo: *mut AllocationInfo,
    ) -> Result;
}
extern "C" {
    /// Function similar to vmaCreateAliasingBuffer().
    pub fn vmaCreateAliasingImage(
        allocator: Allocator,
        allocation: Allocation,
        pImageCreateInfo: *const ImageCreateInfo,
        pImage: *mut Image,
    ) -> Result;
}
extern "C" {
    /// Destroys Vulkan image and frees allocated memory.
    ///
    ///This is just a convenience function equivalent to:
    ///
    ///\code
    ///vkDestroyImage(device, image, allocationCallbacks);
    ///vmaFreeMemory(allocator, allocation);
    ///\endcode
    ///
    ///It it safe to pass null as image and/or allocation.
    pub fn vmaDestroyImage(allocator: Allocator, image: Image, allocation: Allocation);
}
extern "C" {
    /// Builds and returns statistics as a null-terminated string in JSON format.
    ///\param allocator
    ///\param[out] ppStatsString Must be freed using vmaFreeStatsString() function.
    ///\param detailedMap
    pub fn vmaBuildStatsString(allocator: Allocator, ppStatsString: *mut *mut c_char, detailedMap: Bool32);
}
extern "C" {
    pub fn vmaFreeStatsString(allocator: Allocator, pStatsString: *mut c_char);
}
