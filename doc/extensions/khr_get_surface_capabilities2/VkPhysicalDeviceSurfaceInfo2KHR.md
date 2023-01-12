[VkPhysicalDeviceSurfaceInfo2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html) - Structure specifying a surface and related swapchain creation parameters

# C Specifications
The [`PhysicalDeviceSurfaceInfo2KHR`] structure is defined as:
```c
// Provided by VK_KHR_get_surface_capabilities2
typedef struct VkPhysicalDeviceSurfaceInfo2KHR {
    VkStructureType    sType;
    const void*        pNext;
    VkSurfaceKHR       surface;
} VkPhysicalDeviceSurfaceInfo2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`surface`] is the surface that will be associated with the swapchain.

# Description
The members of [`PhysicalDeviceSurfaceInfo2KHR`] correspond to the
arguments to [`get_physical_device_surface_capabilities_khr`], with
[`s_type`] and [`p_next`] added for extensibility.Additional capabilities of a surface  **may**  be available to swapchains created
with different full-screen exclusive settings - particularly if exclusive
full-screen access is application controlled.
These additional capabilities  **can**  be queried by adding a
[`SurfaceFullScreenExclusiveInfoEXT`] structure to the [`p_next`] chain
of this structure when used to query surface properties.
Additionally, for Win32 surfaces with application controlled exclusive
full-screen access, chaining a
[`SurfaceFullScreenExclusiveWin32InfoEXT`] structure  **may**  also report
additional surface capabilities.
These additional capabilities only apply to swapchains created with the same
parameters included in the [`p_next`] chain of
[`SwapchainCreateInfoKHR`].
## Valid Usage
-    If the [`p_next`] chain includes a [`SurfaceFullScreenExclusiveInfoEXT`] structure with its `fullScreenExclusive` member set to `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`, and [`surface`] was created using [`create_win32_surface_khr`], a [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure  **must**  be included in the [`p_next`] chain
-    When passed as the `pSurfaceInfo` parameter of [`get_physical_device_surface_capabilities2_khr`], if the `[`google_surfaceless_query`]` extension is enabled and the [`p_next`] chain of the `pSurfaceCapabilities` parameter includes [`SurfaceProtectedCapabilitiesKHR`], then [`surface`] **can**  be [`crate::Handle::null`]. Otherwise, [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-    When passed as the `pSurfaceInfo` parameter of [`get_physical_device_surface_formats2_khr`], if the `[`google_surfaceless_query`]` extension is enabled, then [`surface`] **can**  be [`crate::Handle::null`]. Otherwise, [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-    When passed as the `pSurfaceInfo` parameter of [`get_physical_device_surface_present_modes2_ext`], if the `[`google_surfaceless_query`]` extension is enabled, then [`surface`] **can**  be [`crate::Handle::null`]. Otherwise, [`surface`] **must**  be a valid [`SurfaceKHR`] handle

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`SurfaceFullScreenExclusiveInfoEXT`] or [`SurfaceFullScreenExclusiveWin32InfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-    If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid [`SurfaceKHR`] handle

# Related
- [`khr_get_surface_capabilities2`]
- [`StructureType`]
- [`SurfaceKHR`]
- [`get_device_group_surface_present_modes2_ext`]
- [`get_physical_device_surface_capabilities2_khr`]
- [`get_physical_device_surface_formats2_khr`]
- [`get_physical_device_surface_present_modes2_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        