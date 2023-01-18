[vkAcquireFullScreenExclusiveModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html) - Acquire full-screen exclusive mode for a swapchain

# C Specifications
To acquire exclusive full-screen access for a swapchain, call:
```c
// Provided by VK_EXT_full_screen_exclusive
VkResult vkAcquireFullScreenExclusiveModeEXT(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the swapchain to acquire exclusive full-screen access for.

# Description
## Valid Usage
-  [`swapchain`] **must**  not be in the retired state
-  [`swapchain`] **must**  be a swapchain created with a [`SurfaceFullScreenExclusiveInfoEXT`] structure, with `fullScreenExclusive` set to `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`
-  [`swapchain`] **must**  not currently have exclusive full-screen access
A return value of `VK_SUCCESS` indicates that the [`swapchain`]
successfully acquired exclusive full-screen access.
The swapchain will retain this exclusivity until either the application
releases exclusive full-screen access with
[`release_full_screen_exclusive_mode_ext`], destroys the swapchain, or if any
of the swapchain commands return
`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` indicating that the mode
was lost because of platform-specific changes.If the swapchain was unable to acquire exclusive full-screen access to the
display then `VK_ERROR_INITIALIZATION_FAILED` is returned.
An application  **can**  attempt to acquire exclusive full-screen access again
for the same swapchain even if this command fails, or if
`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` has been returned by a
swapchain command.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-    Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_EXT_full_screen_exclusive`]
- [`Device`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        