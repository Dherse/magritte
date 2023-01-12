[VkFormatFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits.html) - Bitmask specifying features supported by a buffer

# C Specifications
Bits which  **can**  be set in the [`FormatProperties`] features
`linearTilingFeatures`, `optimalTilingFeatures`,
[`DrmFormatModifierPropertiesEXT::drm_format_modifier_tiling_features`],
and `bufferFeatures` are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkFormatFeatureFlagBits {
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 0x00000001,
    VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = 0x00000002,
    VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004,
    VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008,
    VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 0x00000010,
    VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020,
    VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = 0x00000040,
    VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 0x00000080,
    VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100,
    VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200,
    VK_FORMAT_FEATURE_BLIT_SRC_BIT = 0x00000400,
    VK_FORMAT_FEATURE_BLIT_DST_BIT = 0x00000800,
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_TRANSFER_SRC_BIT = 0x00004000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_TRANSFER_DST_BIT = 0x00008000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT = 0x00020000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 0x00040000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 0x00080000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 0x00100000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 0x00200000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_DISJOINT_BIT = 0x00400000,
  // Provided by VK_VERSION_1_1
    VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT = 0x00800000,
  // Provided by VK_VERSION_1_2
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT = 0x00010000,
  // Provided by VK_IMG_filter_cubic
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = 0x00002000,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_FORMAT_FEATURE_VIDEO_DECODE_OUTPUT_BIT_KHR = 0x02000000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_FORMAT_FEATURE_VIDEO_DECODE_DPB_BIT_KHR = 0x04000000,
#endif
  // Provided by VK_KHR_acceleration_structure
    VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR = 0x20000000,
  // Provided by VK_EXT_fragment_density_map
    VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT = 0x01000000,
  // Provided by VK_KHR_fragment_shading_rate
    VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x40000000,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_FORMAT_FEATURE_VIDEO_ENCODE_INPUT_BIT_KHR = 0x08000000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_FORMAT_FEATURE_VIDEO_ENCODE_DPB_BIT_KHR = 0x10000000,
#endif
  // Provided by VK_KHR_maintenance1
    VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR = VK_FORMAT_FEATURE_TRANSFER_SRC_BIT,
  // Provided by VK_KHR_maintenance1
    VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR = VK_FORMAT_FEATURE_TRANSFER_DST_BIT,
  // Provided by VK_EXT_sampler_filter_minmax
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT = VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR = VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR = VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_FORMAT_FEATURE_DISJOINT_BIT_KHR = VK_FORMAT_FEATURE_DISJOINT_BIT,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR = VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT,
  // Provided by VK_EXT_filter_cubic
    VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT = VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG,
} VkFormatFeatureFlagBits;
```

# Description
These values
all have the same meaning as the equivalently named values for
[`FormatFeatureFlags2`] and
 **may**  be set in
`linearTilingFeatures`, `optimalTilingFeatures`, and
[`DrmFormatModifierPropertiesEXT::drm_format_modifier_tiling_features`],
specifying that the features are supported by [`Image`] or
[`ImageView`]
or [`SamplerYcbcrConversion`]
created with the queried
[`get_physical_device_format_properties`]`::format`:
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image view  **can**  be [sampled from](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage).
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image view  **can**  be used as a [storage image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage).
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image view  **can**  be used as storage image that supports atomic operations.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image view  **can**  be used as a framebuffer color attachment and as an input attachment.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image view  **can**  be used as a framebuffer color attachment that supports blending and as an input attachment.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image view  **can**  be used as a framebuffer depth/stencil attachment and as an input attachment.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image  **can**  be used as `srcImage` for the [`cmd_blit_image2`] and [`cmd_blit_image`] commands.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that an image  **can**  be used as `dstImage` for the [`cmd_blit_image2`] and [`cmd_blit_image`] commands.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that if [`VK_FORMAT_FEATURE_FLAG_BITS`] is also set, an image view  **can**  be used with a sampler that has either of `magFilter` or `minFilter` set to `VK_FILTER_LINEAR`, or `mipmapMode` set to `VK_SAMPLER_MIPMAP_MODE_LINEAR`. If [`VK_FORMAT_FEATURE_FLAG_BITS`] is also set, an image can be used as the `srcImage` to [`cmd_blit_image2`] and [`cmd_blit_image`] with a `filter` of `VK_FILTER_LINEAR`. This bit  **must**  only be exposed for formats that also support the [`VK_FORMAT_FEATURE_FLAG_BITS`] or [`VK_FORMAT_FEATURE_FLAG_BITS`].If the format being queried is a depth/stencil format, this bit only specifies that the depth aspect (not the stencil aspect) of an image of this format supports linear filtering, and that linear filtering of the depth aspect is supported whether depth compare is enabled in the sampler or not. Where depth comparison is supported it  **may**  be linear filtered whether this bit is present or not, but where this bit is not present the filtered value  **may**  be computed in an implementation-dependent manner which differs from the normal rules of linear filtering. The resulting value  **must**  be in the range [0,1] and  **should**  be proportional to, or a weighted average of, the number of comparison passes or failures.
- [`TRANSFER_SRC`] specifies that an image  **can**  be used as a source image for [copy commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies).
- [`TRANSFER_DST`] specifies that an image  **can**  be used as a destination image for [copy commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies) and [clear commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears).
- [`SAMPLED_IMAGE_FILTER_MINMAX`] specifies [`Image`] **can**  be used as a sampled image with a min or max [`SamplerReductionMode`]. This bit  **must**  only be exposed for formats that also support the [`VK_FORMAT_FEATURE_FLAG_BITS`].
- [`SAMPLED_IMAGE_FILTER_CUBIC_EXT`] specifies that [`Image`] **can**  be used with a sampler that has either of `magFilter` or `minFilter` set to `VK_FILTER_CUBIC_EXT`, or be the source image for a blit with `filter` set to `VK_FILTER_CUBIC_EXT`. This bit  **must**  only be exposed for formats that also support the [`VK_FORMAT_FEATURE_FLAG_BITS`]. If the format being queried is a depth/stencil format, this only specifies that the depth aspect is cubic filterable.
- [`MIDPOINT_CHROMA_SAMPLES`] specifies that an application  **can**  define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source, and that an image of this format  **can**  be used with a [`SamplerYcbcrConversionCreateInfo`]`xChromaOffset` and/or `yChromaOffset` of `VK_CHROMA_LOCATION_MIDPOINT`. Otherwise both `xChromaOffset` and `yChromaOffset` **must**  be `VK_CHROMA_LOCATION_COSITED_EVEN`. If a format does not incorporate chroma downsampling (it is not a “422” or “420” format) but the implementation supports sampler Y′C<sub>B</sub>C<sub>R</sub> conversion for this format, the implementation  **must**  set [`MIDPOINT_CHROMA_SAMPLES`].
- [`COSITED_CHROMA_SAMPLES`] specifies that an application  **can**  define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source, and that an image of this format  **can**  be used with a [`SamplerYcbcrConversionCreateInfo`]`xChromaOffset` and/or `yChromaOffset` of `VK_CHROMA_LOCATION_COSITED_EVEN`. Otherwise both `xChromaOffset` and `yChromaOffset` **must**  be `VK_CHROMA_LOCATION_MIDPOINT`. If neither [`COSITED_CHROMA_SAMPLES`] nor [`MIDPOINT_CHROMA_SAMPLES`] is set, the application  **must**  not define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source.
- [`SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER`] specifies that an application  **can**  define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source with `chromaFilter` set to `VK_FILTER_LINEAR`.
- [`SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER`] specifies that the format can have different chroma, min, and mag filters.
- [`SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT`] specifies that reconstruction is explicit, as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction). If this bit is not present, reconstruction is implicit by default.
- [`SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE`] specifies that reconstruction  **can**  be forcibly made explicit by setting [`SamplerYcbcrConversionCreateInfo::force_explicit_reconstruction`] to `VK_TRUE`. If the format being queried supports [`SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT`] it  **must**  also support [`SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE`].
- [`DISJOINT`] specifies that a multi-planar image  **can**  have the `VK_IMAGE_CREATE_DISJOINT_BIT` set during image creation. An implementation  **must**  not set [`DISJOINT`] for *single-plane formats*.
- [`FRAGMENT_DENSITY_MAP_EXT`] specifies that an image view  **can**  be used as a [fragment density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment).
- [`FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`] specifies that an image view  **can**  be used as a [fragment shading rate attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment). An implementation  **must**  not set this feature for formats with numeric type other than `*UINT`, or set it as a buffer feature.
- [`VIDEO_DECODE_OUTPUT_KHR`] specifies that an image view with this format  **can**  be used as an output for [video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations)
- [`VIDEO_DECODE_DPB_KHR`] specifies that an image view with this format  **can**  be used as a DPB for [video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations)
- [`VIDEO_ENCODE_INPUT_KHR`] specifies that an image view with this format  **can**  be used as an input to [video encode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations)
- [`VIDEO_ENCODE_DPB_KHR`] specifies that an image view with this format  **can**  be used as a DPB for [video encode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations)
The following bits  **may**  be set in `bufferFeatures`, specifying that the
features are supported by [`Buffer`] or [`BufferView`] created with the queried
[`get_physical_device_format_properties`]`::format`:
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that the format  **can**  be used to create a buffer view that  **can**  be bound to a `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` descriptor.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that the format  **can**  be used to create a buffer view that  **can**  be bound to a `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` descriptor.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that atomic operations are supported on `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` with this format.
- [`VK_FORMAT_FEATURE_FLAG_BITS`] specifies that the format  **can**  be used as a vertex attribute format ([`VertexInputAttributeDescription::format`]).
- [`ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR`] specifies that the format  **can**  be used as the vertex format when creating an [acceleration structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure) ([`AccelerationStructureGeometryTrianglesDataKHR::vertex_format`]). This format  **can**  also be used as the vertex format in host memory when doing [host acceleration structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#host-acceleration-structure) builds.

# Related
- [`crate::vulkan1_0`]
- [VkFormatFeatureFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        