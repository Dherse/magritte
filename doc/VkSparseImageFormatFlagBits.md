[VkSparseImageFormatFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatFlagBits.html) - Bitmask specifying additional information about a sparse image resource

# C Specifications
Bits which  **may**  be set in [`SparseImageFormatProperties::flags`],
specifying additional information about the sparse resource, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSparseImageFormatFlagBits {
    VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 0x00000001,
    VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 0x00000002,
    VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004,
} VkSparseImageFormatFlagBits;
```

# Description
- [`SINGLE_MIPTAIL`] specifies that the image uses a single mip tail region for all array layers.
- [`ALIGNED_MIP_SIZE`] specifies that the first mip level whose dimensions are not integer multiples of the corresponding dimensions of the sparse image block begins the mip tail region.
- [`NONSTANDARD_BLOCK_SIZE`] specifies that the image uses non-standard sparse image block dimensions, and the `imageGranularity` values do not match the standard sparse image block dimensions for the given format.

# Related
- [`crate::vulkan1_0`]
- [`SparseImageFormatFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        