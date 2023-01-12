[VkImageBlit](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageBlit.html) - Structure specifying an image blit operation

# C Specifications
The [`ImageBlit`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageBlit {
    VkImageSubresourceLayers    srcSubresource;
    VkOffset3D                  srcOffsets[2];
    VkImageSubresourceLayers    dstSubresource;
    VkOffset3D                  dstOffsets[2];
} VkImageBlit;
```

# Members
- [`src_subresource`] is the subresource to blit from.
- [`src_offsets`] is a pointer to an array of two [`Offset3D`] structures specifying the bounds of the source region within [`src_subresource`].
- [`dst_subresource`] is the subresource to blit into.
- [`dst_offsets`] is a pointer to an array of two [`Offset3D`] structures specifying the bounds of the destination region within [`dst_subresource`].

# Description
For each element of the `pRegions` array, a blit operation is performed
for the specified source and destination regions.
## Valid Usage
-    The `aspectMask` member of [`src_subresource`] and [`dst_subresource`] **must**  match
-    The `layerCount` member of [`src_subresource`] and [`dst_subresource`] **must**  match

## Valid Usage (Implicit)
-  [`src_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure
-  [`dst_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure

# Related
- [`crate::vulkan1_0`]
- [`ImageSubresourceLayers`]
- [`Offset3D`]
- [`cmd_blit_image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        