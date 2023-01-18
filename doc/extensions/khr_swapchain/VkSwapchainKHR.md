[VkSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html) - Opaque handle to a swapchain object

# C Specifications
A swapchain object (a.k.a.
swapchain) provides the ability to present rendering results to a surface.
Swapchain objects are represented by [`SwapchainKHR`] handles:
```c
// Provided by VK_KHR_swapchain
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSwapchainKHR)
```

# Related
- [`VK_KHR_swapchain`]
- [`AcquireNextImageInfoKHR`]
- [`BindImageMemorySwapchainInfoKHR`]
- [`ImageSwapchainCreateInfoKHR`]
- [`PresentInfoKHR`]
- [`SwapchainCreateInfoKHR`]
- [`acquire_full_screen_exclusive_mode_ext`]
- [`acquire_next_image_khr`]
- [`create_shared_swapchains_khr`]
- [`create_swapchain_khr`]
- [`destroy_swapchain_khr`]
- [`get_past_presentation_timing_google`]
- [`get_refresh_cycle_duration_google`]
- [`get_swapchain_counter_ext`]
- [`get_swapchain_images_khr`]
- [`get_swapchain_status_khr`]
- [`queue_present_khr`]
- [`release_full_screen_exclusive_mode_ext`]
- [`set_hdr_metadata_ext`]
- [`set_local_dimming_amd`]
- [`wait_for_present_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        