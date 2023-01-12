[vkAcquireNextImageKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html) - Retrieve the index of the next available presentable image

# C Specifications
To acquire an available presentable image to use, and retrieve the index of
that image, call:
```c
// Provided by VK_KHR_swapchain
VkResult vkAcquireNextImageKHR(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain,
    uint64_t                                    timeout,
    VkSemaphore                                 semaphore,
    VkFence                                     fence,
    uint32_t*                                   pImageIndex);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the non-retired swapchain from which an image is being acquired.
- [`timeout`] specifies how long the function waits, in nanoseconds, if no image is available.
- [`semaphore`] is [`crate::Handle::null`] or a semaphore to signal.
- [`fence`] is [`crate::Handle::null`] or a fence to signal.
- [`p_image_index`] is a pointer to a `uint32_t` in which the index of the next image to use (i.e. an index into the array of images returned by [`get_swapchain_images_khr`]) is returned.

# Description
## Valid Usage
-  [`swapchain`] **must**  not be in the retired state
-    If [`semaphore`] is not [`crate::Handle::null`] it  **must**  be unsignaled
-    If [`semaphore`] is not [`crate::Handle::null`] it  **must**  not have any uncompleted signal or wait operations pending
-    If [`fence`] is not [`crate::Handle::null`] it  **must**  be unsignaled and  **must**  not be associated with any other queue command that has not yet completed execution on that queue
-  [`semaphore`] and [`fence`] **must**  not both be equal to [`crate::Handle::null`]
-    If the number of currently acquired images is greater than the difference between the number of images in [`swapchain`] and the value of [`SurfaceCapabilitiesKHR::min_image_count`] as returned by a call to [`get_physical_device_surface_capabilities2_khr`] with the `surface` used to create [`swapchain`], [`timeout`] **must**  not be `UINT64_MAX`
-  [`semaphore`] **must**  have a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-    If [`semaphore`] is not [`crate::Handle::null`], [`semaphore`] **must**  be a valid [`Semaphore`] handle
-    If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be a valid [`Fence`] handle
-  [`p_image_index`] **must**  be a valid pointer to a `uint32_t` value
-    If [`semaphore`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]
-    If [`fence`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]
-    Both of [`device`], and [`swapchain`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized
- Host access to [`semaphore`] **must**  be externally synchronized
- Host access to [`fence`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS`  - `VK_TIMEOUT`  - `VK_NOT_READY`  - `VK_SUBOPTIMAL_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`

# Related
- [`khr_swapchain`]
- [`Device`]
- [`Fence`]
- [`Semaphore`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        