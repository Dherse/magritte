[vkCreateSharedSwapchainsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html) - Create multiple swapchains that share presentable images

# C Specifications
When the [`VK_KHR_display_swapchain`] extension is enabled, multiple
swapchains that share presentable images are created by calling:
```c
// Provided by VK_KHR_display_swapchain
VkResult vkCreateSharedSwapchainsKHR(
    VkDevice                                    device,
    uint32_t                                    swapchainCount,
    const VkSwapchainCreateInfoKHR*             pCreateInfos,
    const VkAllocationCallbacks*                pAllocator,
    VkSwapchainKHR*                             pSwapchains);
```

# Parameters
- [`device`] is the device to create the swapchains for.
- [`swapchain_count`] is the number of swapchains to create.
- [`p_create_infos`] is a pointer to an array of [`SwapchainCreateInfoKHR`] structures specifying the parameters of the created swapchains.
- [`p_allocator`] is the allocator used for host memory allocated for the swapchain objects when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
- [`p_swapchains`] is a pointer to an array of [`SwapchainKHR`] handles in which the created swapchain objects will be returned.

# Description
[`create_shared_swapchains_khr`] is similar to [`create_swapchain_khr`],
except that it takes an array of [`SwapchainCreateInfoKHR`] structures,
and returns an array of swapchain objects.The swapchain creation parameters that affect the properties and number of
presentable images  **must**  match between all the swapchains.
If the displays used by any of the swapchains do not use the same
presentable image layout or are incompatible in a way that prevents sharing
images, swapchain creation will fail with the result code
`VK_ERROR_INCOMPATIBLE_DISPLAY_KHR`.
If any error occurs, no swapchains will be created.
Images presented to multiple swapchains  **must**  be re-acquired from all of
them before transitioning away from `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`.
After destroying one or more of the swapchains, the remaining swapchains and
the presentable images  **can**  continue to be used.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_infos`] **must**  be a valid pointer to an array of [`swapchain_count`] valid [`SwapchainCreateInfoKHR`] structures
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_swapchains`] **must**  be a valid pointer to an array of [`swapchain_count`][`SwapchainKHR`] handles
-  [`swapchain_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`p_create_infos`][].surface  **must**  be externally synchronized
- Host access to [`p_create_infos`][].oldSwapchain  **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INCOMPATIBLE_DISPLAY_KHR`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_KHR_display_swapchain`]
- [`AllocationCallbacks`]
- [`Device`]
- [`SwapchainCreateInfoKHR`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        