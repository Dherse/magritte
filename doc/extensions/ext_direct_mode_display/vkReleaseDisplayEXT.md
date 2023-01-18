[vkReleaseDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html) - Release access to an acquired VkDisplayKHR

# C Specifications
To release a previously acquired display, call:
```c
// Provided by VK_EXT_direct_mode_display
VkResult vkReleaseDisplayEXT(
    VkPhysicalDevice                            physicalDevice,
    VkDisplayKHR                                display);
```

# Parameters
- [`physical_device`] The physical device the display is on.
- [`display`] The display to release control of.

# Description
## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]

## Return Codes
*   - `VK_SUCCESS`

# Related
- [`VK_EXT_direct_mode_display`]
- [`DisplayKHR`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        