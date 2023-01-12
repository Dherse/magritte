[VkPhysicalDeviceTextureCompressionASTCHDRFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html) - Structure describing ASTC HDR features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceTextureCompressionAstchdrFeatures`] structure is
defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           textureCompressionASTC_HDR;
} VkPhysicalDeviceTextureCompressionASTCHDRFeatures;
```
or the equivalent
```c
// Provided by VK_EXT_texture_compression_astc_hdr
typedef VkPhysicalDeviceTextureCompressionASTCHDRFeatures VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT;
```

# Members
This structure describes the following feature:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`texture_compression_astc_hdr`] indicates whether all of the ASTC HDR compressed texture formats are supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must**  be supported in `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK` To query for additional properties, or if the feature is not enabled, [`get_physical_device_format_properties`] and [`get_physical_device_image_format_properties`] **can**  be used to check for supported properties of individual formats as normal.
If the [`PhysicalDeviceTextureCompressionAstchdrFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceTextureCompressionAstchdrFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES`

# Related
- [`ext_texture_compression_astc_hdr`]
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        