[VkCopyAccelerationStructureToMemoryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html) - Parameters for serializing an acceleration structure

# C Specifications
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkCopyAccelerationStructureToMemoryInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkAccelerationStructureKHR            src;
    VkDeviceOrHostAddressKHR              dst;
    VkCopyAccelerationStructureModeKHR    mode;
} VkCopyAccelerationStructureToMemoryInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src`] is the source acceleration structure for the copy
- [`dst`] is the device or host address to memory which is the target for the copy
- [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to perform during the copy.

# Description
## Valid Usage
-    The source acceleration structure [`src`] **must**  have been constructed prior to the execution of this command
-    The memory pointed to by [`dst`] **must**  be at least as large as the serialization size of [`src`], as reported by [`write_acceleration_structures_properties_khr`] or [`cmd_write_acceleration_structures_properties_khr`] with a query type of `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
-  [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`src`] **must**  be a valid [`AccelerationStructureKHR`] handle
-  [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureKHR`]
- [`CopyAccelerationStructureModeKHR`]
- [`DeviceOrHostAddressKHR`]
- [`StructureType`]
- [`cmd_copy_acceleration_structure_to_memory_khr`]
- [`copy_acceleration_structure_to_memory_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        