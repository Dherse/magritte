[VkSamplerCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateInfo.html) - Structure specifying parameters of a newly created sampler

# C Specifications
The [`SamplerCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSamplerCreateInfo {
    VkStructureType         sType;
    const void*             pNext;
    VkSamplerCreateFlags    flags;
    VkFilter                magFilter;
    VkFilter                minFilter;
    VkSamplerMipmapMode     mipmapMode;
    VkSamplerAddressMode    addressModeU;
    VkSamplerAddressMode    addressModeV;
    VkSamplerAddressMode    addressModeW;
    float                   mipLodBias;
    VkBool32                anisotropyEnable;
    float                   maxAnisotropy;
    VkBool32                compareEnable;
    VkCompareOp             compareOp;
    float                   minLod;
    float                   maxLod;
    VkBorderColor           borderColor;
    VkBool32                unnormalizedCoordinates;
} VkSamplerCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`SamplerCreateFlagBits`] describing additional parameters of the sampler.
- [`mag_filter`] is a [`Filter`] value specifying the magnification filter to apply to lookups.
- [`min_filter`] is a [`Filter`] value specifying the minification filter to apply to lookups.
- [`mipmap_mode`] is a [`SamplerMipmapMode`] value specifying the mipmap filter to apply to lookups.
- [`address_mode_u`] is a [`SamplerAddressMode`] value specifying the addressing mode for U coordinates outside [0,1).
- [`address_mode_v`] is a [`SamplerAddressMode`] value specifying the addressing mode for V coordinates outside [0,1).
- [`address_mode_w`] is a [`SamplerAddressMode`] value specifying the addressing mode for W coordinates outside [0,1).
- [`mip_lod_bias`] is the bias to be added to mipmap LOD (level-of-detail) calculation and bias provided by image sampling functions in SPIR-V, as described in the [Level-of-Detail Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-level-of-detail-operation) section.
- [`anisotropy_enable`] is `VK_TRUE` to enable anisotropic filtering, as described in the [Texel Anisotropic Filtering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-texel-anisotropic-filtering) section, or `VK_FALSE` otherwise.
- [`max_anisotropy`] is the anisotropy value clamp used by the sampler when [`anisotropy_enable`] is `VK_TRUE`. If [`anisotropy_enable`] is `VK_FALSE`, [`max_anisotropy`] is ignored.
- [`compare_enable`] is `VK_TRUE` to enable comparison against a reference value during lookups, or `VK_FALSE` otherwise.  - Note: Some implementations will default to shader state if this member does not match. 
- [`compare_op`] is a [`CompareOp`] value specifying the comparison function to apply to fetched data before filtering as described in the [Depth Compare Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-depth-compare-operation) section.
- [`min_lod`] is used to clamp the [minimum of the computed LOD value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-level-of-detail-operation).
- [`max_lod`] is used to clamp the [maximum of the computed LOD value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-level-of-detail-operation). To avoid clamping the maximum value, set [`max_lod`] to the constant `VK_LOD_CLAMP_NONE`.
- [`border_color`] is a [`BorderColor`] value specifying the predefined border color to use.
- [`unnormalized_coordinates`] controls whether to use unnormalized or normalized texel coordinates to address texels of the image. When set to `VK_TRUE`, the range of the image coordinates used to lookup the texel is in the range of zero to the image size in each dimension. When set to `VK_FALSE` the range of image coordinates is zero to one.When [`unnormalized_coordinates`] is `VK_TRUE`, images the sampler is used with in the shader have the following requirements:  - The `viewType` **must**  be either `VK_IMAGE_VIEW_TYPE_1D` or `VK_IMAGE_VIEW_TYPE_2D`.  - The image view  **must**  have a single layer and a single mip level. When [`unnormalized_coordinates`] is `VK_TRUE`, image built-in functions in the shader that use the sampler have the following requirements:  - The functions  **must**  not use projection.  - The functions  **must**  not use offsets.

# Description
The maximum number of sampler objects which  **can**  be simultaneously created
on a device is implementation-dependent and specified by the
[maxSamplerAllocationCount](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxSamplerAllocationCount) member of the
[`PhysicalDeviceLimits`] structure.Since [`Sampler`] is a non-dispatchable handle type, implementations
 **may**  return the same handle for sampler state vectors that are identical.
In such cases, all such objects would only count once against the
`maxSamplerAllocationCount` limit.
## Valid Usage
-    The absolute value of [`mip_lod_bias`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_sampler_lod_bias`]
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::sampler_mip_lod_bias`] is `VK_FALSE`, [`mip_lod_bias`] **must**  be zero
-  [`max_lod`] **must**  be greater than or equal to [`min_lod`]
-    If the [anisotropic sampling](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-samplerAnisotropy) feature is not enabled, [`anisotropy_enable`] **must**  be `VK_FALSE`
-    If [`anisotropy_enable`] is `VK_TRUE`, [`max_anisotropy`] **must**  be between `1.0` and [`PhysicalDeviceLimits::max_sampler_anisotropy`], inclusive
-    If [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) is enabled and the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`, [`min_filter`] and [`mag_filter`] **must**  be equal to the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion’s `chromaFilter`
-    If [`unnormalized_coordinates`] is `VK_TRUE`, [`min_filter`] and [`mag_filter`] **must**  be equal
-    If [`unnormalized_coordinates`] is `VK_TRUE`, [`mipmap_mode`] **must**  be `VK_SAMPLER_MIPMAP_MODE_NEAREST`
-    If [`unnormalized_coordinates`] is `VK_TRUE`, [`min_lod`] and [`max_lod`] **must**  be zero
-    If [`unnormalized_coordinates`] is `VK_TRUE`, [`address_mode_u`] and [`address_mode_v`] **must**  each be either `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE` or `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER`
-    If [`unnormalized_coordinates`] is `VK_TRUE`, [`anisotropy_enable`] **must**  be `VK_FALSE`
-    If [`unnormalized_coordinates`] is `VK_TRUE`, [`compare_enable`] **must**  be `VK_FALSE`
-    If any of [`address_mode_u`], [`address_mode_v`] or [`address_mode_w`] are `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER`, [`border_color`] **must**  be a valid [`BorderColor`] value
-    If [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) is enabled, [`address_mode_u`], [`address_mode_v`], and [`address_mode_w`] **must**  be `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`, [`anisotropy_enable`] **must**  be `VK_FALSE`, and [`unnormalized_coordinates`] **must**  be `VK_FALSE`
-    The sampler reduction mode  **must**  be set to `VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE` if [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) is enabled
-    If [samplerMirrorClampToEdge](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-samplerMirrorClampToEdge) is not enabled, and if the `[`khr_sampler_mirror_clamp_to_edge`]` extension is not enabled, [`address_mode_u`], [`address_mode_v`] and [`address_mode_w`] **must**  not be `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE`
-    If [`compare_enable`] is `VK_TRUE`, [`compare_op`] **must**  be a valid [`CompareOp`] value
-    If either [`mag_filter`] or [`min_filter`] is `VK_FILTER_CUBIC_EXT`, [`anisotropy_enable`] **must**  be `VK_FALSE`
-    If [`compare_enable`] is `VK_TRUE`, the `reductionMode` member of [`SamplerReductionModeCreateInfo`] **must**  be `VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE`
-    If [`flags`] includes `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`, then [`min_filter`] and [`mag_filter`] **must**  be equal
-    If [`flags`] includes `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`, then [`mipmap_mode`] **must**  be `VK_SAMPLER_MIPMAP_MODE_NEAREST`
-    If [`flags`] includes `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`, then [`min_lod`] and [`max_lod`] **must**  be zero
-    If [`flags`] includes `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`, then [`address_mode_u`] and [`address_mode_v`] **must**  each be either `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE` or `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER`
-    If [`flags`] includes `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`, then [`anisotropy_enable`] **must**  be `VK_FALSE`
-    If [`flags`] includes `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`, then [`compare_enable`] **must**  be `VK_FALSE`
-    If [`flags`] includes `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`, then [`unnormalized_coordinates`] **must**  be `VK_FALSE`
-    If [`border_color`] is one of `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT` or `VK_BORDER_COLOR_INT_CUSTOM_EXT`, then a [`SamplerCustomBorderColorCreateInfoEXT`] **must**  be included in the [`p_next`] chain
-    If the [`customBorderColors`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColors) feature is not enabled, [`border_color`] **must**  not be `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT` or `VK_BORDER_COLOR_INT_CUSTOM_EXT`
-    If [`border_color`] is one of `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT` or `VK_BORDER_COLOR_INT_CUSTOM_EXT`, and [`SamplerCustomBorderColorCreateInfoEXT::format`] is not `VK_FORMAT_UNDEFINED`, [`SamplerCustomBorderColorCreateInfoEXT::custom_border_color`] **must**  be within the range of values representable in `format`
-    The maximum number of samplers with custom border colors which  **can**  be simultaneously created on a device is implementation-dependent and specified by the [maxCustomBorderColorSamplers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxCustomBorderColorSamplers) member of the [`PhysicalDeviceCustomBorderColorPropertiesEXT`] structure

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`SamplerBorderColorComponentMappingCreateInfoEXT`], [`SamplerCustomBorderColorCreateInfoEXT`], [`SamplerReductionModeCreateInfo`], or [`SamplerYcbcrConversionInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`SamplerCreateFlagBits`] values
-  [`mag_filter`] **must**  be a valid [`Filter`] value
-  [`min_filter`] **must**  be a valid [`Filter`] value
-  [`mipmap_mode`] **must**  be a valid [`SamplerMipmapMode`] value
-  [`address_mode_u`] **must**  be a valid [`SamplerAddressMode`] value
-  [`address_mode_v`] **must**  be a valid [`SamplerAddressMode`] value
-  [`address_mode_w`] **must**  be a valid [`SamplerAddressMode`] value

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`BorderColor`]
- [`CompareOp`]
- [`Filter`]
- [`SamplerAddressMode`]
- [VkSamplerCreateFlags]()
- [`SamplerMipmapMode`]
- [`StructureType`]
- [`create_sampler`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        