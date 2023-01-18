[vkGetImageViewAddressNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageViewAddressNVX.html) - Get the device address of an image view

# C Specifications
To get the device address for an image view, call:
```c
// Provided by VK_NVX_image_view_handle
VkResult vkGetImageViewAddressNVX(
    VkDevice                                    device,
    VkImageView                                 imageView,
    VkImageViewAddressPropertiesNVX*            pProperties);
```

# Parameters
- [`device`] is the logical device that owns the image view.
- [`image_view`] is a handle to the image view.
- [`p_properties`] contains the device address and size when the call returns.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`image_view`] **must**  be a valid [`ImageView`] handle
-  [`p_properties`] **must**  be a valid pointer to a [`ImageViewAddressPropertiesNVX`] structure
-  [`image_view`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_UNKNOWN`

# Related
- [`VK_NVX_image_view_handle`]
- [`Device`]
- [`ImageView`]
- [`ImageViewAddressPropertiesNVX`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        