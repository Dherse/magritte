[vkGetPhysicalDeviceSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html) - Query supported presentation modes

# C Specifications
To query the supported presentation modes for a surface, call:
```c
// Provided by VK_KHR_surface
VkResult vkGetPhysicalDeviceSurfacePresentModesKHR(
    VkPhysicalDevice                            physicalDevice,
    VkSurfaceKHR                                surface,
    uint32_t*                                   pPresentModeCount,
    VkPresentModeKHR*                           pPresentModes);
```

# Parameters
- [`physical_device`] is the physical device that will be associated with the swapchain to be created, as described for [`create_swapchain_khr`].
- [`surface`] is the surface that will be associated with the swapchain.
- [`p_present_mode_count`] is a pointer to an integer related to the number of presentation modes available or queried, as described below.
- [`p_present_modes`] is either `NULL` or a pointer to an array of [`PresentModeKHR`] values, indicating the supported presentation modes.

# Description
If [`p_present_modes`] is `NULL`, then the number of presentation modes
supported for the given [`surface`] is returned in
[`p_present_mode_count`].
Otherwise, [`p_present_mode_count`] **must**  point to a variable set by the user
to the number of elements in the [`p_present_modes`] array, and on return
the variable is overwritten with the number of values actually written to
[`p_present_modes`].
If the value of [`p_present_mode_count`] is less than the number of
presentation modes supported, at most [`p_present_mode_count`] values will be
written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available modes were
returned.If the `[`google_surfaceless_query`]` extension is enabled, the values
returned in [`p_present_modes`] will be identical for every valid surface
created on this physical device, and so [`surface`] **can**  be
[`crate::Handle::null`].
## Valid Usage
-    If the `[`google_surfaceless_query`]` extension is not enabled, [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-    If [`surface`] is not [`crate::Handle::null`], it  **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-    If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`p_present_mode_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_present_mode_count`] is not `0`, and [`p_present_modes`] is not `NULL`, [`p_present_modes`] **must**  be a valid pointer to an array of [`p_present_mode_count`][`PresentModeKHR`] values
-    Both of [`physical_device`], and [`surface`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`khr_surface`]
- [`PhysicalDevice`]
- [`PresentModeKHR`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        