[VkWin32SurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWin32SurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created Win32 surface object

# C Specifications
The [`Win32SurfaceCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_win32_surface
typedef struct VkWin32SurfaceCreateInfoKHR {
    VkStructureType                 sType;
    const void*                     pNext;
    VkWin32SurfaceCreateFlagsKHR    flags;
    HINSTANCE                       hinstance;
    HWND                            hwnd;
} VkWin32SurfaceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`hinstance`] is the Win32 [`HINSTANCE`] for the window to associate the surface with.
- [`hwnd`] is the Win32 [`HWND`] for the window to associate the surface with.

# Description
## Valid Usage
-  [`hinstance`] **must**  be a valid Win32 [`HINSTANCE`]
-  [`hwnd`] **must**  be a valid Win32 [`HWND`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`

# Related
- [`VK_KHR_win32_surface`]
- [`StructureType`]
- [`Win32SurfaceCreateFlagsKHR`]
- [`create_win32_surface_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        