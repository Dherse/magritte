[VkMacOSSurfaceCreateInfoMVK](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMacOSSurfaceCreateInfoMVK.html) - Structure specifying parameters of a newly created macOS surface object

# C Specifications
The [`MacOsSurfaceCreateInfoMVK`] structure is defined as:
```c
// Provided by VK_MVK_macos_surface
typedef struct VkMacOSSurfaceCreateInfoMVK {
    VkStructureType                 sType;
    const void*                     pNext;
    VkMacOSSurfaceCreateFlagsMVK    flags;
    const void*                     pView;
} VkMacOSSurfaceCreateInfoMVK;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`view`] is a reference to either a [`CaMetalLayer`] object or an `NSView` object.

# Description
## Valid Usage
-    If [`view`] is a [`CaMetalLayer`] object, it  **must**  be a valid [`CaMetalLayer`]
-    If [`view`] is an `NSView` object, it  **must**  be a valid `NSView`,  **must**  be backed by a `CALayer` object of type [`CaMetalLayer`], and [`create_mac_os_surface_mvk`] **must**  be called on the main thread

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`VK_MVK_macos_surface`]
- [`MacOsSurfaceCreateFlagsMVK`]
- [`StructureType`]
- [`create_mac_os_surface_mvk`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        