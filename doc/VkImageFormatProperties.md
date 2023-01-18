[VkImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties.html) - Structure specifying an image format properties

# C Specifications
The [`ImageFormatProperties`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkImageFormatProperties {
    VkExtent3D            maxExtent;
    uint32_t              maxMipLevels;
    uint32_t              maxArrayLayers;
    VkSampleCountFlags    sampleCounts;
    VkDeviceSize          maxResourceSize;
} VkImageFormatProperties;
```

# Members
- [`max_extent`] are the maximum image dimensions. See the [Allowed Extent Values](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-extentperimagetype) section below for how these values are constrained by `type`.
- [`max_mip_levels`] is the maximum number of mipmap levels. [`max_mip_levels`] **must**  be equal to the number of levels in the complete mipmap chain based on the `maxExtent.width`, `maxExtent.height`, and `maxExtent.depth`, except when one of the following conditions is true, in which case it  **may**  instead be `1`:  - [`get_physical_device_image_format_properties`]`::tiling` was `VK_IMAGE_TILING_LINEAR`  - [`PhysicalDeviceImageFormatInfo2::tiling`] was `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`  - the [`PhysicalDeviceImageFormatInfo2::p_next`] chain included a [`PhysicalDeviceExternalImageFormatInfo`] structure with a handle type included in the `handleTypes` member for which mipmap image support is not required  - image `format` is one of the [formats that require a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)  - `flags` contains `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT` 
- [`max_array_layers`] is the maximum number of array layers. [`max_array_layers`] **must**  be no less than [`PhysicalDeviceLimits::max_image_array_layers`], except when one of the following conditions is true, in which case it  **may**  instead be `1`:  - `tiling` is `VK_IMAGE_TILING_LINEAR`  - `tiling` is `VK_IMAGE_TILING_OPTIMAL` and `type` is `VK_IMAGE_TYPE_3D`  - `format` is one of the [formats that require a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) 
- If `tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then [`max_array_layers`] **must**  not be 0.
- [`sample_counts`] is a bitmask of [`SampleCountFlagBits`] specifying all the supported sample counts for this image as described [below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-supported-sample-counts).
- [`max_resource_size`] is an upper bound on the total image size in bytes, inclusive of all image subresources. Implementations  **may**  have an address space limit on total size of a resource, which is advertised by this property. [`max_resource_size`] **must**  be at least 2<sup>31</sup>.

# Description
If the combination of parameters to
[`get_physical_device_image_format_properties`] is not supported by the
implementation for use in [`create_image`], then all members of
[`ImageFormatProperties`] will be filled with zero.

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [`Extent3D`]
- [`ExternalImageFormatPropertiesNV`]
- [`ImageFormatProperties2`]
- [`SampleCountFlags`]
- [`get_physical_device_image_format_properties`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        