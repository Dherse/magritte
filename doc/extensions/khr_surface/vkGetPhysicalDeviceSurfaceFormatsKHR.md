[vkGetPhysicalDeviceSurfaceFormatsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html) - Query color formats supported by surface

# C Specifications
To query the supported swapchain format-color space pairs for a surface,
call:
```c
// Provided by VK_KHR_surface
VkResult vkGetPhysicalDeviceSurfaceFormatsKHR(
    VkPhysicalDevice                            physicalDevice,
    VkSurfaceKHR                                surface,
    uint32_t*                                   pSurfaceFormatCount,
    VkSurfaceFormatKHR*                         pSurfaceFormats);
```

# Parameters
- [`physical_device`] is the physical device that will be associated with the swapchain to be created, as described for [`create_swapchain_khr`].
- [`surface`] is the surface that will be associated with the swapchain.
- [`p_surface_format_count`] is a pointer to an integer related to the number of format pairs available or queried, as described below.
- [`p_surface_formats`] is either `NULL` or a pointer to an array of [`SurfaceFormatKHR`] structures.

# Description
If [`p_surface_formats`] is `NULL`, then the number of format pairs
supported for the given [`surface`] is returned in
[`p_surface_format_count`].
Otherwise, [`p_surface_format_count`] **must**  point to a variable set by the
user to the number of elements in the [`p_surface_formats`] array, and on
return the variable is overwritten with the number of structures actually
written to [`p_surface_formats`].
If the value of [`p_surface_format_count`] is less than the number of format
pairs supported, at most [`p_surface_format_count`] structures will be
written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available format pairs were
returned.The number of format pairs supported  **must**  be greater than or equal to 1.
[`p_surface_formats`] **must**  not contain an entry whose value for
`format` is `VK_FORMAT_UNDEFINED`.If [`p_surface_formats`] includes an entry whose value for `colorSpace`
is `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR` and whose value for `format`
is a UNORM (or SRGB) format and the corresponding SRGB (or UNORM) format is
a color renderable format for `VK_IMAGE_TILING_OPTIMAL`, then
[`p_surface_formats`] **must**  also contain an entry with the same value for
`colorSpace` and `format` equal to the corresponding SRGB (or UNORM)
format.If the `[`google_surfaceless_query`]` extension is enabled, the values
returned in [`p_surface_formats`] will be identical for every valid surface
created on this physical device, and so [`surface`] **can**  be
[`crate::Handle::null`].
## Valid Usage
-    If the `[`google_surfaceless_query`]` extension is not enabled, [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-    If [`surface`] is not [`crate::Handle::null`], it  **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-    If [`surface`] is not [`crate::Handle::null`], [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`p_surface_format_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_surface_format_count`] is not `0`, and [`p_surface_formats`] is not `NULL`, [`p_surface_formats`] **must**  be a valid pointer to an array of [`p_surface_format_count`][`SurfaceFormatKHR`] structures
-    Both of [`physical_device`], and [`surface`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`khr_surface`]
- [`PhysicalDevice`]
- [`SurfaceFormatKHR`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        