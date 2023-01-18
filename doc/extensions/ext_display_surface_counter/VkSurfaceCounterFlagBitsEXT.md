[VkSurfaceCounterFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) - Surface-relative counter types

# C Specifications
Bits which  **can**  be set in
[`SurfaceCapabilities2EXT::supported_surface_counters`], indicating
supported surface counter types, are:
```c
// Provided by VK_EXT_display_surface_counter
typedef enum VkSurfaceCounterFlagBitsEXT {
    VK_SURFACE_COUNTER_VBLANK_BIT_EXT = 0x00000001,
    VK_SURFACE_COUNTER_VBLANK_EXT = VK_SURFACE_COUNTER_VBLANK_BIT_EXT,
} VkSurfaceCounterFlagBitsEXT;
```

# Description
- [`VBLANK`] specifies a counter incrementing once every time a vertical blanking period occurs on the display associated with the surface.

# Related
- [`VK_EXT_display_surface_counter`]
- [`SurfaceCounterFlagsEXT`]
- [`get_swapchain_counter_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        