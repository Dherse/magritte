[VkMemoryAllocateFlagsInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html) - Structure controlling how many instances of memory will be allocated

# C Specifications
If the [`p_next`] chain of [`MemoryAllocateInfo`] includes a
[`MemoryAllocateFlagsInfo`] structure, then that structure includes
flags and a device mask controlling how many instances of the memory will be
allocated.The [`MemoryAllocateFlagsInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkMemoryAllocateFlagsInfo {
    VkStructureType          sType;
    const void*              pNext;
    VkMemoryAllocateFlags    flags;
    uint32_t                 deviceMask;
} VkMemoryAllocateFlagsInfo;
```
or the equivalent
```c
// Provided by VK_KHR_device_group
typedef VkMemoryAllocateFlagsInfo VkMemoryAllocateFlagsInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`MemoryAllocateFlagBits`] controlling the allocation.
- [`device_mask`] is a mask of physical devices in the logical device, indicating that memory  **must**  be allocated on each device in the mask, if `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set in [`flags`].

# Description
If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is not set, the number of
instances allocated depends on whether
`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` is set in the memory heap.
If `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` is set, then memory is allocated
for every physical device in the logical device (as if [`device_mask`] has
bits set for all device indices).
If `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` is not set, then a single
instance of memory is allocated (as if [`device_mask`] is set to one).On some implementations, allocations from a multi-instance heap  **may**  consume
memory on all physical devices even if the [`device_mask`] excludes some
devices.
If [`PhysicalDeviceGroupProperties::subset_allocation`] is
[`TRUE`], then memory is only consumed for the devices in the device
mask.
## Valid Usage
-    If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set, [`device_mask`] **must**  be a valid device mask
-    If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set, [`device_mask`] **must**  not be zero

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`
-  [`flags`] **must**  be a valid combination of [`MemoryAllocateFlagBits`] values

# Related
- [`crate::vulkan1_1`]
- [`MemoryAllocateFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        