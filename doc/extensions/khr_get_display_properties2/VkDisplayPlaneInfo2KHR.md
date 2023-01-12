[VkDisplayPlaneInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneInfo2KHR.html) - Structure defining the intended configuration of a display plane

# C Specifications
The [`DisplayPlaneInfo2KHR`] structure is defined as:
```c
// Provided by VK_KHR_get_display_properties2
typedef struct VkDisplayPlaneInfo2KHR {
    VkStructureType     sType;
    const void*         pNext;
    VkDisplayModeKHR    mode;
    uint32_t            planeIndex;
} VkDisplayPlaneInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`mode`] is the display mode the application intends to program when using the specified plane.

# Description
- [`plane_index`] is the plane which the application intends to use with the display.
The members of [`DisplayPlaneInfo2KHR`] correspond to the arguments to
[`get_display_plane_capabilities_khr`], with [`s_type`] and [`p_next`]
added for extensibility.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`mode`] **must**  be a valid [`DisplayModeKHR`] handle

## Host Synchronization
- Host access to [`mode`] **must**  be externally synchronized

# Related
- [`khr_get_display_properties2`]
- [`DisplayModeKHR`]
- [`StructureType`]
- [`get_display_plane_capabilities2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        