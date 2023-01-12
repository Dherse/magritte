[VkSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2KHR.html) - Structure describing capabilities of a surface

# C Specifications
The [`SurfaceCapabilities2KHR`] structure is defined as:
```c
// Provided by VK_KHR_get_surface_capabilities2
typedef struct VkSurfaceCapabilities2KHR {
    VkStructureType             sType;
    void*                       pNext;
    VkSurfaceCapabilitiesKHR    surfaceCapabilities;
} VkSurfaceCapabilities2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`surface_capabilities`] is a [`SurfaceCapabilitiesKHR`] structure describing the capabilities of the specified surface.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DisplayNativeHdrSurfaceCapabilitiesAMD`], [`SharedPresentSurfaceCapabilitiesKHR`], [`SurfaceCapabilitiesFullScreenExclusiveEXT`], or [`SurfaceProtectedCapabilitiesKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`khr_get_surface_capabilities2`]
- [`StructureType`]
- [`SurfaceCapabilitiesKHR`]
- [`get_physical_device_surface_capabilities2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        