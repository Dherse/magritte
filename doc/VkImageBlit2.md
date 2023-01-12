[VkImageBlit2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageBlit2.html) - Structure specifying an image blit operation

# C Specifications
The [`ImageBlit2`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkImageBlit2 {
    VkStructureType             sType;
    const void*                 pNext;
    VkImageSubresourceLayers    srcSubresource;
    VkOffset3D                  srcOffsets[2];
    VkImageSubresourceLayers    dstSubresource;
    VkOffset3D                  dstOffsets[2];
} VkImageBlit2;
```
or the equivalent
```c
// Provided by VK_KHR_copy_commands2
typedef VkImageBlit2 VkImageBlit2KHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
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
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_BLIT_2`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`CopyCommandTransformInfoQCOM`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`src_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure
-  [`dst_subresource`] **must**  be a valid [`ImageSubresourceLayers`] structure

# Related
- [`khr_copy_commands2`]
- [`crate::vulkan1_3`]
- [`BlitImageInfo2`]
- [`ImageSubresourceLayers`]
- [`Offset3D`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        