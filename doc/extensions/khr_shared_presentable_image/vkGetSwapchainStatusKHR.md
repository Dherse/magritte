[vkGetSwapchainStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html) - Get a swapchain's status

# C Specifications
In order to query a swapchain’s status when rendering to a shared
presentable image, call:
```c
// Provided by VK_KHR_shared_presentable_image
VkResult vkGetSwapchainStatusKHR(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the swapchain to query.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-    Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS`  - `VK_SUBOPTIMAL_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`

# Related
- [`VK_KHR_shared_presentable_image`]
- [`Device`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        