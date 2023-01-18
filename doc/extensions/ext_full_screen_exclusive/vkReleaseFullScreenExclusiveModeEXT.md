[vkReleaseFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html) - Release full-screen exclusive mode from a swapchain

# C Specifications
To release exclusive full-screen access from a swapchain, call:
```c
// Provided by VK_EXT_full_screen_exclusive
VkResult vkReleaseFullScreenExclusiveModeEXT(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the swapchain to release exclusive full-screen access from.

# Description
## Valid Usage
-  [`swapchain`] **must**  not be in the retired state
-  [`swapchain`] **must**  be a swapchain created with a [`SurfaceFullScreenExclusiveInfoEXT`] structure, with `fullScreenExclusive` set to `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`

# Related
- [`VK_EXT_full_screen_exclusive`]
- [`Device`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        