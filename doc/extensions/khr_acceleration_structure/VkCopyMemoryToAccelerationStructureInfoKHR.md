[VkCopyMemoryToAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html) - Parameters for deserializing an acceleration structure

# C Specifications
The [`CopyMemoryToAccelerationStructureInfoKHR`] structure is defined
as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkCopyMemoryToAccelerationStructureInfoKHR {
    VkStructureType                       sType;
    const void*                           pNext;
    VkDeviceOrHostAddressConstKHR         src;
    VkAccelerationStructureKHR            dst;
    VkCopyAccelerationStructureModeKHR    mode;
} VkCopyMemoryToAccelerationStructureInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src`] is the device or host address to memory containing the source data for the copy.
- [`dst`] is the target acceleration structure for the copy.
- [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to perform during the copy.

# Description
## Valid Usage
-    The source memory pointed to by [`src`] **must**  contain data previously serialized using [`cmd_copy_acceleration_structure_to_memory_khr`], potentially modified to relocate acceleration structure references as described in that command
-  [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR`
-    The data in [`src`] **must**  have a format compatible with the destination physical device as returned by [`get_device_acceleration_structure_compatibility_khr`]
-  [`dst`] **must**  have been created with a `size` greater than or equal to that used to serialize the data in [`src`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`dst`] **must**  be a valid [`AccelerationStructureKHR`] handle
-  [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureKHR`]
- [`CopyAccelerationStructureModeKHR`]
- [`DeviceOrHostAddressConstKHR`]
- [`StructureType`]
- [`cmd_copy_memory_to_acceleration_structure_khr`]
- [`copy_memory_to_acceleration_structure_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        