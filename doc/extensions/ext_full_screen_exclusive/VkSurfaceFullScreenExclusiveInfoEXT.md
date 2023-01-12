[VkSurfaceFullScreenExclusiveInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html) - Structure specifying the preferred full-screen transition behavior

# C Specifications
If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
[`SurfaceFullScreenExclusiveInfoEXT`] structure, then that structure
specifies the application’s preferred full-screen transition behavior.The [`SurfaceFullScreenExclusiveInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_full_screen_exclusive
typedef struct VkSurfaceFullScreenExclusiveInfoEXT {
    VkStructureType             sType;
    void*                       pNext;
    VkFullScreenExclusiveEXT    fullScreenExclusive;
} VkSurfaceFullScreenExclusiveInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`full_screen_exclusive`] is a [`FullScreenExclusiveEXT`] value specifying the preferred full-screen transition behavior.

# Description
If this structure is not present, [`full_screen_exclusive`] is considered to
be `VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT`
-  [`full_screen_exclusive`] **must**  be a valid [`FullScreenExclusiveEXT`] value

# Related
- [`ext_full_screen_exclusive`]
- [`FullScreenExclusiveEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        