[VkXcbSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXcbSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Xcb surface object

# C Specifications
The [`XcbSurfaceCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_xcb_surface
typedef struct VkXcbSurfaceCreateInfoKHR {
    VkStructureType               sType;
    const void*                   pNext;
    VkXcbSurfaceCreateFlagsKHR    flags;
    xcb_connection_t*             connection;
    xcb_window_t                  window;
} VkXcbSurfaceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`connection`] is a pointer to an `xcb_connection_t` to the X server.
- [`window`] is the `xcb_window_t` for the X11 window to associate the surface with.

# Description
## Valid Usage
-  [`connection`] **must**  point to a valid X11 `xcb_connection_t`
-  [`window`] **must**  be a valid X11 `xcb_window_t`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`khr_xcb_surface`]
- [`StructureType`]
- [`XcbSurfaceCreateFlagsKHR`]
- [`create_xcb_surface_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        