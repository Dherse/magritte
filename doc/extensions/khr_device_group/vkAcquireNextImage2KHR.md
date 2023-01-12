[vkAcquireNextImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html) - Retrieve the index of the next available presentable image

# C Specifications
To acquire an available presentable image to use, and retrieve the index of
that image, call:
```c
// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
VkResult vkAcquireNextImage2KHR(
    VkDevice                                    device,
    const VkAcquireNextImageInfoKHR*            pAcquireInfo,
    uint32_t*                                   pImageIndex);
```

# Parameters
- [`device`] is the device associated with `swapchain`.
- [`p_acquire_info`] is a pointer to a [`AcquireNextImageInfoKHR`] structure containing parameters of the acquire.
- [`p_image_index`] is a pointer to a `uint32_t` that is set to the index of the next image to use.

# Description
## Valid Usage
-    If the number of currently acquired images is greater than the difference between the number of images in the `swapchain` member of [`p_acquire_info`] and the value of [`SurfaceCapabilitiesKHR::min_image_count`] as returned by a call to [`get_physical_device_surface_capabilities2_khr`] with the `surface` used to create `swapchain`, the `timeout` member of [`p_acquire_info`] **must**  not be `UINT64_MAX`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_acquire_info`] **must**  be a valid pointer to a valid [`AcquireNextImageInfoKHR`] structure
-  [`p_image_index`] **must**  be a valid pointer to a `uint32_t` value

## Return Codes
*   - `VK_SUCCESS`  - `VK_TIMEOUT`  - `VK_NOT_READY`  - `VK_SUBOPTIMAL_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`

# Related
- [`khr_device_group`]
- [`khr_swapchain`]
- [`crate::vulkan1_1`]
- [`AcquireNextImageInfoKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        