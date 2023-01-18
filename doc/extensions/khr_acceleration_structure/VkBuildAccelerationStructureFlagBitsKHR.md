[VkBuildAccelerationStructureFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html) - Bitmask specifying additional parameters for acceleration structure builds

# C Specifications
Bits which  **can**  be set in
[`AccelerationStructureBuildGeometryInfoKHR::flags`]
or
[`AccelerationStructureInfoNV::flags`]
specifying additional parameters for acceleration structure builds, are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkBuildAccelerationStructureFlagBitsKHR {
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = 0x00000001,
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = 0x00000002,
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = 0x00000004,
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = 0x00000008,
    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = 0x00000010,
  // Provided by VK_NV_ray_tracing_motion_blur
    VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV = 0x00000020,
  // Provided by VK_NV_ray_tracing
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV = VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV = VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV = VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV = VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV = VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR,
} VkBuildAccelerationStructureFlagBitsKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkBuildAccelerationStructureFlagBitsKHR VkBuildAccelerationStructureFlagBitsNV;
```

# Description
- [`ALLOW_UPDATE`] indicates     that the specified acceleration structure  **can**  be updated with     a `mode` of `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` in     [`AccelerationStructureBuildGeometryInfoKHR`] or     an `update` of [`TRUE`] in     [`cmd_build_acceleration_structure_nv`]     .
- [`ALLOW_COMPACTION`] indicates that the specified acceleration structure  **can**  act as the source for a copy acceleration structure command with `mode` of `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` to produce a compacted acceleration structure.
- [`PREFER_FAST_TRACE`] indicates that the given acceleration structure build  **should**  prioritize trace performance over build time.
- [`PREFER_FAST_BUILD`] indicates that the given acceleration structure build  **should**  prioritize build time over trace performance.
- [`LOW_MEMORY`] indicates that this acceleration structure  **should**  minimize the size of the scratch memory and the final result acceleration structure, potentially at the expense of build time or trace performance.

# Related
- [`VK_KHR_acceleration_structure`]
- [`VK_NV_ray_tracing`]
- [`BuildAccelerationStructureFlagsKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        