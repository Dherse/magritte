[vkGetSwapchainImagesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html) - Obtain the array of presentable images associated with a swapchain

# C Specifications
To obtain the array of presentable images associated with a swapchain, call:
```c
// Provided by VK_KHR_swapchain
VkResult vkGetSwapchainImagesKHR(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain,
    uint32_t*                                   pSwapchainImageCount,
    VkImage*                                    pSwapchainImages);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the swapchain to query.
- [`p_swapchain_image_count`] is a pointer to an integer related to the number of presentable images available or queried, as described below.
- [`p_swapchain_images`] is either `NULL` or a pointer to an array of [`Image`] handles.

# Description
If [`p_swapchain_images`] is `NULL`, then the number of presentable images
for [`swapchain`] is returned in [`p_swapchain_image_count`].
Otherwise, [`p_swapchain_image_count`] **must**  point to a variable set by the
user to the number of elements in the [`p_swapchain_images`] array, and on
return the variable is overwritten with the number of structures actually
written to [`p_swapchain_images`].
If the value of [`p_swapchain_image_count`] is less than the number of
presentable images for [`swapchain`], at most [`p_swapchain_image_count`]
structures will be written, and `VK_INCOMPLETE` will be returned instead
of `VK_SUCCESS`, to indicate that not all the available presentable
images were returned.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-  [`p_swapchain_image_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_swapchain_image_count`] is not `0`, and [`p_swapchain_images`] is not `NULL`, [`p_swapchain_images`] **must**  be a valid pointer to an array of [`p_swapchain_image_count`][`Image`] handles
-    Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_swapchain`]
- [`Device`]
- [`Image`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        