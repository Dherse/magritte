[VkIOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIOSSurfaceCreateInfoMVK.html) - Structure specifying parameters of a newly created iOS surface object

# C Specifications
The [`IosSurfaceCreateInfoMVK`] structure is defined as:
```c
// Provided by VK_MVK_ios_surface
typedef struct VkIOSSurfaceCreateInfoMVK {
    VkStructureType               sType;
    const void*                   pNext;
    VkIOSSurfaceCreateFlagsMVK    flags;
    const void*                   pView;
} VkIOSSurfaceCreateInfoMVK;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`view`] is a reference to either a [`CaMetalLayer`] object or a `UIView` object.

# Description
## Valid Usage
-    If [`view`] is a [`CaMetalLayer`] object, it  **must**  be a valid [`CaMetalLayer`]
-    If [`view`] is a `UIView` object, it  **must**  be a valid `UIView`,  **must**  be backed by a `CALayer` object of type [`CaMetalLayer`], and [`create_ios_surface_mvk`] **must**  be called on the main thread

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`VK_MVK_ios_surface`]
- [`IosSurfaceCreateFlagsMVK`]
- [`StructureType`]
- [`create_ios_surface_mvk`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        