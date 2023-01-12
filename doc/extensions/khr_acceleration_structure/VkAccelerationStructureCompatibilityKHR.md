[VkAccelerationStructureCompatibilityKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html) - Acceleration structure compatibility

# C Specifications
Possible values of `pCompatibility` returned by
[`get_device_acceleration_structure_compatibility_khr`] are:
```c
// Provided by VK_KHR_acceleration_structure
typedef enum VkAccelerationStructureCompatibilityKHR {
    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR = 0,
    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR = 1,
} VkAccelerationStructureCompatibilityKHR;
```

# Description
- [`VK_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR`] if the `pVersionData` version acceleration structure is compatible with `device`.
- [`VK_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR`] if the `pVersionData` version acceleration structure is not compatible with `device`.

# Related
- [`khr_acceleration_structure`]
- [`get_device_acceleration_structure_compatibility_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        