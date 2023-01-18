[VkViSurfaceCreateInfoNN](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViSurfaceCreateInfoNN.html) - Structure specifying parameters of a newly created VI surface object

# C Specifications
The [`ViSurfaceCreateInfoNN`] structure is defined as:
```c
// Provided by VK_NN_vi_surface
typedef struct VkViSurfaceCreateInfoNN {
    VkStructureType             sType;
    const void*                 pNext;
    VkViSurfaceCreateFlagsNN    flags;
    void*                       window;
} VkViSurfaceCreateInfoNN;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`window`] is the `nn`::`vi`::`NativeWindowHandle` for the `nn`::`vi`::`Layer` with which to associate the surface.

# Description
## Valid Usage
-  [`window`] **must**  be a valid `nn`::`vi`::`NativeWindowHandle`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`VK_NN_vi_surface`]
- [`StructureType`]
- [`ViSurfaceCreateFlagsNN`]
- [`create_vi_surface_nn`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        