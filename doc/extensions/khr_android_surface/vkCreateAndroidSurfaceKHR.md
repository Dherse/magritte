[vkCreateAndroidSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAndroidSurfaceKHR.html) - Create a slink:VkSurfaceKHR object for an Android native window

# C Specifications
To create a [`SurfaceKHR`] object for an Android native window, call:
```c
// Provided by VK_KHR_android_surface
VkResult vkCreateAndroidSurfaceKHR(
    VkInstance                                  instance,
    const VkAndroidSurfaceCreateInfoKHR*        pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkSurfaceKHR*                               pSurface);
```

# Parameters
- [`instance`] is the instance to associate the surface with.
- [`p_create_info`] is a pointer to a [`AndroidSurfaceCreateInfoKHR`] structure containing parameters affecting the creation of the surface object.
- [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
- [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is returned.

# Description
During the lifetime of a surface created using a particular
[`ANativeWindow`] handle any attempts to create another surface for the
same [`ANativeWindow`] and any attempts to connect to the same
[`ANativeWindow`] through other platform mechanisms will fail.If successful, [`create_android_surface_khr`] increments the
[`ANativeWindow`]’s reference count, and [`destroy_surface_khr`] will
decrement it.On Android, when a swapchain’s `imageExtent` does not match the
surface’s `currentExtent`, the presentable images will be scaled to the
surface’s dimensions during presentation.
`minImageExtent` is (1,1), and `maxImageExtent` is the maximum
image size supported by the consumer.
For the system compositor, `currentExtent` is the window size (i.e. the
consumer’s preferred size).
## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`AndroidSurfaceCreateInfoKHR`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`

# Related
- [`VK_KHR_android_surface`]
- [`AllocationCallbacks`]
- [`AndroidSurfaceCreateInfoKHR`]
- [`Instance`]
- [`SurfaceKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        