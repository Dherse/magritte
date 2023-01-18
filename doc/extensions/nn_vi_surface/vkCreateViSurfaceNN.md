[vkCreateViSurfaceNN](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html) - Create a slink:VkSurfaceKHR object for a VI layer

# C Specifications
To create a [`SurfaceKHR`] object for an `nn`::`vi`::`Layer`,
query the layer’s native handle using
`nn`::`vi`::`GetNativeWindow`, and then call:
```c
// Provided by VK_NN_vi_surface
VkResult vkCreateViSurfaceNN(
    VkInstance                                  instance,
    const VkViSurfaceCreateInfoNN*              pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkSurfaceKHR*                               pSurface);
```

# Parameters
- [`instance`] is the instance with which to associate the surface.
- [`p_create_info`] is a pointer to a [`ViSurfaceCreateInfoNN`] structure containing parameters affecting the creation of the surface object.
- [`p_allocator`] is the allocator used for host memory allocated for the surface object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
- [`p_surface`] is a pointer to a [`SurfaceKHR`] handle in which the created surface object is returned.

# Description
During the lifetime of a surface created using a particular
`nn`::`vi`::`NativeWindowHandle`, applications  **must**  not attempt to
create another surface for the same `nn`::`vi`::`Layer` or attempt
to connect to the same `nn`::`vi`::`Layer` through other platform
mechanisms.If the native window is created with a specified size, `currentExtent`
will reflect that size.
In this case, applications should use the same size for the swapchain’s
`imageExtent`.
Otherwise, the `currentExtent` will have the special value
(0xFFFFFFFF, 0xFFFFFFFF), indicating that applications are expected to
choose an appropriate size for the swapchain’s `imageExtent` (e.g., by
matching the result of a call to
`nn`::`vi`::`GetDisplayResolution`).
## Valid Usage (Implicit)
-  [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`ViSurfaceCreateInfoNN`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_surface`] **must**  be a valid pointer to a [`SurfaceKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`

# Related
- [`VK_NN_vi_surface`]
- [`AllocationCallbacks`]
- [`Instance`]
- [`SurfaceKHR`]
- [`ViSurfaceCreateInfoNN`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        