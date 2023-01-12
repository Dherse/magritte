[vkSetHdrMetadataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html) - Set Hdr metadata

# C Specifications
To provide Hdr metadata to an implementation, call:
```c
// Provided by VK_EXT_hdr_metadata
void vkSetHdrMetadataEXT(
    VkDevice                                    device,
    uint32_t                                    swapchainCount,
    const VkSwapchainKHR*                       pSwapchains,
    const VkHdrMetadataEXT*                     pMetadata);
```

# Parameters
- [`device`] is the logical device where the swapchain(s) were created.
- [`swapchain_count`] is the number of swapchains included in [`p_swapchains`].
- [`p_swapchains`] is a pointer to an array of [`swapchain_count`][`SwapchainKHR`] handles.
- [`p_metadata`] is a pointer to an array of [`swapchain_count`][`HdrMetadataEXT`] structures.

# Description
The metadata will be applied to the specified [`SwapchainKHR`] objects
at the next [`queue_present_khr`] call using that [`SwapchainKHR`]
object.
The metadata will persist until a subsequent [`set_hdr_metadata_ext`]
changes it.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_swapchains`] **must**  be a valid pointer to an array of [`swapchain_count`] valid [`SwapchainKHR`] handles
-  [`p_metadata`] **must**  be a valid pointer to an array of [`swapchain_count`] valid [`HdrMetadataEXT`] structures
-  [`swapchain_count`] **must**  be greater than `0`
-    Both of [`device`], and the elements of [`p_swapchains`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

# Related
- [`ext_hdr_metadata`]
- [`Device`]
- [`HdrMetadataEXT`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        