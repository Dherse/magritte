[vkGetPhysicalDeviceMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html) - Reports memory information for the specified physical device

# C Specifications
To query memory properties, call:
```c
// Provided by VK_VERSION_1_0
void vkGetPhysicalDeviceMemoryProperties(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceMemoryProperties*           pMemoryProperties);
```

# Parameters
- [`physical_device`] is the handle to the device to query.
- [`p_memory_properties`] is a pointer to a [`PhysicalDeviceMemoryProperties`] structure in which the properties are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_memory_properties`] **must**  be a valid pointer to a [`PhysicalDeviceMemoryProperties`] structure

# Related
- [`crate::vulkan1_0`]
- [`PhysicalDevice`]
- [`PhysicalDeviceMemoryProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        