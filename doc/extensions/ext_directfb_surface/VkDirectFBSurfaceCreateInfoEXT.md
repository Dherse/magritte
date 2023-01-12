[VkDirectFBSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDirectFBSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created DirectFB surface object

# C Specifications
The [`DirectFBSurfaceCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_directfb_surface
typedef struct VkDirectFBSurfaceCreateInfoEXT {
    VkStructureType                    sType;
    const void*                        pNext;
    VkDirectFBSurfaceCreateFlagsEXT    flags;
    IDirectFB*                         dfb;
    IDirectFBSurface*                  surface;
} VkDirectFBSurfaceCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`dfb`] is a pointer to the `IDirectFB` main interface of DirectFB.
- [`surface`] is a pointer to a `IDirectFBSurface` surface interface.

# Description
## Valid Usage
-  [`dfb`] **must**  point to a valid DirectFB `IDirectFB`
-  [`surface`] **must**  point to a valid DirectFB `IDirectFBSurface`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`ext_directfb_surface`]
- [`DirectFBSurfaceCreateFlagsEXT`]
- [`StructureType`]
- [`create_direct_fb_surface_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        