[vkCmdCopyMemoryToAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html) - Copy device memory to an acceleration structure

# C Specifications
To copy device memory to an acceleration structure call:
```c
// Provided by VK_KHR_acceleration_structure
void vkCmdCopyMemoryToAccelerationStructureKHR(
    VkCommandBuffer                             commandBuffer,
    const VkCopyMemoryToAccelerationStructureInfoKHR* pInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`p_info`] is a pointer to a [`CopyMemoryToAccelerationStructureInfoKHR`] structure defining the copy operation.

# Description
Accesses to `pInfo->dst` **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`.
Accesses to the buffer indicated by `pInfo->src.deviceAddress` **must**  be
synchronized with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` pipeline stage
and an access type of `VK_ACCESS_TRANSFER_READ_BIT`.This command can accept acceleration structures produced by either
[`cmd_copy_acceleration_structure_to_memory_khr`] or
[`copy_acceleration_structure_to_memory_khr`].The structure provided as input to deserialize is as described in
[`cmd_copy_acceleration_structure_to_memory_khr`], with any acceleration
structure handles filled in with the newly-queried handles to bottom level
acceleration structures created before deserialization.
These do not need to be built at deserialize time, but  **must**  be created.
## Valid Usage
-  `pInfo->src.deviceAddress` **must**  be a valid device address for a buffer bound to device memory
-  `pInfo->src.deviceAddress` **must**  be aligned to `256` bytes
-    If the buffer pointed to by `pInfo->src.deviceAddress` is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    The `buffer` used to create `pInfo->dst` **must**  be bound to device memory

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`CopyMemoryToAccelerationStructureInfoKHR`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_KHR_acceleration_structure`]
- [`CommandBuffer`]
- [`CopyMemoryToAccelerationStructureInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        