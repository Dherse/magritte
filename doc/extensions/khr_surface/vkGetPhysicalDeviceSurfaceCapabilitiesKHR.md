[vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html) - Query surface capabilities

# C Specifications
To query the basic capabilities of a surface, needed in order to create a
swapchain, call:
```c
// Provided by VK_KHR_surface
VkResult vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    VkPhysicalDevice                            physicalDevice,
    VkSurfaceKHR                                surface,
    VkSurfaceCapabilitiesKHR*                   pSurfaceCapabilities);
```

# Parameters
- [`physical_device`] is the physical device that will be associated with the swapchain to be created, as described for [`create_swapchain_khr`].
- [`surface`] is the surface that will be associated with the swapchain.
- [`p_surface_capabilities`] is a pointer to a [`SurfaceCapabilitiesKHR`] structure in which the capabilities are returned.

# Description
## Valid Usage
-  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`surface`] **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`p_surface_capabilities`] **must**  be a valid pointer to a [`SurfaceCapabilitiesKHR`] structure
-    Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`khr_surface`]
- [`PhysicalDevice`]
- [`SurfaceCapabilitiesKHR`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        