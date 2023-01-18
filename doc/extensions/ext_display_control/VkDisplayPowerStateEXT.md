[VkDisplayPowerStateEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerStateEXT.html) - Possible power states for a display

# C Specifications
Possible values of [`DisplayPowerInfoEXT::power_state`], specifying
the new power state of a display, are:
```c
// Provided by VK_EXT_display_control
typedef enum VkDisplayPowerStateEXT {
    VK_DISPLAY_POWER_STATE_OFF_EXT = 0,
    VK_DISPLAY_POWER_STATE_SUSPEND_EXT = 1,
    VK_DISPLAY_POWER_STATE_ON_EXT = 2,
} VkDisplayPowerStateEXT;
```

# Description
- [`OFF`] specifies that the display is powered down.
- [`SUSPEND`] specifies that the display is put into a low power mode, from which it  **may**  be able to transition back to [`ON`] more quickly than if it were in [`OFF`]. This state  **may**  be the same as [`OFF`].
- [`ON`] specifies that the display is powered on.

# Related
- [`VK_EXT_display_control`]
- [`DisplayPowerInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        