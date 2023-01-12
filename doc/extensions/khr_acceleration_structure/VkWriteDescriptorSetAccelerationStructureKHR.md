[VkWriteDescriptorSetAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html) - Structure specifying acceleration structure descriptor information

# C Specifications
The [`WriteDescriptorSetAccelerationStructureKHR`] structure is defined
as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkWriteDescriptorSetAccelerationStructureKHR {
    VkStructureType                      sType;
    const void*                          pNext;
    uint32_t                             accelerationStructureCount;
    const VkAccelerationStructureKHR*    pAccelerationStructures;
} VkWriteDescriptorSetAccelerationStructureKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`acceleration_structure_count`] is the number of elements in [`acceleration_structures`].
- [`acceleration_structures`] is a pointer to an array of [`AccelerationStructureKHR`] structures specifying the acceleration structures to update.

# Description
## Valid Usage
-  [`acceleration_structure_count`] **must**  be equal to `descriptorCount` in the extended structure
-    Each acceleration structure in [`acceleration_structures`] **must**  have been created with a `type` of `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR` or `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
-    If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) feature is not enabled, each element of [`acceleration_structures`] **must**  not be [`crate::Handle::null`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR`
-  [`acceleration_structures`] **must**  be a valid pointer to an array of [`acceleration_structure_count`] valid or [`crate::Handle::null`][`AccelerationStructureKHR`] handles
-  [`acceleration_structure_count`] **must**  be greater than `0`

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        