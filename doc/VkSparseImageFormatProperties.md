[VkSparseImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties.html) - Structure specifying sparse image format properties

# C Specifications
The [`SparseImageFormatProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSparseImageFormatProperties {
    VkImageAspectFlags          aspectMask;
    VkExtent3D                  imageGranularity;
    VkSparseImageFormatFlags    flags;
} VkSparseImageFormatProperties;
```

# Members
- [`aspect_mask`] is a bitmask [`ImageAspectFlagBits`] specifying which aspects of the image the properties apply to.
- [`image_granularity`] is the width, height, and depth of the sparse image block in texels or compressed texel blocks.
- [`flags`] is a bitmask of [`SparseImageFormatFlagBits`] specifying additional information about the sparse resource.

# Related
- [`crate::vulkan1_0`]
- [`Extent3D`]
- [VkImageAspectFlags]()
- [VkSparseImageFormatFlags]()
- [`SparseImageFormatProperties2`]
- [`SparseImageMemoryRequirements`]
- [`get_physical_device_sparse_image_format_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        