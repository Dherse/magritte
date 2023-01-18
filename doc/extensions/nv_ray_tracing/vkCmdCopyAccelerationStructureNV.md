[vkCmdCopyAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html) - Copy an acceleration structure

# C Specifications
To copy an acceleration structure call:
```c
// Provided by VK_NV_ray_tracing
void vkCmdCopyAccelerationStructureNV(
    VkCommandBuffer                             commandBuffer,
    VkAccelerationStructureNV                   dst,
    VkAccelerationStructureNV                   src,
    VkCopyAccelerationStructureModeKHR          mode);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`dst`] is the target acceleration structure for the copy.
- [`src`] is the source acceleration structure for the copy.
- [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to perform during the copy.

# Description
Accesses to [`src`] and [`dst`] **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR` or
`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR` as appropriate.
## Valid Usage
-  [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` or `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR`
-    The source acceleration structure [`src`] **must**  have been constructed prior to the execution of this command
-    If [`mode`] is `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR`, [`src`] **must**  have been constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` in the build
-    The `buffer` used to create [`src`] **must**  be bound to device memory
-    The `buffer` used to create [`dst`] **must**  be bound to device memory

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`dst`] **must**  be a valid [`AccelerationStructureNV`] handle
-  [`src`] **must**  be a valid [`AccelerationStructureNV`] handle
-  [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Each of [`command_buffer`], [`dst`], and [`src`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureNV`]
- [`CommandBuffer`]
- [`CopyAccelerationStructureModeKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        