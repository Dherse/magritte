[vkGetPhysicalDeviceSurfacePresentModes2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html) - Query supported presentation modes

# C Specifications
Alternatively, to query the supported presentation modes for a surface
combined with select other fixed swapchain creation parameters, call:
```c
// Provided by VK_EXT_full_screen_exclusive
VkResult vkGetPhysicalDeviceSurfacePresentModes2EXT(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
    uint32_t*                                   pPresentModeCount,
    VkPresentModeKHR*                           pPresentModes);
```

# Parameters
- [`physical_device`] is the physical device that will be associated with the swapchain to be created, as described for [`create_swapchain_khr`].
- [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
- [`p_present_mode_count`] is a pointer to an integer related to the number of presentation modes available or queried, as described below.
- [`p_present_modes`] is either `NULL` or a pointer to an array of [`PresentModeKHR`] values, indicating the supported presentation modes.

# Description
[`get_physical_device_surface_present_modes2_ext`] behaves similarly to
[`get_physical_device_surface_present_modes_khr`], with the ability to specify
extended inputs via chained input structures.
## Valid Usage
-    If the `[`google_surfaceless_query`]` extension is not enabled, `pSurfaceInfo->surface` **must**  be a valid [`SurfaceKHR`] handle
-    If `pSurfaceInfo->surface` is not [`crate::Handle::null`], it  **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`] structure
-  [`p_present_mode_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_present_mode_count`] is not `0`, and [`p_present_modes`] is not `NULL`, [`p_present_modes`] **must**  be a valid pointer to an array of [`p_present_mode_count`][`PresentModeKHR`] values

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`ext_full_screen_exclusive`]
- [`PhysicalDevice`]
- [`PhysicalDeviceSurfaceInfo2KHR`]
- [`PresentModeKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        