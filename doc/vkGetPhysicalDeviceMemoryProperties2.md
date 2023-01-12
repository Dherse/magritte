[vkGetPhysicalDeviceMemoryProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html) - Reports memory information for the specified physical device

# C Specifications
To query memory properties, call:
```c
// Provided by VK_VERSION_1_1
void vkGetPhysicalDeviceMemoryProperties2(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceMemoryProperties2*          pMemoryProperties);
```
or the equivalent command
```c
// Provided by VK_KHR_get_physical_device_properties2
void vkGetPhysicalDeviceMemoryProperties2KHR(
    VkPhysicalDevice                            physicalDevice,
    VkPhysicalDeviceMemoryProperties2*          pMemoryProperties);
```

# Parameters
- [`physical_device`] is the handle to the device to query.
- [`p_memory_properties`] is a pointer to a [`PhysicalDeviceMemoryProperties2`] structure in which the properties are returned.

# Description
[`get_physical_device_memory_properties2`] behaves similarly to
[`get_physical_device_memory_properties`], with the ability to return
extended information in a `pNext` chain of output structures.
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_memory_properties`] **must**  be a valid pointer to a [`PhysicalDeviceMemoryProperties2`] structure

# Related
- [`crate::vulkan1_1`]
- [`PhysicalDevice`]
- [`PhysicalDeviceMemoryProperties2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        