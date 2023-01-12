[vkGetPhysicalDeviceProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceProperties2.html) - Returns properties of a physical device

# C Specifications
To query general properties of physical devices once enumerated, call:
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceProperties2(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceProperties2*                pProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_get_physical_device_properties2
void vkGetPhysicalDeviceProperties2KHR(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceProperties2*                pProperties);
```

# Parameters
- [`physical_device`] is the handle to the physical device whose properties will be queried.
- [`p_properties`] is a pointer to a [`PhysicalDeviceProperties2`] structure in which properties are returned.

# Description
Each structure in [`p_properties`] and its `pNext` chain contains
members corresponding to implementation-dependent properties, behaviors, or
limits.
[`get_physical_device_properties2`] fills in each member to specify the
corresponding value for the implementation.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_properties`] **must**  be a valid pointer to a [`PhysicalDeviceProperties2`] structure

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDevice`]
- [`PhysicalDeviceProperties2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        