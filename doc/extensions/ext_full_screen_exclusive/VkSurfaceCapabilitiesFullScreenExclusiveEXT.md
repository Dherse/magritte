[VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html) - Structure describing full screen exclusive capabilities of a surface

# C Specifications
The [`SurfaceCapabilitiesFullScreenExclusiveEXT`] structure is defined
as:
```c
// Provided by VK_EXT_full_screen_exclusive
typedef struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           fullScreenExclusiveSupported;
} VkSurfaceCapabilitiesFullScreenExclusiveEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- `fullScreenExclusiveControlSupported` is a boolean describing whether the surface is able to make use of exclusive full-screen access.

# Description
This structure  **can**  be included in the [`p_next`] chain of
[`SurfaceCapabilities2KHR`] to determine support for exclusive
full-screen access.
If [`full_screen_exclusive_supported`] is [`FALSE`], it indicates that
exclusive full-screen access is not obtainable for this surface.Applications  **must**  not attempt to create swapchains with
`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT` set if
[`full_screen_exclusive_supported`] is [`FALSE`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`

# Related
- [`VK_EXT_full_screen_exclusive`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        