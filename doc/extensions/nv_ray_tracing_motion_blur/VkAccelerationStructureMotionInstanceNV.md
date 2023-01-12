[VkAccelerationStructureMotionInstanceNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureMotionInstanceNV.html) - Structure specifying a single acceleration structure motion instance for building into an acceleration structure geometry

# C Specifications
*Acceleration structure motion instances* **can**  be built into top-level
acceleration structures.
Each acceleration structure instance is a separate entry in the top-level
acceleration structure which includes all the geometry of a bottom-level
acceleration structure at a transformed location including a type of motion
and parameters to determine the motion of the instance over time.An acceleration structure motion instance is defined by the structure:
```c
// Provided by VK_NV_ray_tracing_motion_blur
typedef struct VkAccelerationStructureMotionInstanceNV {
    VkAccelerationStructureMotionInstanceTypeNV     type;
    VkAccelerationStructureMotionInstanceFlagsNV    flags;
    VkAccelerationStructureMotionInstanceDataNV     data;
} VkAccelerationStructureMotionInstanceNV;
```

# Members
- [`type_`] is a [`AccelerationStructureMotionInstanceTypeNV`] enumerant identifying which type of motion instance this is and which type of the union is valid.
- [`flags`] is currently unused, but is required to keep natural alignment of [`data`].
- [`data`] is a [`AccelerationStructureMotionInstanceDataNV`] containing motion instance data for this instance.

# Description
## Valid Usage (Implicit)
-  [`type_`] **must**  be a valid [`AccelerationStructureMotionInstanceTypeNV`] value
-  [`flags`] **must**  be `0`
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV`, the `staticInstance` member of [`data`] **must**  be a valid [`AccelerationStructureInstanceKHR`] structure
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV`, the `matrixMotionInstance` member of [`data`] **must**  be a valid [`AccelerationStructureMatrixMotionInstanceNV`] structure
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV`, the `srtMotionInstance` member of [`data`] **must**  be a valid [`AccelerationStructureSrtMotionInstanceNV`] structure

# Related
- [`nv_ray_tracing_motion_blur`]
- [`AccelerationStructureMotionInstanceDataNV`]
- [`AccelerationStructureMotionInstanceFlagsNV`]
- [`AccelerationStructureMotionInstanceTypeNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        