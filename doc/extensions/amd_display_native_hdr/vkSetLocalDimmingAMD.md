[vkSetLocalDimmingAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetLocalDimmingAMD.html) - Set Local Dimming

# C Specifications
The local dimming HDR setting may also be changed over the life of a
swapchain by calling:
```c
// Provided by VK_AMD_display_native_hdr
void vkSetLocalDimmingAMD(
    VkDevice                                    device,
    VkSwapchainKHR                              swapChain,
    VkBool32                                    localDimmingEnable);
```

# Parameters
- [`device`] is the device associated with [`swap_chain`].
- [`swap_chain`] handle to enable local dimming.
- [`local_dimming_enable`] specifies whether local dimming is enabled for the swapchain.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swap_chain`] **must**  be a valid [`SwapchainKHR`] handle
-    Both of [`device`], and [`swap_chain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Valid Usage
-  [`DisplayNativeHdrSurfaceCapabilitiesAMD::local_dimming_support`] **must**  be supported

# Related
- [`VK_AMD_display_native_hdr`]
- [`Bool32`]
- [`Device`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        