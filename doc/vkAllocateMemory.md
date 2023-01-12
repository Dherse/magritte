[vkAllocateMemory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateMemory.html) - Allocate device memory

# C Specifications
To allocate memory objects, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkAllocateMemory(
    VkDevice                                    device,
    const VkMemoryAllocateInfo*                 pAllocateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDeviceMemory*                             pMemory);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`p_allocate_info`] is a pointer to a [`MemoryAllocateInfo`] structure describing parameters of the allocation. A successfully returned allocation  **must**  use the requested parameters — no substitution is permitted by the implementation.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_memory`] is a pointer to a [`DeviceMemory`] handle in which information about the allocated memory is returned.

# Description
Allocations returned by [`allocate_memory`] are guaranteed to meet any
alignment requirement of the implementation.
For example, if an implementation requires 128 byte alignment for images and
64 byte alignment for buffers, the device memory returned through this
mechanism would be 128-byte aligned.
This ensures that applications  **can**  correctly suballocate objects of
different types (with potentially different alignment requirements) in the
same memory object.When memory is allocated, its contents are undefined with the following
constraint:
- The contents of unprotected memory  **must**  not be a function of the contents of data protected memory objects, even if those memory objects were previously freed.
The maximum number of valid memory allocations that  **can**  exist
simultaneously within a [`Device`] **may**  be restricted by implementation-
or platform-dependent limits.
The [`maxMemoryAllocationCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxMemoryAllocationCount)
feature describes the number of allocations that  **can**  exist simultaneously
before encountering these internal limits.Some platforms  **may**  have a limit on the maximum size of a single allocation.
For example, certain systems  **may**  fail to create allocations with a size
greater than or equal to 4GB.
Such a limit is implementation-dependent, and if such a failure occurs then
the error `VK_ERROR_OUT_OF_DEVICE_MEMORY` **must**  be returned.
This limit is advertised in
[`PhysicalDeviceMaintenance3Properties::max_memory_allocation_size`].The cumulative memory size allocated to a heap  **can**  be limited by the size
of the specified heap.
In such cases, allocated memory is tracked on a per-device and per-heap
basis.
Some platforms allow overallocation into other heaps.
The overallocation behavior  **can**  be specified through the
`[`amd_memory_overallocation_behavior`]` extension.If the
[`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT::pageable_device_local_memory`]
feature is enabled, memory allocations made from a heap that includes
`VK_MEMORY_HEAP_DEVICE_LOCAL_BIT` in [`MemoryHeap::flags`] **may**  be transparently moved to host-local memory allowing multiple
applications to share device-local memory.
If there is no space left in device-local memory when this new allocation is
made, other allocations  **may**  be moved out transparently to make room.
The operating system will determine which allocations to move to
device-local memory or host-local memory based on platform-specific
criteria.
To help the operating system make good choices, the application  **should**  set
the appropriate memory priority with [`MemoryPriorityAllocateInfoEXT`]
and adjust it as necessary with [`set_device_memory_priority_ext`].
Higher priority allocations will moved to device-local memory first.Memory allocations made on heaps without the
`VK_MEMORY_HEAP_DEVICE_LOCAL_BIT` property will not be transparently
promoted to device-local memory by the operating system.
## Valid Usage
-  `pAllocateInfo->allocationSize` **must**  be less than or equal to [`PhysicalDeviceMemoryProperties::memory_heaps`][`memindex`].`size` where `memindex` = [`PhysicalDeviceMemoryProperties::memory_types`][`pAllocateInfo->memoryTypeIndex`].`heapIndex` as returned by [`get_physical_device_memory_properties`] for the [`PhysicalDevice`] that [`device`] was created from
-  `pAllocateInfo->memoryTypeIndex` **must**  be less than [`PhysicalDeviceMemoryProperties::memory_type_count`] as returned by [`get_physical_device_memory_properties`] for the [`PhysicalDevice`] that [`device`] was created from
-    If the [`deviceCoherentMemory`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceCoherentMemory) feature is not enabled, `pAllocateInfo->memoryTypeIndex` **must**  not identify a memory type supporting `VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD`
-    There  **must**  be less than [`PhysicalDeviceLimits::max_memory_allocation_count`] device memory allocations currently allocated on the device

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_allocate_info`] **must**  be a valid pointer to a valid [`MemoryAllocateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_memory`] **must**  be a valid pointer to a [`DeviceMemory`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`  - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`DeviceMemory`]
- [`MemoryAllocateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        