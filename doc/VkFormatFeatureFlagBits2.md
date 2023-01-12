[VkFormatFeatureFlagBits2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatFeatureFlagBits2.html) - Bitmask specifying features supported by a buffer

# C Specifications
Bits which  **can**  be set in the [`FormatProperties3`] features
`linearTilingFeatures`, `optimalTilingFeatures`, and
`bufferFeatures` are:
```c
// Provided by VK_VERSION_1_3
// Flag bits for VkFormatFeatureFlagBits2
typedef VkFlags64 VkFormatFeatureFlagBits2;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT = 0x00000001ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR = 0x00000001ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT = 0x00000002ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR = 0x00000002ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR = 0x00000004ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR = 0x00000008ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT = 0x00000010ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR = 0x00000010ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR = 0x00000020ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT = 0x00000040ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR = 0x00000040ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT = 0x00000080ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR = 0x00000080ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR = 0x00000100ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR = 0x00000200ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_BLIT_SRC_BIT = 0x00000400ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR = 0x00000400ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_BLIT_DST_BIT = 0x00000800ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR = 0x00000800ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR = 0x00001000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT = 0x00002000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT = 0x00002000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT = 0x00004000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR = 0x00004000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT = 0x00008000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR = 0x00008000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT = 0x00010000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR = 0x00010000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT = 0x00020000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR = 0x00020000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 0x00040000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR = 0x00040000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 0x00080000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR = 0x00080000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 0x00100000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR = 0x00100000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 0x00200000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR = 0x00200000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_DISJOINT_BIT = 0x00400000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR = 0x00400000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT = 0x00800000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR = 0x00800000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT = 0x80000000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR = 0x80000000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT = 0x100000000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR = 0x100000000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT = 0x200000000ULL;
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR = 0x200000000ULL;
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_format_feature_flags2 with VK_KHR_video_decode_queue
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR = 0x02000000ULL;
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_format_feature_flags2 with VK_KHR_video_decode_queue
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR = 0x04000000ULL;
#endif
// Provided by VK_KHR_acceleration_structure with VK_KHR_format_feature_flags2
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR = 0x20000000ULL;
// Provided by VK_KHR_format_feature_flags2 with VK_EXT_fragment_density_map
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT = 0x01000000ULL;
// Provided by VK_KHR_format_feature_flags2 with VK_KHR_fragment_shading_rate
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x40000000ULL;
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_format_feature_flags2 with VK_KHR_video_encode_queue
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR = 0x08000000ULL;
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_format_feature_flags2 with VK_KHR_video_encode_queue
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR = 0x10000000ULL;
#endif
// Provided by VK_KHR_format_feature_flags2 with VK_NV_linear_color_attachment
static const VkFormatFeatureFlagBits2 VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV = 0x4000000000ULL;
```
or the equivalent
```c
// Provided by VK_KHR_format_feature_flags2
typedef VkFormatFeatureFlagBits2 VkFormatFeatureFlagBits2KHR;
```

# Description
The following bits  **may**  be set in `linearTilingFeatures` and
`optimalTilingFeatures`, specifying that the features are supported by
[`Image`] or [`ImageView`]
or [`SamplerYcbcrConversion`]
created with the queried
[`get_physical_device_format_properties2`]`::format`:
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image view  **can**  be [sampled from](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage).
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image view  **can**  be used as a [storage image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage).
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image view  **can**  be used as storage image that supports atomic operations.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image view  **can**  be used as a framebuffer color attachment and as an input attachment.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image view  **can**  be used as a framebuffer color attachment that supports blending and as an input attachment.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image view  **can**  be used as a framebuffer depth/stencil attachment and as an input attachment.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image  **can**  be     used as the `srcImage` for [`cmd_blit_image2`] and     [`cmd_blit_image`].
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image  **can**  be     used as the `dstImage` for [`cmd_blit_image2`] and     [`cmd_blit_image`].
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that     if [`VK_FORMAT_FEATURE_FLAG_BITS2`] is also set, an image     view  **can**  be used with a sampler that has either of `magFilter` or     `minFilter` set to `VK_FILTER_LINEAR`, or `mipmapMode` set     to `VK_SAMPLER_MIPMAP_MODE_LINEAR`.     If [`VK_FORMAT_FEATURE_FLAG_BITS2`] is also set, an image can be     used as the `srcImage` for [`cmd_blit_image2`] and     [`cmd_blit_image`] with a `filter` of `VK_FILTER_LINEAR`.     This bit  **must**  only be exposed for formats that also support the     [`VK_FORMAT_FEATURE_FLAG_BITS2`] or     [`VK_FORMAT_FEATURE_FLAG_BITS2`].If the format being queried is a depth/stencil format, this bit only specifies that the depth aspect (not the stencil aspect) of an image of this format supports linear filtering. Where depth comparison is supported it  **may**  be linear filtered whether this bit is present or not, but where this bit is not present the filtered value  **may**  be computed in an implementation-dependent manner which differs from the normal rules of linear filtering. The resulting value  **must**  be in the range [0,1] and  **should**  be proportional to, or a weighted average of, the number of comparison passes or failures.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image  **can**  be used as a source image for [copy commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies).
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an image  **can**  be used as a destination image for [copy commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies) and [clear commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears).
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies [`Image`] **can**  be used as a sampled image with a min or max [`SamplerReductionMode`]. This bit  **must**  only be exposed for formats that also support the [`VK_FORMAT_FEATURE_FLAG_BITS2`].
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that [`Image`] **can**  be used with a sampler that has either of `magFilter` or `minFilter` set to `VK_FILTER_CUBIC_EXT`, or be the source image for a blit with `filter` set to `VK_FILTER_CUBIC_EXT`. This bit  **must**  only be exposed for formats that also support the [`VK_FORMAT_FEATURE_FLAG_BITS2`]. If the format being queried is a depth/stencil format, this only specifies that the depth aspect is cubic filterable.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an application  **can**  define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source, and that an image of this format  **can**  be used with a [`SamplerYcbcrConversionCreateInfo`]`xChromaOffset` and/or `yChromaOffset` of `VK_CHROMA_LOCATION_MIDPOINT`. Otherwise both `xChromaOffset` and `yChromaOffset` **must**  be `VK_CHROMA_LOCATION_COSITED_EVEN`. If a format does not incorporate chroma downsampling (it is not a “422” or “420” format) but the implementation supports sampler Y′C<sub>B</sub>C<sub>R</sub> conversion for this format, the implementation  **must**  set [`VK_FORMAT_FEATURE_FLAG_BITS2`].
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an application  **can**  define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source, and that an image of this format  **can**  be used with a [`SamplerYcbcrConversionCreateInfo`]`xChromaOffset` and/or `yChromaOffset` of `VK_CHROMA_LOCATION_COSITED_EVEN`. Otherwise both `xChromaOffset` and `yChromaOffset` **must**  be `VK_CHROMA_LOCATION_MIDPOINT`. If neither [`VK_FORMAT_FEATURE_FLAG_BITS2`] nor [`VK_FORMAT_FEATURE_FLAG_BITS2`] is set, the application  **must**  not define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that an application  **can**  define a [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion) using this format as a source with `chromaFilter` set to `VK_FILTER_LINEAR`.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that the format can have different chroma, min, and mag filters.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that reconstruction is explicit, as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction). If this bit is not present, reconstruction is implicit by default.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that reconstruction  **can**  be forcibly made explicit by setting [`SamplerYcbcrConversionCreateInfo::force_explicit_reconstruction`] to `VK_TRUE`. If the format being queried supports [`VK_FORMAT_FEATURE_FLAG_BITS2`] it  **must**  also support [`VK_FORMAT_FEATURE_FLAG_BITS2`].
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that a multi-planar image  **can**  have the `VK_IMAGE_CREATE_DISJOINT_BIT` set during image creation. An implementation  **must**  not set [`VK_FORMAT_FEATURE_FLAG_BITS2`] for *single-plane formats*.
- [`FORMAT_FEATURE2_FRAGMENT_DENSITY_MAP_EXT`] specifies that an image view  **can**  be used as a [fragment density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment).
- [`FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`] specifies that an image view  **can**  be used as a [fragment shading rate attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment). An implementation  **must**  not set this feature for formats with numeric type other than `*UINT`, or set it as a buffer feature.
- [`FORMAT_FEATURE2_VIDEO_DECODE_OUTPUT_KHR`] specifies that an image view with this format  **can**  be used as an output for [video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations)
- [`FORMAT_FEATURE2_VIDEO_DECODE_DPB_KHR`] specifies that an image view with this format  **can**  be used as a DPB for [video decode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations)
- [`FORMAT_FEATURE2_VIDEO_ENCODE_INPUT_KHR`] specifies that an image view with this format  **can**  be used as an input to [video encode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations)
- [`FORMAT_FEATURE2_VIDEO_ENCODE_DPB_KHR`] specifies that an image view with this format  **can**  be used as a DPB for [video encode operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations)
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that image views created with this format  **can**  be used as [storage images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage) for read operations without specifying a format.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that image views created with this format  **can**  be used as [storage images](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage) for write operations without specifying a format.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that image views created with this format  **can**  be used for depth comparison performed by `OpImage*Dref*` instructions.
- [`FORMAT_FEATURE2_LINEAR_COLOR_ATTACHMENT_NV`] specifies that    the format is supported as a renderable [Linear Color    Attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary).    This bit will be set for renderable color formats in the    `linearTilingFeatures`. This  **must**  not be set in the `optimalTilingFeatures` or `bufferFeatures` members.
The following bits  **may**  be set in `bufferFeatures`, specifying that the
features are supported by [`Buffer`] or [`BufferView`] created with the queried
[`get_physical_device_format_properties2`]`::format`:
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that the format  **can**  be used to create a buffer view that  **can**  be bound to a `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` descriptor.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that the format  **can**  be used to create a buffer view that  **can**  be bound to a `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` descriptor.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that atomic operations are supported on `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` with this format.
- [`VK_FORMAT_FEATURE_FLAG_BITS2`] specifies that the format  **can**  be used as a vertex attribute format ([`VertexInputAttributeDescription::format`]).
- [`FORMAT_FEATURE2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR`] specifies that the format  **can**  be used as the vertex format when creating an [acceleration structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure) ([`AccelerationStructureGeometryTrianglesDataKHR::vertex_format`]). This format  **can**  also be used as the vertex format in host memory when doing [host acceleration structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#host-acceleration-structure) builds.

# Related
- [`khr_format_feature_flags2`]
- [`crate::vulkan1_3`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        