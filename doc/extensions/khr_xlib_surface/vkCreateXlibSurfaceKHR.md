[vkCreateXlibSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateXlibSurfaceKHR.html) - Create a slink:VkSurfaceKHR object for an X11 window, using the Xlib client-side library

# C Specifications
To create a [`SurfaceKHR`] object for an X11 window, using the Xlib
client-side library, call:
```c
// Provided by VK_KHR_xlib_surface
VkResult vkCreateXlibSurfaceKHR(
    VkInstance                                  instance,
    const VkXlibSurfaceCreateInfoKHR*           pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkSurfaceKHR*                               pSurface);
```

# Parameters
- [`instance`] is the instance to associate the surface with.
- [`p_create_info`] is a pointer to a [`XlibSurfaceCreateInfoKHR`] structure containing the parameters affecting the creation of the surface object.
- [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
- [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is returned.

# Description
## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`XlibSurfaceCreateInfoKHR`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_KHR_xlib_surface`]
- [`AllocationCallbacks`]
- [`Instance`]
- [`SurfaceKHR`]
- [`XlibSurfaceCreateInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        