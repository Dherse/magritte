[vkGetPhysicalDeviceSurfaceSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html) - Query if presentation is supported

# C Specifications
To determine whether a queue family of a physical device supports
presentation to a given surface, call:
```c
// Provided by VK_KHR_surface
VkResult vkGetPhysicalDeviceSurfaceSupportKHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t                                    queueFamilyIndex,
    VkSurfaceKHR                                surface,
    VkBool32*                                   pSupported);
```

# Parameters
- [`physical_device`] is the physical device.
- [`queue_family_index`] is the queue family.
- [`surface`] is the surface.
- [`p_supported`] is a pointer to a [`Bool32`], which is set to `VK_TRUE` to indicate support, and `VK_FALSE` otherwise.

# Description
## Valid Usage
-  [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by [`get_physical_device_queue_family_properties`] for the given [`physical_device`]

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`surface`] **must**  be a valid [`SurfaceKHR`] handle
-  [`p_supported`] **must**  be a valid pointer to a [`Bool32`] value
-    Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`khr_surface`]
- [`Bool32`]
- [`PhysicalDevice`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        