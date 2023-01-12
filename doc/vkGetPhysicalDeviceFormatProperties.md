[vkGetPhysicalDeviceFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html) - Lists physical device's format capabilities

# C Specifications
To query supported format features which are properties of the physical
device, call:
```c
// Provided by VK_VERSION_1_0
void vkGetPhysicalDeviceFormatProperties(
    VkPhysicalDevice                            physicalDevice,
    VkFormat                                    format,
    VkFormatProperties*                         pFormatProperties);
```

# Parameters
- [`physical_device`] is the physical device from which to query the format properties.
- [`format`] is the format whose properties are queried.
- [`p_format_properties`] is a pointer to a [`FormatProperties`] structure in which physical device properties for [`format`] are returned.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`format`] **must**  be a valid [`Format`] value
-  [`p_format_properties`] **must**  be a valid pointer to a [`FormatProperties`] structure

# Related
- [`crate::vulkan1_0`]
- [`Format`]
- [`FormatProperties`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        