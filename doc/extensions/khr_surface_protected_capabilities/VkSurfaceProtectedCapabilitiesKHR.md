[VkSurfaceProtectedCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceProtectedCapabilitiesKHR.html) - Structure describing capability of a surface to be protected

# C Specifications
An application queries if a protected [`SurfaceKHR`] is displayable on a
specific windowing system using [`SurfaceProtectedCapabilitiesKHR`],
which  **can**  be passed in [`p_next`] parameter of
[`SurfaceCapabilities2KHR`].The [`SurfaceProtectedCapabilitiesKHR`] structure is defined as:
```c
// Provided by VK_KHR_surface_protected_capabilities
typedef struct VkSurfaceProtectedCapabilitiesKHR {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           supportsProtected;
} VkSurfaceProtectedCapabilitiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`supports_protected`] specifies whether a protected swapchain created from [`PhysicalDeviceSurfaceInfo2KHR::surface`] for a particular windowing system  **can**  be displayed on screen or not. If [`supports_protected`] is [`TRUE`], then creation of swapchains with the `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set  **must**  be supported for `surface`.

# Description
If the `[`VK_GOOGLE_surfaceless_query`]` extension is enabled, the value
returned in [`supports_protected`] will be identical for every valid
surface created on this physical device, and so in the
[`get_physical_device_surface_capabilities2_khr`] call,
[`PhysicalDeviceSurfaceInfo2KHR::surface`] **can**  be
[`crate::Handle::null`].
In that case, the contents of
[`SurfaceCapabilities2KHR::surface_capabilities`] as well as any
other struct chained to it will be undefined.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`

# Related
- [`VK_KHR_surface_protected_capabilities`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        