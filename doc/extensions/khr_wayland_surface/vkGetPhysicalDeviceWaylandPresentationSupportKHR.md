[vkGetPhysicalDeviceWaylandPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html) - Query physical device for presentation to Wayland

# C Specifications
To determine whether a queue family of a physical device supports
presentation to a Wayland compositor, call:
```c
// Provided by VK_KHR_wayland_surface
VkBool32 vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t                                    queueFamilyIndex,
    struct wl_display*                          display);
```

# Parameters
- [`physical_device`] is the physical device.
- [`queue_family_index`] is the queue family index.
- [`display`] is a pointer to the [`wl_display`] associated with a Wayland compositor.

# Description
This platform-specific function  **can**  be called prior to creating a surface.
## Valid Usage
-  [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by [`get_physical_device_queue_family_properties`] for the given [`physical_device`]

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`display`] **must**  be a valid pointer to a [`wl_display`] value

# Related
- [`VK_KHR_wayland_surface`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        