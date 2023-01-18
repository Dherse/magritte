[VkPresentModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html) - Presentation mode supported for a surface

# C Specifications
Possible values of elements of the
[`get_physical_device_surface_present_modes_khr`]`::pPresentModes` array,
indicating the supported presentation modes for a surface, are:
```c
// Provided by VK_KHR_surface
typedef enum VkPresentModeKHR {
    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
    VK_PRESENT_MODE_MAILBOX_KHR = 1,
    VK_PRESENT_MODE_FIFO_KHR = 2,
    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
  // Provided by VK_KHR_shared_presentable_image
    VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR = 1000111000,
  // Provided by VK_KHR_shared_presentable_image
    VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR = 1000111001,
} VkPresentModeKHR;
```

# Description
- [`IMMEDIATE`] specifies that the presentation engine does not wait for a vertical blanking period to update the current image, meaning this mode  **may**  result in visible tearing. No internal queuing of presentation requests is needed, as the requests are applied immediately.
- [`MAILBOX`] specifies that the presentation engine waits for the next vertical blanking period to update the current image. Tearing  **cannot**  be observed. An internal single-entry queue is used to hold pending presentation requests. If the queue is full when a new presentation request is received, the new request replaces the existing entry, and any images associated with the prior entry become available for re-use by the application. One request is removed from the queue and processed during each vertical blanking period in which the queue is non-empty.
- [`FIFO`] specifies that the presentation engine waits for the next vertical blanking period to update the current image. Tearing  **cannot**  be observed. An internal queue is used to hold pending presentation requests. New requests are appended to the end of the queue, and one request is removed from the beginning of the queue and processed during each vertical blanking period in which the queue is non-empty. This is the only value of `presentMode` that is  **required**  to be supported.
- [`FIFO_RELAXED`] specifies that the presentation engine generally waits for the next vertical blanking period to update the current image. If a vertical blanking period has already passed since the last update of the current image then the presentation engine does not wait for another vertical blanking period for the update, meaning this mode  **may**  result in visible tearing in this case. This mode is useful for reducing visual stutter with an application that will mostly present a new image before the next vertical blanking period, but may occasionally be late, and present a new image just after the next vertical blanking period. An internal queue is used to hold pending presentation requests. New requests are appended to the end of the queue, and one request is removed from the beginning of the queue and processed during or after each vertical blanking period in which the queue is non-empty.
- [`SHARED_DEMAND_REFRESH`] specifies that the presentation engine and application have concurrent access to a single image, which is referred to as a *shared presentable image*. The presentation engine is only required to update the current image after a new presentation request is received. Therefore the application  **must**  make a presentation request whenever an update is required. However, the presentation engine  **may**  update the current image at any point, meaning this mode  **may**  result in visible tearing.
- [`SHARED_CONTINUOUS_REFRESH`] specifies that the presentation engine and application have concurrent access to a single image, which is referred to as a *shared presentable image*. The presentation engine periodically updates the current image on its regular refresh cycle. The application is only required to make one initial presentation request, after which the presentation engine  **must**  update the current image without any need for further presentation requests. The application  **can**  indicate the image contents have been updated by making a presentation request, but this does not guarantee the timing of when it will be updated. This mode  **may**  result in visible tearing if rendering to the image is not timed correctly.
The supported [`ImageUsageFlagBits`] of the presentable images of a
swapchain created for a surface  **may**  differ depending on the presentation
mode, and can be determined as per the table below:

# Related
- [`VK_KHR_surface`]
- [`SwapchainCreateInfoKHR`]
- [`get_physical_device_surface_present_modes2_ext`]
- [`get_physical_device_surface_present_modes_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        