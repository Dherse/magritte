[vkGetPhysicalDeviceFeatures2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFeatures2.html) - Reports capabilities of a physical device

# C Specifications
To query supported features defined by the core or extensions, call:
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceFeatures2(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceFeatures2*                  pFeatures);
```
or the equivalent command
```c
// Provided by VK_KHR_get_physical_device_properties2
void vkGetPhysicalDeviceFeatures2KHR(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceFeatures2*                  pFeatures);
```

# Parameters
- [`physical_device`] is the physical device from which to query the supported features.
- [`p_features`] is a pointer to a [`PhysicalDeviceFeatures2`] structure in which the physical device features are returned.

# Description
Each structure in [`p_features`] and its `pNext` chain contains members
corresponding to fine-grained features.
[`get_physical_device_features2`] writes each member to a boolean value
indicating whether that feature is supported.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_features`] **must**  be a valid pointer to a [`PhysicalDeviceFeatures2`] structure

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDevice`]
- [`PhysicalDeviceFeatures2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        