[VkAndroidSurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidSurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Android surface object

# C Specifications
The [`AndroidSurfaceCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_android_surface
typedef struct VkAndroidSurfaceCreateInfoKHR {
    VkStructureType                   sType;
    const void*                       pNext;
    VkAndroidSurfaceCreateFlagsKHR    flags;
    struct ANativeWindow*             window;
} VkAndroidSurfaceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`window`] is a pointer to the [`ANativeWindow`] to associate the surface with.

# Description
## Valid Usage
-  [`window`] **must**  point to a valid Android [`ANativeWindow`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`VK_KHR_android_surface`]
- [`AndroidSurfaceCreateFlagsKHR`]
- [`StructureType`]
- [`create_android_surface_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        