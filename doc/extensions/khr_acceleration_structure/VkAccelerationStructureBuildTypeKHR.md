[VkAccelerationStructureBuildTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html) - Acceleration structure build type

# C Specifications
Possible values of `buildType` in
[`get_acceleration_structure_build_sizes_khr`] are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkAccelerationStructureBuildTypeKHR {
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR = 0,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR = 1,
    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR = 2,
} VkAccelerationStructureBuildTypeKHR;
```

# Description
- [`VK_ACCELERATION_STRUCTURE_BUILD_TYPE_KHR`] requests the memory requirement for operations performed by the host.
- [`VK_ACCELERATION_STRUCTURE_BUILD_TYPE_KHR`] requests the memory requirement for operations performed by the device.
- [`VK_ACCELERATION_STRUCTURE_BUILD_TYPE_KHR`] requests the memory requirement for operations performed by either the host, or the device.

# Related
- [`khr_acceleration_structure`]
- [`get_acceleration_structure_build_sizes_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        