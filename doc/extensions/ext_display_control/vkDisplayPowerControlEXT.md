[vkDisplayPowerControlEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html) - Set the power state of a display

# C Specifications
To set the power state of a display, call:
```c
// Provided by VK_EXT_display_control
VkResult vkDisplayPowerControlEXT(
    VkDevice                                    device,
    VkDisplayKHR                                display,
    const VkDisplayPowerInfoEXT*                pDisplayPowerInfo);
```

# Parameters
- [`device`] is a logical device associated with [`display`].
- [`display`] is the display whose power state is modified.
- [`p_display_power_info`] is a pointer to a [`DisplayPowerInfoEXT`] structure specifying the new power state of [`display`].

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`p_display_power_info`] **must**  be a valid pointer to a valid [`DisplayPowerInfoEXT`] structure
-    Both of [`device`], and [`display`] **must**  have been created, allocated, or retrieved from the same [`PhysicalDevice`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_EXT_display_control`]
- [`Device`]
- [`DisplayKHR`]
- [`DisplayPowerInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        