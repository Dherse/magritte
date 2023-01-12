[VkXlibSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXlibSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Xlib surface object

# C Specifications
The [`XlibSurfaceCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_xlib_surface
typedef struct VkXlibSurfaceCreateInfoKHR {
    VkStructureType                sType;
    const void*                    pNext;
    VkXlibSurfaceCreateFlagsKHR    flags;
    Display*                       dpy;
    Window                         window;
} VkXlibSurfaceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`dpy`] is a pointer to an Xlib `Display` connection to the X server.
- [`window`] is an Xlib `Window` to associate the surface with.

# Description
## Valid Usage
-  [`dpy`] **must**  point to a valid Xlib `Display`
-  [`window`] **must**  be a valid Xlib `Window`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`khr_xlib_surface`]
- [`StructureType`]
- [`XlibSurfaceCreateFlagsKHR`]
- [`create_xlib_surface_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        