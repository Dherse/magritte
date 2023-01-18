[vkGetPhysicalDeviceSurfaceCapabilities2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html) - Query surface capabilities

# C Specifications
To query the basic capabilities of a surface, needed in order to create a
swapchain, call:
```c
// Provided by VK_EXT_display_surface_counter
VkResult vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    VkPhysicalDevice                            physicalDevice,
    VkSurfaceKHR                                surface,
    VkSurfaceCapabilities2EXT*                  pSurfaceCapabilities);
```

# Parameters
- [`physical_device`] is the physical device that will be associated with the swapchain to be created, as described for [`create_swapchain_khr`].
- [`surface`] is the surface that will be associated with the swapchain.
- [`p_surface_capabilities`] is a pointer to a [`SurfaceCapabilities2EXT`] structure in which the capabilities are returned.

# Description
[`get_physical_device_surface_capabilities2_ext`] behaves similarly to
[`get_physical_device_surface_capabilities_khr`], with the ability to return
extended information by adding extending structures to the `pNext` chain
of its [`p_surface_capabilities`] parameter.
## Valid Usage
- [[VUID-{refpage}-surface-06523]]  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
- [[VUID-{refpage}-surface-06211]]  [`surface`] **must**  be supported by [`physical_device`], as reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`p_surface_capabilities`] **must**  be a valid pointer to a [`SurfaceCapabilities2EXT`] structure
-    Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_EXT_display_surface_counter`]
- [`PhysicalDevice`]
- [`SurfaceCapabilities2EXT`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        