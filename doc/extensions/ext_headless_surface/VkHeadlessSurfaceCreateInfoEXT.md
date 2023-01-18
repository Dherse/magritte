[VkHeadlessSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHeadlessSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created headless surface object

# C Specifications
The [`HeadlessSurfaceCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_headless_surface
typedef struct VkHeadlessSurfaceCreateInfoEXT {
    VkStructureType                    sType;
    const void*                        pNext;
    VkHeadlessSurfaceCreateFlagsEXT    flags;
} VkHeadlessSurfaceCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`VK_EXT_headless_surface`]
- [`HeadlessSurfaceCreateFlagsEXT`]
- [`StructureType`]
- [`create_headless_surface_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        