[vkCmdCopyAccelerationStructureToMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html) - Copy an acceleration structure to device memory

# C Specifications
To copy an acceleration structure to device memory call:
```c
// Provided by VK_KHR_acceleration_structure
void vkCmdCopyAccelerationStructureToMemoryKHR(
    VkCommandBuffer                             commandBuffer,
    const VkCopyAccelerationStructureToMemoryInfoKHR* pInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`p_info`] is an a pointer to a [`CopyAccelerationStructureToMemoryInfoKHR`] structure defining the copy operation.

# Description
Accesses to `pInfo->src` **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`.
Accesses to the buffer indicated by `pInfo->dst.deviceAddress` **must**  be
synchronized with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` pipeline stage
and an access type of `VK_ACCESS_TRANSFER_WRITE_BIT`.This command produces the same results as
[`copy_acceleration_structure_to_memory_khr`], but writes its result to a
device address, and is executed on the device rather than the host.
The output  **may**  not necessarily be bit-for-bit identical, but it can be
equally used by either [`cmd_copy_memory_to_acceleration_structure_khr`] or
[`copy_memory_to_acceleration_structure_khr`].The defined header structure for the serialized data consists of:
- `VK_UUID_SIZE` bytes of data matching [`PhysicalDeviceIdProperties::driver_uuid`]
- `VK_UUID_SIZE` bytes of data identifying the compatibility for comparison using [`get_device_acceleration_structure_compatibility_khr`]
- A 64-bit integer of the total size matching the value queried using `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
- A 64-bit integer of the deserialized size to be passed in to [`AccelerationStructureCreateInfoKHR::size`]
- A 64-bit integer of the count of the number of acceleration structure handles following. This will be zero for a bottom-level acceleration structure. For top-level acceleration structures this number is implementation-dependent; the number of and ordering of the handles may not match the instance descriptions which were used to build the acceleration structure.
The corresponding handles matching the values returned by
[`get_acceleration_structure_device_address_khr`]
or
[`get_acceleration_structure_handle_nv`]
are tightly packed in the buffer following the count.
The application is expected to store a mapping between those handles and the
original application-generated bottom-level acceleration structures to
provide when deserializing.
The serialized data is written to the buffer (or read from the buffer)
according to the host endianness.
## Valid Usage
-  `pInfo->dst.deviceAddress` **must**  be a valid device address for a buffer bound to device memory
-  `pInfo->dst.deviceAddress` **must**  be aligned to `256` bytes
-    If the buffer pointed to by `pInfo->dst.deviceAddress` is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    The `buffer` used to create `pInfo->src` **must**  be bound to device memory

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`CopyAccelerationStructureToMemoryInfoKHR`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_acceleration_structure`]
- [`CommandBuffer`]
- [`CopyAccelerationStructureToMemoryInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        