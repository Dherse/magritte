[VkPhysicalDeviceSparseProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseProperties.html) - Structure specifying physical device sparse memory properties

# C Specifications
The [`PhysicalDeviceSparseProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPhysicalDeviceSparseProperties {
    VkBool32    residencyStandard2DBlockShape;
    VkBool32    residencyStandard2DMultisampleBlockShape;
    VkBool32    residencyStandard3DBlockShape;
    VkBool32    residencyAlignedMipSize;
    VkBool32    residencyNonResidentStrict;
} VkPhysicalDeviceSparseProperties;
```

# Members
- [`residency_standard2_d_block_shape`] is `VK_TRUE` if the physical device will access all single-sample 2D sparse resources using the standard sparse image block shapes (based on image format), as described in the [Standard Sparse Image Block Shapes (Single Sample)](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseblockshapessingle) table. If this property is not supported the value returned in the `imageGranularity` member of the [`SparseImageFormatProperties`] structure for single-sample 2D images is not  **required**  to match the standard sparse image block dimensions listed in the table.
- [`residency_standard2_d_multisample_block_shape`] is `VK_TRUE` if the physical device will access all multisample 2D sparse resources using the standard sparse image block shapes (based on image format), as described in the [Standard Sparse Image Block Shapes (MSAA)](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseblockshapesmsaa) table. If this property is not supported, the value returned in the `imageGranularity` member of the [`SparseImageFormatProperties`] structure for multisample 2D images is not  **required**  to match the standard sparse image block dimensions listed in the table.
- [`residency_standard3_d_block_shape`] is `VK_TRUE` if the physical device will access all 3D sparse resources using the standard sparse image block shapes (based on image format), as described in the [Standard Sparse Image Block Shapes (Single Sample)](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseblockshapessingle) table. If this property is not supported, the value returned in the `imageGranularity` member of the [`SparseImageFormatProperties`] structure for 3D images is not  **required**  to match the standard sparse image block dimensions listed in the table.
- [`residency_aligned_mip_size`] is `VK_TRUE` if images with mip level dimensions that are not integer multiples of the corresponding dimensions of the sparse image block  **may**  be placed in the mip tail. If this property is not reported, only mip levels with dimensions smaller than the `imageGranularity` member of the [`SparseImageFormatProperties`] structure will be placed in the mip tail. If this property is reported the implementation is allowed to return `VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT` in the `flags` member of [`SparseImageFormatProperties`], indicating that mip level dimensions that are not integer multiples of the corresponding dimensions of the sparse image block will be placed in the mip tail.
- [`residency_non_resident_strict`] specifies whether the physical device  **can**  consistently access non-resident regions of a resource. If this property is `VK_TRUE`, access to non-resident regions of resources will be guaranteed to return values as if the resource was populated with 0; writes to non-resident regions will be discarded.

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`PhysicalDeviceProperties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        