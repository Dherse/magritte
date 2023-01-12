[vkGetPhysicalDeviceProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties.html) - Returns properties of a physical device

# C Specifications
To query general properties of physical devices once enumerated, call:
```c
// Provided by VK_VERSION_1_0
void vkGetPhysicalDeviceProperties(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceProperties*                 pProperties);
```

# Parameters
- [`physical_device`] is the handle to the physical device whose properties will be queried.
- [`p_properties`] is a pointer to a [`PhysicalDeviceProperties`] structure in which properties are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_properties`] **must**  be a valid pointer to a [`PhysicalDeviceProperties`] structure

# Related
- [`crate::vulkan1_0`]
- [`PhysicalDevice`]
- [`PhysicalDeviceProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        