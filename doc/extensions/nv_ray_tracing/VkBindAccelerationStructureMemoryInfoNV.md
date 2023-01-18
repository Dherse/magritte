[VkBindAccelerationStructureMemoryInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindAccelerationStructureMemoryInfoNV.html) - Structure specifying acceleration structure memory binding

# C Specifications
The [`BindAccelerationStructureMemoryInfoNV`] structure is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkBindAccelerationStructureMemoryInfoNV {
    VkStructureType              sType;
    const void*                  pNext;
    VkAccelerationStructureNV    accelerationStructure;
    VkDeviceMemory               memory;
    VkDeviceSize                 memoryOffset;
    uint32_t                     deviceIndexCount;
    const uint32_t*              pDeviceIndices;
} VkBindAccelerationStructureMemoryInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`acceleration_structure`] is the acceleration structure to be attached to memory.
- [`memory`] is a [`DeviceMemory`] object describing the device memory to attach.
- [`memory_offset`] is the start offset of the region of memory that is to be bound to the acceleration structure. The number of bytes returned in the [`MemoryRequirements::size`] member in [`memory`], starting from [`memory_offset`] bytes, will be bound to the specified acceleration structure.
- [`device_index_count`] is the number of elements in [`device_indices`].
- [`device_indices`] is a pointer to an array of device indices.

# Description
## Valid Usage
-  [`acceleration_structure`] **must**  not already be backed by a memory object
-  [`memory_offset`] **must**  be less than the size of [`memory`]
-  [`memory`] **must**  have been allocated using one of the memory types allowed in the `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to [`get_acceleration_structure_memory_requirements_nv`] with [`acceleration_structure`] and `type` of `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
-  [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the [`MemoryRequirements`] structure returned from a call to [`get_acceleration_structure_memory_requirements_nv`] with [`acceleration_structure`] and `type` of `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV`
-    The `size` member of the [`MemoryRequirements`] structure returned from a call to [`get_acceleration_structure_memory_requirements_nv`] with [`acceleration_structure`] and `type` of `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV` **must**  be less than or equal to the size of [`memory`] minus [`memory_offset`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`acceleration_structure`] **must**  be a valid [`AccelerationStructureNV`] handle
-  [`memory`] **must**  be a valid [`DeviceMemory`] handle
-    If [`device_index_count`] is not `0`, [`device_indices`] **must**  be a valid pointer to an array of [`device_index_count`]`uint32_t` values
-    Both of [`acceleration_structure`], and [`memory`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureNV`]
- [`DeviceMemory`]
- [`DeviceSize`]
- [`StructureType`]
- [`bind_acceleration_structure_memory_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        