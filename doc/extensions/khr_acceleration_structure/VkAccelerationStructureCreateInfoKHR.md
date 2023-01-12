[VkAccelerationStructureCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html) - Structure specifying the parameters of a newly created acceleration structure object

# C Specifications
The [`AccelerationStructureCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureCreateInfoKHR {
    VkStructureType                          sType;
    const void*                              pNext;
    VkAccelerationStructureCreateFlagsKHR    createFlags;
    VkBuffer                                 buffer;
    VkDeviceSize                             offset;
    VkDeviceSize                             size;
    VkAccelerationStructureTypeKHR           type;
    VkDeviceAddress                          deviceAddress;
} VkAccelerationStructureCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`create_flags`] is a bitmask of [`AccelerationStructureCreateFlagBitsKHR`] specifying additional creation parameters of the acceleration structure.
- [`buffer`] is the buffer on which the acceleration structure will be stored.
- [`offset`] is an offset in bytes from the base address of the buffer at which the acceleration structure will be stored, and  **must**  be a multiple of `256`.
- [`size`] is the size required for the acceleration structure.
- [`type_`] is a [`AccelerationStructureTypeKHR`] value specifying the type of acceleration structure that will be created.
- [`device_address`] is the device address requested for the acceleration structure if the [`accelerationStructureCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureCaptureReplay) feature is being used.

# Description
If [`device_address`] is zero, no specific address is requested.If [`device_address`] is not zero, [`device_address`] **must**  be an address
retrieved from an identically created acceleration structure on the same
implementation.
The acceleration structure  **must**  also be placed on an identically created
[`buffer`] and at the same [`offset`].Applications  **should**  avoid creating acceleration structures with
application-provided addresses and implementation-provided addresses in the
same process, to reduce the likelihood of
`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR` errors.Applications  **should**  create an acceleration structure with a specific
[`AccelerationStructureTypeKHR`] other than
`VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`.If the acceleration structure will be the target of a build operation, the
required size for an acceleration structure  **can**  be queried with
[`get_acceleration_structure_build_sizes_khr`].
If the acceleration structure is going to be the target of a compacting
copy, [`cmd_write_acceleration_structures_properties_khr`] or
[`write_acceleration_structures_properties_khr`] **can**  be used to obtain the
compacted size required.If the acceleration structure will be the target of a build operation with
`VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` it  **must**  include
`VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` in `flags` and
include [`AccelerationStructureMotionInfoNV`] as an extension structure
in [`p_next`] with the number of instances as metadata for the object.
## Valid Usage
-    If [`device_address`] is not zero, [`create_flags`] **must**  include `VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`
-    If [`create_flags`] includes `VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`, [`PhysicalDeviceAccelerationStructureFeaturesKHR::acceleration_structure_capture_replay`] **must**  be `VK_TRUE`
-  [`buffer`] **must**  have been created with a `usage` value containing `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR`
-  [`buffer`] **must**  not have been created with `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT`
-    The sum of [`offset`] and [`size`] **must**  be less than the size of [`buffer`]
-  [`offset`] **must**  be a multiple of `256` bytes
-       If `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` is set in    `flags` and [`type_`] is    `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, one member of the [`p_next`] chain  **must**  be a pointer to a valid instance of [`AccelerationStructureMotionInfoNV`]
-    If any geometry includes [`AccelerationStructureGeometryMotionTrianglesDataNV`] then `flags` **must**  contain `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`AccelerationStructureMotionInfoNV`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`create_flags`] **must**  be a valid combination of [`AccelerationStructureCreateFlagBitsKHR`] values
-  [`buffer`] **must**  be a valid [`Buffer`] handle
-  [`type_`] **must**  be a valid [`AccelerationStructureTypeKHR`] value

# Related
- [`khr_acceleration_structure`]
- [VkAccelerationStructureCreateFlagsKHR]()
- [`AccelerationStructureTypeKHR`]
- [`Buffer`]
- [`DeviceAddress`]
- [`DeviceSize`]
- [`StructureType`]
- [`create_acceleration_structure_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        