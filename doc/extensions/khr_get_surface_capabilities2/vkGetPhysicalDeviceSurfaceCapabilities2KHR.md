[vkGetPhysicalDeviceSurfaceCapabilities2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html) - Reports capabilities of a surface on a physical device

# C Specifications
To query the basic capabilities of a surface defined by the core or
extensions, call:
```c
// Provided by VK_KHR_get_surface_capabilities2
VkResult vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    VkPhysicalDevice                            physicalDevice,
    const VkPhysicalDeviceSurfaceInfo2KHR*      pSurfaceInfo,
    VkSurfaceCapabilities2KHR*                  pSurfaceCapabilities);
```

# Parameters
- [`physical_device`] is the physical device that will be associated with the swapchain to be created, as described for [`create_swapchain_khr`].
- [`p_surface_info`] is a pointer to a [`PhysicalDeviceSurfaceInfo2KHR`] structure describing the surface and other fixed parameters that would be consumed by [`create_swapchain_khr`].
- [`p_surface_capabilities`] is a pointer to a [`SurfaceCapabilities2KHR`] structure in which the capabilities are returned.

# Description
[`get_physical_device_surface_capabilities2_khr`] behaves similarly to
[`get_physical_device_surface_capabilities_khr`], with the ability to specify
extended inputs via chained input structures, and to return extended
information via chained output structures.
## Valid Usage
-  `pSurfaceInfo->surface` **must**  be a valid [`SurfaceKHR`] handle
-  `pSurfaceInfo->surface` **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

-    If a [`SurfaceCapabilitiesFullScreenExclusiveEXT`] structure is included in the `pNext` chain of [`p_surface_capabilities`], a [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure  **must**  be included in the `pNext` chain of [`p_surface_info`]

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`p_surface_info`] **must**  be a valid pointer to a valid [`PhysicalDeviceSurfaceInfo2KHR`] structure
-  [`p_surface_capabilities`] **must**  be a valid pointer to a [`SurfaceCapabilities2KHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_KHR_get_surface_capabilities2`]
- [`PhysicalDevice`]
- [`PhysicalDeviceSurfaceInfo2KHR`]
- [`SurfaceCapabilities2KHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        