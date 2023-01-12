[VkDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html) - Structure describing an available display device

# C Specifications
The [`DisplayPropertiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_display
typedef struct VkDisplayPropertiesKHR {
    VkDisplayKHR                  display;
    const char*                   displayName;
    VkExtent2D                    physicalDimensions;
    VkExtent2D                    physicalResolution;
    VkSurfaceTransformFlagsKHR    supportedTransforms;
    VkBool32                      planeReorderPossible;
    VkBool32                      persistentContent;
} VkDisplayPropertiesKHR;
```

# Members
- [`display`] is a handle that is used to refer to the display described here. This handle will be valid for the lifetime of the Vulkan instance.
- [`display_name`] is `NULL` or a pointer to a null-terminated UTF-8 string containing the name of the display. Generally, this will be the name provided by the display’s EDID. If `NULL`, no suitable name is available. If not `NULL`, the string pointed to  **must**  remain accessible and unmodified as long as [`display`] is valid.
- [`physical_dimensions`] describes the physical width and height of the visible portion of the display, in millimeters.
- [`physical_resolution`] describes the physical, native, or preferred resolution of the display.

# Description
- [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] describing which transforms are supported by this display.
- [`plane_reorder_possible`] tells whether the planes on this display  **can**  have their z order changed. If this is `VK_TRUE`, the application  **can**  re-arrange the planes on this display in any order relative to each other.
- [`persistent_content`] tells whether the display supports self-refresh/internal buffering. If this is true, the application  **can**  submit persistent present operations on swapchains created against this display.

# Related
- [`khr_display`]
- [`Bool32`]
- [`DisplayKHR`]
- [`DisplayProperties2KHR`]
- [`Extent2D`]
- [VkSurfaceTransformFlagsKHR]()
- [`get_physical_device_display_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        