[vkGetPhysicalDeviceSurfaceFormats2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html) - Query color formats supported by surface

# C Specifications
To query the supported swapchain format tuples for a surface, call:
```c
// Provided by VK_KHR_get_surface_capabilities2
VkResult vkGetPhysicalDeviceSurfaceFormats2KHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
    uint32_t*                                   pSurfaceFormatCount,
    VkSurfaceFormat2KHR*                        pSurfaceFormats);
```

# Parameters
- [`physical_device`] is the physical device that will be associated with the swapchain to be created, as described for [`create_swapchain_khr`].
- [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
- [`p_surface_format_count`] is a pointer to an integer related to the number of format tuples available or queried, as described below.
- [`p_surface_formats`] is either `NULL` or a pointer to an array of [`SurfaceFormat2KHR`] structures.

# Description
[`get_physical_device_surface_formats2_khr`] behaves similarly to
[`get_physical_device_surface_formats_khr`], with the ability to be extended
via `pNext` chains.If [`p_surface_formats`] is `NULL`, then the number of format tuples
supported for the given `surface` is returned in
[`p_surface_format_count`].
Otherwise, [`p_surface_format_count`] **must**  point to a variable set by the
user to the number of elements in the [`p_surface_formats`] array, and on
return the variable is overwritten with the number of structures actually
written to [`p_surface_formats`].
If the value of [`p_surface_format_count`] is less than the number of format
tuples supported, at most [`p_surface_format_count`] structures will be
written, and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available values were
returned.
## Valid Usage
-    If the `[`VK_GOOGLE_surfaceless_query`]` extension is not enabled, `pSurfaceInfo->surface` **must**  be a valid [`SurfaceKHR`] handle
-    If `pSurfaceInfo->surface` is not [`crate::Handle::null`], it  **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`] structure
-  [`p_surface_format_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_surface_format_count`] is not `0`, and [`p_surface_formats`] is not `NULL`, [`p_surface_formats`] **must**  be a valid pointer to an array of [`p_surface_format_count`][`SurfaceFormat2KHR`] structures

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_KHR_get_surface_capabilities2`]
- [`PhysicalDevice`]
- [`PhysicalDeviceSurfaceInfo2KHR`]
- [`SurfaceFormat2KHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        