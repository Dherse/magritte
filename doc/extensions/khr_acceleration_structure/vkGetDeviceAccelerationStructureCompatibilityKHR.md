[vkGetDeviceAccelerationStructureCompatibilityKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html) - Check if a serialized acceleration structure is compatible with the current device

# C Specifications
To check if a serialized acceleration structure is compatible with the
current device call:
```c
// Provided by VK_KHR_acceleration_structure
void vkGetDeviceAccelerationStructureCompatibilityKHR(
    VkDevice                                    device,
    const VkAccelerationStructureVersionInfoKHR* pVersionInfo,
    VkAccelerationStructureCompatibilityKHR*    pCompatibility);
```

# Parameters
- [`device`] is the device to check the version against.
- [`p_version_info`] is a pointer to a [`AccelerationStructureVersionInfoKHR`] structure specifying version information to check against the device.
- [`p_compatibility`] is a pointer to a [`AccelerationStructureCompatibilityKHR`] value in which compatibility information is returned.

# Description
## Valid Usage
-    The [`rayTracingPipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipeline) or [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayQuery) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_version_info`] **must**  be a valid pointer to a valid [`AccelerationStructureVersionInfoKHR`] structure
-  [`p_compatibility`] **must**  be a valid pointer to a [`AccelerationStructureCompatibilityKHR`] value

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureCompatibilityKHR`]
- [`AccelerationStructureVersionInfoKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        