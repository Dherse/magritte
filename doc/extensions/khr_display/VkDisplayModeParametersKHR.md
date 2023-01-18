[VkDisplayModeParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html) - Structure describing display parameters associated with a display mode

# C Specifications
The [`DisplayModeParametersKHR`] structure is defined as:
```c
// Provided by VK_KHR_display
typedef struct VkDisplayModeParametersKHR {
    VkExtent2D    visibleRegion;
    uint32_t      refreshRate;
} VkDisplayModeParametersKHR;
```

# Members
- [`visible_region`] is the 2D extents of the visible region.
- [`refresh_rate`] is a `uint32_t` that is the number of times the display is refreshed each second multiplied by 1000.

# Description
## Valid Usage
-    The `width` member of [`visible_region`] **must**  be greater than `0`
-    The `height` member of [`visible_region`] **must**  be greater than `0`
-  [`refresh_rate`] **must**  be greater than `0`

# Related
- [`VK_KHR_display`]
- [`DisplayModeCreateInfoKHR`]
- [`DisplayModePropertiesKHR`]
- [`Extent2D`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        