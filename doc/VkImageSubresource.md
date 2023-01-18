[VkImageSubresource](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresource.html) - Structure specifying an image subresource

# C Specifications
The [`ImageSubresource`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageSubresource {
    VkImageAspectFlags    aspectMask;
    uint32_t              mipLevel;
    uint32_t              arrayLayer;
} VkImageSubresource;
```

# Members
- [`aspect_mask`] is a [`ImageAspectFlags`] value selecting the image *aspect*.
- [`mip_level`] selects the mipmap level.
- [`array_layer`] selects the array layer.

# Description
## Valid Usage (Implicit)
-  [`aspect_mask`] **must**  be a valid combination of [`ImageAspectFlagBits`] values
-  [`aspect_mask`] **must**  not be `0`

# Related
- [`crate::vulkan1_0`]
- [`ImageAspectFlags`]
- [`SparseImageMemoryBind`]
- [`get_image_subresource_layout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        