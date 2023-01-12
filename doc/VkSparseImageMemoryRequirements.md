[VkSparseImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements.html) - Structure specifying sparse image memory requirements

# C Specifications
The [`SparseImageMemoryRequirements`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSparseImageMemoryRequirements {
    VkSparseImageFormatProperties    formatProperties;
    uint32_t                         imageMipTailFirstLod;
    VkDeviceSize                     imageMipTailSize;
    VkDeviceSize                     imageMipTailOffset;
    VkDeviceSize                     imageMipTailStride;
} VkSparseImageMemoryRequirements;
```

# Members
- [`format_properties`] is a [`SparseImageFormatProperties`] structure specifying properties of the image format.
- [`image_mip_tail_first_lod`] is the first mip level at which image subresources are included in the mip tail region.
- [`image_mip_tail_size`] is the memory size (in bytes) of the mip tail region. If `formatProperties.flags` contains `VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT`, this is the size of the whole mip tail, otherwise this is the size of the mip tail of a single array layer. This value is guaranteed to be a multiple of the sparse block size in bytes.
- [`image_mip_tail_offset`] is the opaque memory offset used with [`SparseImageOpaqueMemoryBindInfo`] to bind the mip tail region(s).
- [`image_mip_tail_stride`] is the offset stride between each array-layer’s mip tail, if `formatProperties.flags` does not contain `VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT` (otherwise the value is undefined).

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [`SparseImageFormatProperties`]
- [`SparseImageMemoryRequirements2`]
- [`get_image_sparse_memory_requirements`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        