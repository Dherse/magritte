[vkGetPhysicalDeviceFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures.html) - Reports capabilities of a physical device

# C Specifications
To query supported features, call:
```c
// Provided by VK_VERSION_1_0
void vkGetPhysicalDeviceFeatures(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceFeatures*                   pFeatures);
```

# Parameters
- [`physical_device`] is the physical device from which to query the supported features.
- [`p_features`] is a pointer to a [`PhysicalDeviceFeatures`] structure in which the physical device features are returned. For each feature, a value of [`TRUE`] specifies that the feature is supported on this physical device, and [`FALSE`] specifies that the feature is not supported.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_features`] **must**  be a valid pointer to a [`PhysicalDeviceFeatures`] structure

# Related
- [`crate::vulkan1_0`]
- [`PhysicalDevice`]
- [`PhysicalDeviceFeatures`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        