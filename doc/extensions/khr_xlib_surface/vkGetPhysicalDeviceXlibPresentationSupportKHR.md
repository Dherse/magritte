[vkGetPhysicalDeviceXlibPresentationSupportKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html) - Query physical device for presentation to X11 server using Xlib

# C Specifications
To determine whether a queue family of a physical device supports
presentation to an X11 server, using the Xlib client-side library, call:
```c
// Provided by VK_KHR_xlib_surface
VkBool32 vkGetPhysicalDeviceXlibPresentationSupportKHR(
    VkPhysicalDevice                            physicalDevice,
    uint32_t                                    queueFamilyIndex,
    Display*                                    dpy,
    VisualID                                    visualID);
```

# Parameters
- [`physical_device`] is the physical device.
- [`queue_family_index`] is the queue family index.
- [`dpy`] is a pointer to an Xlib [`Display`] connection to the server.
- `visualId` is an X11 visual ([`VisualID`]).

# Description
This platform-specific function  **can**  be called prior to creating a surface.
## Valid Usage
-  [`queue_family_index`] **must**  be less than `pQueueFamilyPropertyCount` returned by [`get_physical_device_queue_family_properties`] for the given [`physical_device`]

## Valid Usage (Implicit)
-  [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
-  [`dpy`] **must**  be a valid pointer to a [`Display`] value

# Related
- [`VK_KHR_xlib_surface`]
- [`PhysicalDevice`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        