[VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) - Structure describing display native HDR specific capabilities of a surface

# C Specifications
The [`DisplayNativeHdrSurfaceCapabilitiesAMD`] structure is defined as:
```c
// Provided by VK_AMD_display_native_hdr
typedef struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           localDimmingSupport;
} VkDisplayNativeHdrSurfaceCapabilitiesAMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`local_dimming_support`] specifies whether the surface supports local dimming. If this is `VK_TRUE`, [`SwapchainDisplayNativeHdrCreateInfoAMD`] **can**  be used to explicitly enable or disable local dimming for the surface. Local dimming may also be overriden by [`set_local_dimming_amd`] during the lifetime of the swapchain.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`

# Related
- [`amd_display_native_hdr`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        