[VkAccelerationStructureCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) - Bitmask specifying additional creation parameters for acceleration structure

# C Specifications
Bits which  **can**  be set in
[`AccelerationStructureCreateInfoKHR::create_flags`], specifying
additional creation parameters for acceleration structures, are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkAccelerationStructureCreateFlagBitsKHR {
    VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = 0x00000001,
  // Provided by VK_NV_ray_tracing_motion_blur
    VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV = 0x00000004,
} VkAccelerationStructureCreateFlagBitsKHR;
```

# Description
- [`VK_ACCELERATION_STRUCTURE_CREATE_FLAG_BITS_KHR`] specifies that the acceleration structure’s address  **can**  be saved and reused on a subsequent run.

# Related
- [`khr_acceleration_structure`]
- [VkAccelerationStructureCreateFlagsKHR]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        