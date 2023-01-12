[vkGetImageViewHandleNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewHandleNVX.html) - Get the handle for an image view for a specific descriptor type

# C Specifications
To get the handle for an image view, call:
```c
// Provided by VK_NVX_image_view_handle
uint32_t vkGetImageViewHandleNVX(
    VkDevice                                    device,
    const VkImageViewHandleInfoNVX*             pInfo);
```

# Parameters
- [`device`] is the logical device that owns the image view.
- [`p_info`] describes the image view to query and type of handle.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`ImageViewHandleInfoNVX`] structure

# Related
- [`nvx_image_view_handle`]
- [`Device`]
- [`ImageViewHandleInfoNVX`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        