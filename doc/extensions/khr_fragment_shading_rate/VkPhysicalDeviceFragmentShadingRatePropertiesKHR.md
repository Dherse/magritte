[VkPhysicalDeviceFragmentShadingRatePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html) - Structure describing variable fragment shading rate limits that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFragmentShadingRatePropertiesKHR`] structure is
defined as:
```c
// Provided by VK_KHR_fragment_shading_rate
typedef struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
    VkStructureType          sType;
    void*                    pNext;
    VkExtent2D               minFragmentShadingRateAttachmentTexelSize;
    VkExtent2D               maxFragmentShadingRateAttachmentTexelSize;
    uint32_t                 maxFragmentShadingRateAttachmentTexelSizeAspectRatio;
    VkBool32                 primitiveFragmentShadingRateWithMultipleViewports;
    VkBool32                 layeredShadingRateAttachments;
    VkBool32                 fragmentShadingRateNonTrivialCombinerOps;
    VkExtent2D               maxFragmentSize;
    uint32_t                 maxFragmentSizeAspectRatio;
    uint32_t                 maxFragmentShadingRateCoverageSamples;
    VkSampleCountFlagBits    maxFragmentShadingRateRasterizationSamples;
    VkBool32                 fragmentShadingRateWithShaderDepthStencilWrites;
    VkBool32                 fragmentShadingRateWithSampleMask;
    VkBool32                 fragmentShadingRateWithShaderSampleMask;
    VkBool32                 fragmentShadingRateWithConservativeRasterization;
    VkBool32                 fragmentShadingRateWithFragmentShaderInterlock;
    VkBool32                 fragmentShadingRateWithCustomSampleLocations;
    VkBool32                 fragmentShadingRateStrictMultiplyCombiner;
} VkPhysicalDeviceFragmentShadingRatePropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`min_fragment_shading_rate_attachment_texel_size`] indicates minimum supported width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. Each value  **must**  be less than or equal to the values in [`max_fragment_shading_rate_attachment_texel_size`]. Each value  **must**  be a power-of-two. It  **must**  be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
- [`max_fragment_shading_rate_attachment_texel_size`] indicates maximum supported width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. Each value  **must**  be greater than or equal to the values in [`min_fragment_shading_rate_attachment_texel_size`]. Each value  **must**  be a power-of-two. It  **must**  be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
- [`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] indicates the maximum ratio between the width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. [`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] **must**  be a power-of-two value, and  **must**  be less than or equal to max(`maxFragmentShadingRateAttachmentTexelSize.width` / `minFragmentShadingRateAttachmentTexelSize.height`, `maxFragmentShadingRateAttachmentTexelSize.height` / `minFragmentShadingRateAttachmentTexelSize.width`). It  **must**  be 0 if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
- [`primitive_fragment_shading_rate_with_multiple_viewports`] specifies     whether the [primitive     fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive) **can**  be used when multiple viewports are used.     If this value is [`FALSE`], only a single viewport  **must**  be used,     and applications  **must**  not write to the     `ViewportMaskNV` or     `ViewportIndex` built-in when setting `PrimitiveShadingRateKHR`.     It  **must**  be [`FALSE`] if     the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,     the `[`VK_EXT_shader_viewport_index_layer`]` extension, or     the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not     supported, or if the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) feature is not supported.
- [`layered_shading_rate_attachments`] specifies whether a shading rate     attachment image view  **can**  be created with multiple layers.     If this value is [`FALSE`], when creating an image view with a     `usage` that includes     `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`,     `layerCount` **must**  be `1`.     It  **must**  be [`FALSE`] if     the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature,     the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,     the `[`VK_EXT_shader_viewport_index_layer`]` extension, or     the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not     supported, or if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
- [`fragment_shading_rate_non_trivial_combiner_ops`] specifies whether [`FragmentShadingRateCombinerOpKHR`] enums other than `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR` **can**  be used. It  **must**  be [`FALSE`] unless either the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) or [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is supported.
- [`max_fragment_size`] indicates the maximum supported width and height of a fragment. Its `width` and `height` members  **must**  both be power-of-two values. This limit is purely informational, and is not validated.
- [`max_fragment_size_aspect_ratio`] indicates the maximum ratio between the width and height of a fragment. [`max_fragment_size_aspect_ratio`] **must**  be a power-of-two value, and  **must**  be less than or equal to the maximum of the `width` and `height` members of [`max_fragment_size`]. This limit is purely informational, and is not validated.
- [`max_fragment_shading_rate_coverage_samples`] specifies the maximum number of coverage samples supported in a single fragment. [`max_fragment_shading_rate_coverage_samples`] **must**  be less than or equal to the product of the `width` and `height` members of [`max_fragment_size`], and the sample count reported by [`max_fragment_shading_rate_rasterization_samples`]. [`max_fragment_shading_rate_coverage_samples`] **must**  be less than or equal to `maxSampleMaskWords` × 32 if [`fragment_shading_rate_with_shader_sample_mask`] is supported. This limit is purely informational, and is not validated.
- [`max_fragment_shading_rate_rasterization_samples`] is a [`SampleCountFlagBits`] value specifying the maximum sample rate supported when a fragment covers multiple pixels. This limit is purely informational, and is not validated.
- [`fragment_shading_rate_with_shader_depth_stencil_writes`] specifies whether the implementation supports writing `FragDepth` or `FragStencilRefEXT` from a fragment shader for multi-pixel fragments. If this value is [`FALSE`], writing to those built-ins will clamp the fragment shading rate to (1,1).
- [`fragment_shading_rate_with_sample_mask`] specifies whether the the implementation supports setting valid bits of [`PipelineMultisampleStateCreateInfo::sample_mask`] to `0` for multi-pixel fragments. If this value is [`FALSE`], zeroing valid bits in the sample mask will clamp the fragment shading rate to (1,1).
- [`fragment_shading_rate_with_shader_sample_mask`] specifies whether the implementation supports reading or writing [`SampleMask`] for multi-pixel fragments. If this value is [`FALSE`], using that built-in will clamp the fragment shading rate to (1,1).
- [`fragment_shading_rate_with_conservative_rasterization`] specifies whether [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) is supported for multi-pixel fragments. It  **must**  be [`FALSE`] if `[`VK_EXT_conservative_rasterization`]` is not supported. If this value is [`FALSE`], using [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) will clamp the fragment shading rate to (1,1).
- [`fragment_shading_rate_with_fragment_shader_interlock`] specifies whether [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) is supported for multi-pixel fragments. It  **must**  be [`FALSE`] if `[`VK_EXT_fragment_shader_interlock`]` is not supported. If this value is [`FALSE`], using [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) will clamp the fragment shading rate to (1,1).
- [`fragment_shading_rate_with_custom_sample_locations`] specifies whether [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) are supported for multi-pixel fragments. It  **must**  be [`FALSE`] if `[`VK_EXT_sample_locations`]` is not supported. If this value is [`FALSE`], using [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) will clamp the fragment shading rate to (1,1).
- [`fragment_shading_rate_strict_multiply_combiner`] specifies whether `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR` accurately performs a multiplication or not. Implementations where this value is [`FALSE`] will instead combine rates with an addition. If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`FALSE`], implementations  **must**  report this as [`FALSE`]. If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`TRUE`], implementations  **should**  report this as [`TRUE`].

# Description
If the [`PhysicalDeviceFragmentShadingRatePropertiesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.These properties are related to [fragment
shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate).
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`

# Related
- [`VK_KHR_fragment_shading_rate`]
- [`Bool32`]
- [`Extent2D`]
- [`SampleCountFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        