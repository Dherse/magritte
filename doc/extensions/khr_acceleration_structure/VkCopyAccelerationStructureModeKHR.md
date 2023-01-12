[VkCopyAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeKHR.html) - Acceleration structure copy mode

# C Specifications
Possible values of `mode` specifying additional operations to perform
during the copy, are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkCopyAccelerationStructureModeKHR {
    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR = 0,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR = 1,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR = 2,
    VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR = 3,
  // Provided by VK_NV_ray_tracing
    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV = VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR,
  // Provided by VK_NV_ray_tracing
    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV = VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR,
} VkCopyAccelerationStructureModeKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkCopyAccelerationStructureModeKHR VkCopyAccelerationStructureModeNV;
```

# Description
- [`VK_COPY_ACCELERATION_STRUCTURE_MODE_KHR`] creates a direct copy of the acceleration structure specified in `src` into the one specified by `dst`. The `dst` acceleration structure  **must**  have been created with the same parameters as `src`. If `src` contains references to other acceleration structures, `dst` will reference the same acceleration structures.
- [`VK_COPY_ACCELERATION_STRUCTURE_MODE_KHR`] creates a more compact version of an acceleration structure `src` into `dst`. The acceleration structure `dst` **must**  have been created with a size at least as large as that returned by [`cmd_write_acceleration_structures_properties_khr`] or [`write_acceleration_structures_properties_khr`] after the build of the acceleration structure specified by `src`. If `src` contains references to other acceleration structures, `dst` will reference the same acceleration structures.
- [`VK_COPY_ACCELERATION_STRUCTURE_MODE_KHR`] serializes the acceleration structure to a semi-opaque format which can be reloaded on a compatible implementation.
- [`VK_COPY_ACCELERATION_STRUCTURE_MODE_KHR`] deserializes the semi-opaque serialization format in the buffer to the acceleration structure.

# Related
- [`khr_acceleration_structure`]
- [`nv_ray_tracing`]
- [`CopyAccelerationStructureInfoKHR`]
- [`CopyAccelerationStructureToMemoryInfoKHR`]
- [`CopyMemoryToAccelerationStructureInfoKHR`]
- [`cmd_copy_acceleration_structure_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        