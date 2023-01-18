[vkCreateSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html) - Create a swapchain

# C Specifications
To create a swapchain, call:
```c
// Provided by VK_KHR_swapchain
VkResult vkCreateSwapchainKHR(
    VkDevice                                    device,
    const VkSwapchainCreateInfoKHR*             pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkSwapchainKHR*                             pSwapchain);
```

# Parameters
- [`device`] is the device to create the swapchain for.
- [`p_create_info`] is a pointer to a [`SwapchainCreateInfoKHR`] structure specifying the parameters of the created swapchain.
- [`p_allocator`] is the allocator used for host memory allocated for the swapchain object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
- [`p_swapchain`] is a pointer to a [`SwapchainKHR`] handle in which the created swapchain object will be returned.

# Description
As mentioned above, if [`create_swapchain_khr`] succeeds, it will return a
handle to a swapchain containing an array of at least
`pCreateInfo->minImageCount` presentable images.While acquired by the application, presentable images  **can**  be used in any
way that equivalent non-presentable images  **can**  be used.
A presentable image is equivalent to a non-presentable image created with
the following [`ImageCreateInfo`] parameters:The `pCreateInfo->surface` **must**  not be destroyed until after the
swapchain is destroyed.If `pCreateInfo->oldSwapchain` is [`crate::Handle::null`], and the native
window referred to by `pCreateInfo->surface` is already associated with
a Vulkan swapchain, `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR` **must**  be
returned.If the native window referred to by `pCreateInfo->surface` is already
associated with a non-Vulkan graphics API surface,
`VK_ERROR_NATIVE_WINDOW_IN_USE_KHR` **must**  be returned.The native window referred to by `pCreateInfo->surface` **must**  not become
associated with a non-Vulkan graphics API surface before all associated
Vulkan swapchains have been destroyed.[`create_swapchain_khr`] will return `VK_ERROR_DEVICE_LOST` if the
logical device was lost.
The [`SwapchainKHR`] is a child of the [`device`], and  **must**  not be
destroyed before the [`device`].
However, [`SurfaceKHR`] is not a child of any [`Device`] and is not
affected by the lost device.
After successfully recreating a [`Device`], the same [`SurfaceKHR`] **can**  be used to create a new [`SwapchainKHR`], provided the previous one
was destroyed.If the `oldSwapchain` parameter of [`p_create_info`] is a valid
swapchain, which has exclusive full-screen access, that access is released
from `pCreateInfo->oldSwapchain`.
If the command succeeds in this case, the newly created swapchain will
automatically acquire exclusive full-screen access from
`pCreateInfo->oldSwapchain`.In some cases, swapchain creation  **may**  fail if exclusive full-screen mode is
requested for application control, but for some implementation-specific
reason exclusive full-screen access is unavailable for the particular
combination of parameters provided.
If this occurs, `VK_ERROR_INITIALIZATION_FAILED` will be returned.When the [`SurfaceKHR`] in [`SwapchainCreateInfoKHR`] is a display
surface, then the [`DisplayModeKHR`] in display surface’s
[`DisplaySurfaceCreateInfoKHR`] is associated with a particular
[`DisplayKHR`].
Swapchain creation  **may**  fail if that [`DisplayKHR`] is not acquired by
the application.
In this scenario `VK_ERROR_INITIALIZATION_FAILED` is returned.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`SwapchainCreateInfoKHR`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_swapchain`] **must**  be a valid pointer to a [`SwapchainKHR`] handle

## Host Synchronization
- Host access to `pCreateInfo->surface` **must**  be externally synchronized
- Host access to `pCreateInfo->oldSwapchain` **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_SURFACE_LOST_KHR`  - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_KHR_swapchain`]
- [`AllocationCallbacks`]
- [`Device`]
- [`SwapchainCreateInfoKHR`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        