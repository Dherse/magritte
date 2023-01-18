[VkDisplayEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayEventTypeEXT.html) - Events that can occur on a display object

# C Specifications
Possible values of [`DisplayEventInfoEXT::display_event`],
specifying when a fence will be signaled, are:
```c
// Provided by VK_EXT_display_control
typedef enum VkDisplayEventTypeEXT {
    VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT = 0,
} VkDisplayEventTypeEXT;
```

# Description
- [`FIRST_PIXEL_OUT`] specifies that the fence is signaled when the first pixel of the next display refresh cycle leaves the display engine for the display.

# Related
- [`VK_EXT_display_control`]
- [`DisplayEventInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        