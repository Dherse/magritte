[VkImageSubresourceLayers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSubresourceLayers.html) - Structure specifying an image subresource layers

# C Specifications
The [`ImageSubresourceLayers`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageSubresourceLayers {
    VkImageAspectFlags    aspectMask;
    uint32_t              mipLevel;
    uint32_t              baseArrayLayer;
    uint32_t              layerCount;
} VkImageSubresourceLayers;
```

# Members
- [`aspect_mask`] is a combination of [`ImageAspectFlagBits`], selecting the color, depth and/or stencil aspects to be copied.
- [`mip_level`] is the mipmap level to copy
- [`base_array_layer`] and [`layer_count`] are the starting layer and number of layers to copy.

# Description
## Valid Usage
-    If [`aspect_mask`] contains `VK_IMAGE_ASPECT_COLOR_BIT`, it  **must**  not contain either of `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT`
-  [`aspect_mask`] **must**  not contain `VK_IMAGE_ASPECT_METADATA_BIT`
-  [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index *i*
-  [`layer_count`] **must**  be greater than 0

## Valid Usage (Implicit)
-  [`aspect_mask`] **must**  be a valid combination of [`ImageAspectFlagBits`] values
-  [`aspect_mask`] **must**  not be `0`

# Related
- [`crate::vulkan1_0`]
- [`BufferImageCopy`]
- [`BufferImageCopy2`]
- [VkImageAspectFlags]()
- [`ImageBlit`]
- [`ImageBlit2`]
- [`ImageCopy`]
- [`ImageCopy2`]
- [`ImageResolve`]
- [`ImageResolve2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        