[VkPhysicalDeviceFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures.html) - Structure describing the fine-grained features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPhysicalDeviceFeatures {
    VkBool32    robustBufferAccess;
    VkBool32    fullDrawIndexUint32;
    VkBool32    imageCubeArray;
    VkBool32    independentBlend;
    VkBool32    geometryShader;
    VkBool32    tessellationShader;
    VkBool32    sampleRateShading;
    VkBool32    dualSrcBlend;
    VkBool32    logicOp;
    VkBool32    multiDrawIndirect;
    VkBool32    drawIndirectFirstInstance;
    VkBool32    depthClamp;
    VkBool32    depthBiasClamp;
    VkBool32    fillModeNonSolid;
    VkBool32    depthBounds;
    VkBool32    wideLines;
    VkBool32    largePoints;
    VkBool32    alphaToOne;
    VkBool32    multiViewport;
    VkBool32    samplerAnisotropy;
    VkBool32    textureCompressionETC2;
    VkBool32    textureCompressionASTC_LDR;
    VkBool32    textureCompressionBC;
    VkBool32    occlusionQueryPrecise;
    VkBool32    pipelineStatisticsQuery;
    VkBool32    vertexPipelineStoresAndAtomics;
    VkBool32    fragmentStoresAndAtomics;
    VkBool32    shaderTessellationAndGeometryPointSize;
    VkBool32    shaderImageGatherExtended;
    VkBool32    shaderStorageImageExtendedFormats;
    VkBool32    shaderStorageImageMultisample;
    VkBool32    shaderStorageImageReadWithoutFormat;
    VkBool32    shaderStorageImageWriteWithoutFormat;
    VkBool32    shaderUniformBufferArrayDynamicIndexing;
    VkBool32    shaderSampledImageArrayDynamicIndexing;
    VkBool32    shaderStorageBufferArrayDynamicIndexing;
    VkBool32    shaderStorageImageArrayDynamicIndexing;
    VkBool32    shaderClipDistance;
    VkBool32    shaderCullDistance;
    VkBool32    shaderFloat64;
    VkBool32    shaderInt64;
    VkBool32    shaderInt16;
    VkBool32    shaderResourceResidency;
    VkBool32    shaderResourceMinLod;
    VkBool32    sparseBinding;
    VkBool32    sparseResidencyBuffer;
    VkBool32    sparseResidencyImage2D;
    VkBool32    sparseResidencyImage3D;
    VkBool32    sparseResidency2Samples;
    VkBool32    sparseResidency4Samples;
    VkBool32    sparseResidency8Samples;
    VkBool32    sparseResidency16Samples;
    VkBool32    sparseResidencyAliased;
    VkBool32    variableMultisampleRate;
    VkBool32    inheritedQueries;
} VkPhysicalDeviceFeatures;
```

# Members
This structure describes the following features:

# Description
- [`robust_buffer_access`] specifies that accesses to buffers are bounds-checked against the range of the buffer descriptor (as determined by [`DescriptorBufferInfo::range`], [`BufferViewCreateInfo::range`], or the size of the buffer). Out of bounds accesses  **must**  not cause application termination, and the effects of shader loads, stores, and atomics  **must**  conform to an implementation-dependent behavior as described below.  - A buffer access is considered to be out of bounds if any of the following are true:   - The pointer was formed by `OpImageTexelPointer` and the coordinate is less than zero or greater than or equal to the number of whole elements in the bound range.   - The pointer was not formed by `OpImageTexelPointer` and the object pointed to is not wholly contained within the bound range. This includes accesses performed via *variable pointers* where the buffer descriptor being accessed cannot be statically determined. Uninitialized pointers and pointers equal to `OpConstantNull` are treated as pointing to a zero-sized object, so all accesses through such pointers are considered to be out of bounds. Buffer accesses through buffer device addresses are not bounds-checked. If the [`cooperativeMatrixRobustBufferAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-cooperativeMatrixRobustBufferAccess) feature is not enabled, then accesses using `OpCooperativeMatrixLoadNV` and `OpCooperativeMatrixStoreNV` **may**  not be bounds-checked.   - If [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is not enabled and any buffer access is determined to be out of bounds, then any other access of the same type (load, store, or atomic) to the same buffer that accesses an address less than 16 bytes away from the out of bounds address  **may**  also be considered out of bounds.   - If the access is a load that reads from the same memory locations as a prior store in the same shader invocation, with no other intervening accesses to the same memory locations in that shader invocation, then the result of the load  **may**  be the value stored by the store instruction, even if the access is out of bounds. If the load is `Volatile`, then an out of bounds load  **must**  return the appropriate out of bounds value.   - Accesses to descriptors written with a [`crate::Handle::null`] resource or view are not considered to be out of bounds. Instead, each type of descriptor access defines a specific behavior for accesses to a null descriptor.  - Out-of-bounds buffer loads will return any of the following values:   - If the access is to a uniform buffer and [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled, loads of offsets between the end of the descriptor range and the end of the descriptor range rounded up to a multiple of [robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment) bytes  **must**  return either zero values or the contents of the memory at the offset being loaded. Loads of offsets past the descriptor range rounded up to a multiple of [robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment) bytes  **must**  return zero values.   - If the access is to a storage buffer and [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled, loads of offsets between the end of the descriptor range and the end of the descriptor range rounded up to a multiple of [robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment) bytes  **must**  return either zero values or the contents of the memory at the offset being loaded. Loads of offsets past the descriptor range rounded up to a multiple of [robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment) bytes  **must**  return zero values. Similarly, stores to addresses between the end of the descriptor range and the end of the descriptor range rounded up to a multiple of [robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment) bytes  **may**  be discarded.   - Non-atomic accesses to storage buffers that are a multiple of 32 bits  **may**  be decomposed into 32-bit accesses that are individually bounds-checked.   - If the access is to an index buffer and [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled, zero values  **must**  be returned.   - If the access is to a uniform texel buffer or storage texel buffer and [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled, zero values  **must**  be returned, and then [Conversion to RGBA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-conversion-to-rgba) is applied based on the buffer view’s format.   - Values from anywhere within the memory range(s) bound to the buffer (possibly including bytes of memory past the end of the buffer, up to the end of the bound range).   - Zero values, or (0,0,0,x) vectors for vector reads where x is a valid value represented in the type of the vector components and  **may**  be any of:    - 0, 1, or the maximum representable positive integer value, for signed or unsigned integer components    - 0.0 or 1.0, for floating-point components    - Out-of-bounds writes  **may**  modify values within the memory range(s) bound to the buffer, but  **must**  not modify any other memory.   - If [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled, out of bounds writes  **must**  not modify any memory.   - Out-of-bounds atomics  **may**  modify values within the memory range(s) bound to the buffer, but  **must**  not modify any other memory, and return an undefined value.   - If [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled, out of bounds atomics  **must**  not modify any memory, and return an undefined value.   - If [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is disabled, vertex input attributes are considered out of bounds if the offset of the attribute in the bound vertex buffer range plus the size of the attribute is greater than either:   - `vertexBufferRangeSize`, if `bindingStride` == 0; or   - (`vertexBufferRangeSize` - (`vertexBufferRangeSize` % `bindingStride`)) where `vertexBufferRangeSize` is the byte size of the memory range bound to the vertex buffer binding and `bindingStride` is the byte stride of the corresponding vertex input binding. Further, if any vertex input attribute using a specific vertex input binding is out of bounds, then all vertex input attributes using that vertex input binding for that vertex shader invocation are considered out of bounds.   - If a vertex input attribute is out of bounds, it will be assigned one of the following values:    - Values from anywhere within the memory range(s) bound to the buffer, converted according to the format of the attribute.    - Zero values, format converted according to the format of the attribute.    - Zero values, or (0,0,0,x) vectors, as described above.    - If [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled, vertex input attributes are considered out of bounds if the offset of the attribute in the bound vertex buffer range plus the size of the attribute is greater than the byte size of the memory range bound to the vertex buffer binding.   - If a vertex input attribute is out of bounds, the [raw data](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input-extraction) extracted are zero values, and missing G, B, or A components are [filled with (0,0,1)](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#fxvertex-input-extraction).   - If [`robust_buffer_access`] is not enabled, applications  **must**  not perform out of bounds accesses. 
- [`full_draw_index_uint32`] specifies the full 32-bit range of indices is supported for indexed draw calls when using a [`IndexType`] of `VK_INDEX_TYPE_UINT32`. `maxDrawIndexedIndexValue` is the maximum index value that  **may**  be used (aside from the primitive restart index, which is always 2<sup>32</sup>-1 when the [`IndexType`] is `VK_INDEX_TYPE_UINT32`). If this feature is supported, `maxDrawIndexedIndexValue` **must**  be 2<sup>32</sup>-1; otherwise it  **must**  be no smaller than 2<sup>24</sup>-1. See [maxDrawIndexedIndexValue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxDrawIndexedIndexValue).
- [`image_cube_array`] specifies whether image views with a [`ImageViewType`] of `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY` **can**  be created, and that the corresponding `SampledCubeArray` and `ImageCubeArray` SPIR-V capabilities  **can**  be used in shader code.
- [`independent_blend`] specifies whether the [`PipelineColorBlendAttachmentState`] settings are controlled independently per-attachment. If this feature is not enabled, the [`PipelineColorBlendAttachmentState`] settings for all color attachments  **must**  be identical. Otherwise, a different [`PipelineColorBlendAttachmentState`] **can**  be provided for each bound color attachment.
- [`geometry_shader`] specifies whether geometry shaders are supported. If this feature is not enabled, the `VK_SHADER_STAGE_GEOMETRY_BIT` and `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT` enum values  **must**  not be used. This also specifies whether shader modules  **can**  declare the `Geometry` capability.
- [`tessellation_shader`] specifies whether tessellation control and evaluation shaders are supported. If this feature is not enabled, the `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`, `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`, `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT`, `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`, and `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO` enum values  **must**  not be used. This also specifies whether shader modules  **can**  declare the `Tessellation` capability.
- [`sample_rate_shading`] specifies whether [Sample Shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-sampleshading) and multisample interpolation are supported. If this feature is not enabled, the `sampleShadingEnable` member of the [`PipelineMultisampleStateCreateInfo`] structure  **must**  be set to [`FALSE`] and the `minSampleShading` member is ignored. This also specifies whether shader modules  **can**  declare the `SampleRateShading` capability.
- [`dual_src_blend`] specifies whether blend operations which take two sources are supported. If this feature is not enabled, the `VK_BLEND_FACTOR_SRC1_COLOR`, `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`, `VK_BLEND_FACTOR_SRC1_ALPHA`, and `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA` enum values  **must**  not be used as source or destination blending factors. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-dsb](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-dsb).
- [`logic_op`] specifies whether logic operations are supported. If this feature is not enabled, the `logicOpEnable` member of the [`PipelineColorBlendStateCreateInfo`] structure  **must**  be set to [`FALSE`], and the [`logic_op`] member is ignored.
- [`multi_draw_indirect`] specifies whether multiple draw indirect is supported. If this feature is not enabled, the `drawCount` parameter to the [`cmd_draw_indirect`] and [`cmd_draw_indexed_indirect`] commands  **must**  be 0 or 1. The `maxDrawIndirectCount` member of the [`PhysicalDeviceLimits`] structure  **must**  also be 1 if this feature is not supported. See [maxDrawIndirectCount](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxDrawIndirectCount).
- [`draw_indirect_first_instance`] specifies whether indirect drawing calls support the `firstInstance` parameter. If this feature is not enabled, the `firstInstance` member of all [`DrawIndirectCommand`] and [`DrawIndexedIndirectCommand`] structures that are provided to the [`cmd_draw_indirect`] and [`cmd_draw_indexed_indirect`] commands  **must**  be 0.
- [`depth_clamp`] specifies whether depth clamping is supported. If this feature is not enabled, the `depthClampEnable` member of the [`PipelineRasterizationStateCreateInfo`] structure  **must**  be set to [`FALSE`]. Otherwise, setting `depthClampEnable` to [`TRUE`] will enable depth clamping.
- [`depth_bias_clamp`] specifies whether depth bias clamping is supported. If this feature is not enabled, the [`depth_bias_clamp`] member of the [`PipelineRasterizationStateCreateInfo`] structure  **must**  be set to 0.0 unless the `VK_DYNAMIC_STATE_DEPTH_BIAS` dynamic state is enabled, and the [`depth_bias_clamp`] parameter to [`cmd_set_depth_bias`] **must**  be set to 0.0.
- [`fill_mode_non_solid`] specifies whether point and wireframe fill modes are supported. If this feature is not enabled, the `VK_POLYGON_MODE_POINT` and `VK_POLYGON_MODE_LINE` enum values  **must**  not be used.
- [`depth_bounds`] specifies whether depth bounds tests are supported. If this feature is not enabled, the `depthBoundsTestEnable` member of the [`PipelineDepthStencilStateCreateInfo`] structure  **must**  be set to [`FALSE`]. When `depthBoundsTestEnable` is set to [`FALSE`], the `minDepthBounds` and `maxDepthBounds` members of the [`PipelineDepthStencilStateCreateInfo`] structure are ignored.
- [`wide_lines`] specifies whether lines with width other than 1.0 are supported. If this feature is not enabled, the `lineWidth` member of the [`PipelineRasterizationStateCreateInfo`] structure  **must**  be set to 1.0 unless the `VK_DYNAMIC_STATE_LINE_WIDTH` dynamic state is enabled, and the `lineWidth` parameter to [`cmd_set_line_width`] **must**  be set to 1.0. When this feature is supported, the range and granularity of supported line widths are indicated by the `lineWidthRange` and `lineWidthGranularity` members of the [`PhysicalDeviceLimits`] structure, respectively.
- [`large_points`] specifies whether points with size greater than 1.0 are supported. If this feature is not enabled, only a point size of 1.0 written by a shader is supported. The range and granularity of supported point sizes are indicated by the `pointSizeRange` and `pointSizeGranularity` members of the [`PhysicalDeviceLimits`] structure, respectively.
- [`alpha_to_one`] specifies whether the implementation is able to replace the alpha value of the fragment shader color output in the [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-covg) fragment operation. If this feature is not enabled, then the `alphaToOneEnable` member of the [`PipelineMultisampleStateCreateInfo`] structure  **must**  be set to [`FALSE`]. Otherwise setting `alphaToOneEnable` to [`TRUE`] will enable alpha-to-one behavior.
- [`multi_viewport`] specifies whether more than one viewport is supported. If this feature is not enabled:  - The `viewportCount` and `scissorCount` members of the [`PipelineViewportStateCreateInfo`] structure  **must**  be set to 1.  - The `firstViewport` and `viewportCount` parameters to the [`cmd_set_viewport`] command  **must**  be set to 0 and 1, respectively.  - The `firstScissor` and `scissorCount` parameters to the [`cmd_set_scissor`] command  **must**  be set to 0 and 1, respectively.  - The `exclusiveScissorCount` member of the [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure  **must**  be set to 0 or 1.  - The `firstExclusiveScissor` and `exclusiveScissorCount` parameters to the [`cmd_set_exclusive_scissor_nv`] command  **must**  be set to 0 and 1, respectively. 
- [`sampler_anisotropy`] specifies whether anisotropic filtering is supported. If this feature is not enabled, the `anisotropyEnable` member of the [`SamplerCreateInfo`] structure  **must**  be [`FALSE`].
- [`texture_compression_etc2`] specifies whether all of the ETC2 and EAC compressed texture formats are supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must**  be supported in `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK`  - `VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK`  - `VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK`  - `VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK`  - `VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK`  - `VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK`  - `VK_FORMAT_EAC_R11_UNORM_BLOCK`  - `VK_FORMAT_EAC_R11_SNORM_BLOCK`  - `VK_FORMAT_EAC_R11G11_UNORM_BLOCK`  - `VK_FORMAT_EAC_R11G11_SNORM_BLOCK` To query for additional properties, or if the feature is not enabled, [`get_physical_device_format_properties`] and [`get_physical_device_image_format_properties`] **can**  be used to check for supported properties of individual formats as normal.
- [`texture_compression_astc_ldr`] specifies whether all of the ASTC LDR compressed texture formats are supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must**  be supported in `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_ASTC_4x4_UNORM_BLOCK`  - `VK_FORMAT_ASTC_4x4_SRGB_BLOCK`  - `VK_FORMAT_ASTC_5x4_UNORM_BLOCK`  - `VK_FORMAT_ASTC_5x4_SRGB_BLOCK`  - `VK_FORMAT_ASTC_5x5_UNORM_BLOCK`  - `VK_FORMAT_ASTC_5x5_SRGB_BLOCK`  - `VK_FORMAT_ASTC_6x5_UNORM_BLOCK`  - `VK_FORMAT_ASTC_6x5_SRGB_BLOCK`  - `VK_FORMAT_ASTC_6x6_UNORM_BLOCK`  - `VK_FORMAT_ASTC_6x6_SRGB_BLOCK`  - `VK_FORMAT_ASTC_8x5_UNORM_BLOCK`  - `VK_FORMAT_ASTC_8x5_SRGB_BLOCK`  - `VK_FORMAT_ASTC_8x6_UNORM_BLOCK`  - `VK_FORMAT_ASTC_8x6_SRGB_BLOCK`  - `VK_FORMAT_ASTC_8x8_UNORM_BLOCK`  - `VK_FORMAT_ASTC_8x8_SRGB_BLOCK`  - `VK_FORMAT_ASTC_10x5_UNORM_BLOCK`  - `VK_FORMAT_ASTC_10x5_SRGB_BLOCK`  - `VK_FORMAT_ASTC_10x6_UNORM_BLOCK`  - `VK_FORMAT_ASTC_10x6_SRGB_BLOCK`  - `VK_FORMAT_ASTC_10x8_UNORM_BLOCK`  - `VK_FORMAT_ASTC_10x8_SRGB_BLOCK`  - `VK_FORMAT_ASTC_10x10_UNORM_BLOCK`  - `VK_FORMAT_ASTC_10x10_SRGB_BLOCK`  - `VK_FORMAT_ASTC_12x10_UNORM_BLOCK`  - `VK_FORMAT_ASTC_12x10_SRGB_BLOCK`  - `VK_FORMAT_ASTC_12x12_UNORM_BLOCK`  - `VK_FORMAT_ASTC_12x12_SRGB_BLOCK` To query for additional properties, or if the feature is not enabled, [`get_physical_device_format_properties`] and [`get_physical_device_image_format_properties`] **can**  be used to check for supported properties of individual formats as normal.
- [`texture_compression_bc`] specifies whether all of the BC compressed texture formats are supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must**  be supported in `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_BC1_RGB_UNORM_BLOCK`  - `VK_FORMAT_BC1_RGB_SRGB_BLOCK`  - `VK_FORMAT_BC1_RGBA_UNORM_BLOCK`  - `VK_FORMAT_BC1_RGBA_SRGB_BLOCK`  - `VK_FORMAT_BC2_UNORM_BLOCK`  - `VK_FORMAT_BC2_SRGB_BLOCK`  - `VK_FORMAT_BC3_UNORM_BLOCK`  - `VK_FORMAT_BC3_SRGB_BLOCK`  - `VK_FORMAT_BC4_UNORM_BLOCK`  - `VK_FORMAT_BC4_SNORM_BLOCK`  - `VK_FORMAT_BC5_UNORM_BLOCK`  - `VK_FORMAT_BC5_SNORM_BLOCK`  - `VK_FORMAT_BC6H_UFLOAT_BLOCK`  - `VK_FORMAT_BC6H_SFLOAT_BLOCK`  - `VK_FORMAT_BC7_UNORM_BLOCK`  - `VK_FORMAT_BC7_SRGB_BLOCK` To query for additional properties, or if the feature is not enabled, [`get_physical_device_format_properties`] and [`get_physical_device_image_format_properties`] **can**  be used to check for supported properties of individual formats as normal.
- [`occlusion_query_precise`] specifies whether occlusion queries returning actual sample counts are supported. Occlusion queries are created in a [`QueryPool`] by specifying the `queryType` of `VK_QUERY_TYPE_OCCLUSION` in the [`QueryPoolCreateInfo`] structure which is passed to [`create_query_pool`]. If this feature is enabled, queries of this type  **can**  enable `VK_QUERY_CONTROL_PRECISE_BIT` in the `flags` parameter to [`cmd_begin_query`]. If this feature is not supported, the implementation supports only boolean occlusion queries. When any samples are passed, boolean queries will return a non-zero result value, otherwise a result value of zero is returned. When this feature is enabled and `VK_QUERY_CONTROL_PRECISE_BIT` is set, occlusion queries will report the actual number of samples passed.
- [`pipeline_statistics_query`] specifies whether the pipeline statistics queries are supported. If this feature is not enabled, queries of type `VK_QUERY_TYPE_PIPELINE_STATISTICS` **cannot**  be created, and none of the [`QueryPipelineStatisticFlagBits`] bits  **can**  be set in the `pipelineStatistics` member of the [`QueryPoolCreateInfo`] structure.
- [`vertex_pipeline_stores_and_atomics`] specifies whether storage buffers and images support stores and atomic operations in the vertex, tessellation, and geometry shader stages. If this feature is not enabled, all storage image, storage texel buffer, and storage buffer variables used by these stages in shader modules  **must**  be decorated with the `NonWritable` decoration (or the `readonly` memory qualifier in GLSL).
- [`fragment_stores_and_atomics`] specifies whether storage buffers and images support stores and atomic operations in the fragment shader stage. If this feature is not enabled, all storage image, storage texel buffer, and storage buffer variables used by the fragment stage in shader modules  **must**  be decorated with the `NonWritable` decoration (or the `readonly` memory qualifier in GLSL).
- [`shader_tessellation_and_geometry_point_size`] specifies whether the `PointSize` built-in decoration is available in the tessellation control, tessellation evaluation, and geometry shader stages. If this feature is not enabled, members decorated with the `PointSize` built-in decoration  **must**  not be read from or written to and all points written from a tessellation or geometry shader will have a size of 1.0. This also specifies whether shader modules  **can**  declare the `TessellationPointSize` capability for tessellation control and evaluation shaders, or if the shader modules  **can**  declare the `GeometryPointSize` capability for geometry shaders. An implementation supporting this feature  **must**  also support one or both of the [[`tessellation_shader`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) or [[`geometry_shader`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) features.
- [`shader_image_gather_extended`] specifies whether the extended set of image gather instructions are available in shader code. If this feature is not enabled, the `OpImage*Gather` instructions do not support the `Offset` and `ConstOffsets` operands. This also specifies whether shader modules  **can**  declare the `ImageGatherExtended` capability.
- [`shader_storage_image_extended_formats`] specifies whether all the “storage image extended formats” below are supported; if this feature is supported, then the `VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT` **must**  be supported in `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_R16G16_SFLOAT`  - `VK_FORMAT_B10G11R11_UFLOAT_PACK32`  - `VK_FORMAT_R16_SFLOAT`  - `VK_FORMAT_R16G16B16A16_UNORM`  - `VK_FORMAT_A2B10G10R10_UNORM_PACK32`  - `VK_FORMAT_R16G16_UNORM`  - `VK_FORMAT_R8G8_UNORM`  - `VK_FORMAT_R16_UNORM`  - `VK_FORMAT_R8_UNORM`  - `VK_FORMAT_R16G16B16A16_SNORM`  - `VK_FORMAT_R16G16_SNORM`  - `VK_FORMAT_R8G8_SNORM`  - `VK_FORMAT_R16_SNORM`  - `VK_FORMAT_R8_SNORM`  - `VK_FORMAT_R16G16_SINT`  - `VK_FORMAT_R8G8_SINT`  - `VK_FORMAT_R16_SINT`  - `VK_FORMAT_R8_SINT`  - `VK_FORMAT_A2B10G10R10_UINT_PACK32`  - `VK_FORMAT_R16G16_UINT`  - `VK_FORMAT_R8G8_UINT`  - `VK_FORMAT_R16_UINT`  - `VK_FORMAT_R8_UINT` 
- [`shader_storage_image_multisample`] specifies whether multisampled storage images are supported. If this feature is not enabled, images that are created with a `usage` that includes `VK_IMAGE_USAGE_STORAGE_BIT` **must**  be created with `samples` equal to `VK_SAMPLE_COUNT_1_BIT`. This also specifies whether shader modules  **can**  declare the `StorageImageMultisample` and `ImageMSArray` capabilities.
- [`shader_storage_image_read_without_format`] specifies whether storage images require a format qualifier to be specified when reading. [`shader_storage_image_read_without_format`] applies only to formats listed in the [storage without format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-without-shader-storage-format) list.
- [`shader_storage_image_write_without_format`] specifies whether storage images require a format qualifier to be specified when writing. [`shader_storage_image_write_without_format`] applies only to formats listed in the [storage without format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-without-shader-storage-format) list.
- [`shader_uniform_buffer_array_dynamic_indexing`] specifies whether arrays of uniform buffers  **can**  be indexed by *dynamically uniform* integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**  be indexed only by constant integral expressions when aggregated into arrays in shader code. This also specifies whether shader modules  **can**  declare the `UniformBufferArrayDynamicIndexing` capability.
- [`shader_sampled_image_array_dynamic_indexing`] specifies whether arrays of samplers or sampled images  **can**  be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**  be indexed only by constant integral expressions when aggregated into arrays in shader code. This also specifies whether shader modules  **can**  declare the `SampledImageArrayDynamicIndexing` capability.
- [`shader_storage_buffer_array_dynamic_indexing`] specifies whether arrays of storage buffers  **can**  be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**  be indexed only by constant integral expressions when aggregated into arrays in shader code. This also specifies whether shader modules  **can**  declare the `StorageBufferArrayDynamicIndexing` capability.
- [`shader_storage_image_array_dynamic_indexing`] specifies whether arrays of storage images  **can**  be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  be indexed only by constant integral expressions when aggregated into arrays in shader code. This also specifies whether shader modules  **can**  declare the `StorageImageArrayDynamicIndexing` capability.
- [`shader_clip_distance`] specifies whether clip distances are supported in shader code. If this feature is not enabled, any members decorated with the `ClipDistance` built-in decoration  **must**  not be read from or written to in shader modules. This also specifies whether shader modules  **can**  declare the `ClipDistance` capability.
- [`shader_cull_distance`] specifies whether cull distances are supported in shader code. If this feature is not enabled, any members decorated with the `CullDistance` built-in decoration  **must**  not be read from or written to in shader modules. This also specifies whether shader modules  **can**  declare the `CullDistance` capability.
- [`shader_float64`] specifies whether 64-bit floats (doubles) are supported in shader code. If this feature is not enabled, 64-bit floating-point types  **must**  not be used in shader code. This also specifies whether shader modules  **can**  declare the `Float64` capability. Declaring and using 64-bit floats is enabled for all storage classes that SPIR-V allows with the `Float64` capability.
- [`shader_int64`] specifies whether 64-bit integers (signed and unsigned) are supported in shader code. If this feature is not enabled, 64-bit integer types  **must**  not be used in shader code. This also specifies whether shader modules  **can**  declare the `Int64` capability. Declaring and using 64-bit integers is enabled for all storage classes that SPIR-V allows with the `Int64` capability.
- [`shader_int16`] specifies whether 16-bit integers (signed and unsigned) are supported in shader code. If this feature is not enabled, 16-bit integer types  **must**  not be used in shader code. This also specifies whether shader modules  **can**  declare the `Int16` capability. However, this only enables a subset of the storage classes that SPIR-V allows for the `Int16` SPIR-V capability: Declaring and using 16-bit integers in the `Private`, `Workgroup` (for non-Block variables), and `Function` storage classes is enabled, while declaring them in the interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`, `Output`, and `PushConstant`) is not enabled.
- [`shader_resource_residency`] specifies whether image operations that return resource residency information are supported in shader code. If this feature is not enabled, the `OpImageSparse*` instructions  **must**  not be used in shader code. This also specifies whether shader modules  **can**  declare the `SparseResidency` capability. The feature requires at least one of the `sparseResidency*` features to be supported.
- [`shader_resource_min_lod`] specifies whether image operations specifying the minimum resource LOD are supported in shader code. If this feature is not enabled, the `MinLod` image operand  **must**  not be used in shader code. This also specifies whether shader modules  **can**  declare the `MinLod` capability.
- [`sparse_binding`] specifies whether resource memory  **can**  be managed at opaque sparse block level instead of at the object level. If this feature is not enabled, resource memory  **must**  be bound only on a per-object basis using the [`bind_buffer_memory`] and [`bind_image_memory`] commands. In this case, buffers and images  **must**  not be created with `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` and `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` set in the `flags` member of the [`BufferCreateInfo`] and [`ImageCreateInfo`] structures, respectively. Otherwise resource memory  **can**  be managed as described in [Sparse Resource Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseresourcefeatures).
- [`sparse_residency_buffer`] specifies whether the device  **can**  access partially resident buffers. If this feature is not enabled, buffers  **must**  not be created with `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the [`BufferCreateInfo`] structure.
- [`sparse_residency_image2_d`] specifies whether the device  **can**  access partially resident 2D images with 1 sample per pixel. If this feature is not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples` set to `VK_SAMPLE_COUNT_1_BIT` **must**  not be created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the [`ImageCreateInfo`] structure.
- [`sparse_residency_image3_d`] specifies whether the device  **can**  access partially resident 3D images. If this feature is not enabled, images with an `imageType` of `VK_IMAGE_TYPE_3D` **must**  not be created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the [`ImageCreateInfo`] structure.
- [`sparse_residency2_samples`] specifies whether the physical device  **can**  access partially resident 2D images with 2 samples per pixel. If this feature is not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples` set to `VK_SAMPLE_COUNT_2_BIT` **must**  not be created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the [`ImageCreateInfo`] structure.
- [`sparse_residency4_samples`] specifies whether the physical device  **can**  access partially resident 2D images with 4 samples per pixel. If this feature is not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples` set to `VK_SAMPLE_COUNT_4_BIT` **must**  not be created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the [`ImageCreateInfo`] structure.
- [`sparse_residency8_samples`] specifies whether the physical device  **can**  access partially resident 2D images with 8 samples per pixel. If this feature is not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples` set to `VK_SAMPLE_COUNT_8_BIT` **must**  not be created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the [`ImageCreateInfo`] structure.
- [`sparse_residency16_samples`] specifies whether the physical device  **can**  access partially resident 2D images with 16 samples per pixel. If this feature is not enabled, images with an `imageType` of `VK_IMAGE_TYPE_2D` and `samples` set to `VK_SAMPLE_COUNT_16_BIT` **must**  not be created with `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member of the [`ImageCreateInfo`] structure.
- [`sparse_residency_aliased`] specifies whether the physical device  **can**  correctly access data aliased into multiple locations. If this feature is not enabled, the `VK_BUFFER_CREATE_SPARSE_ALIASED_BIT` and `VK_IMAGE_CREATE_SPARSE_ALIASED_BIT` enum values  **must**  not be used in `flags` members of the [`BufferCreateInfo`] and [`ImageCreateInfo`] structures, respectively.
- [`variable_multisample_rate`] specifies whether all pipelines that will be bound to a command buffer during a [subpass which uses no attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-noattachments) **must**  have the same value for [`PipelineMultisampleStateCreateInfo::rasterization_samples`]. If set to [`TRUE`], the implementation supports variable multisample rates in a subpass which uses no attachments. If set to [`FALSE`], then all pipelines bound in such a subpass  **must**  have the same multisample rate. This has no effect in situations where a subpass uses any attachments.
- [`inherited_queries`] specifies whether a secondary command buffer  **may**  be executed while a query is active.

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`DeviceCreateInfo`]
- [`PhysicalDeviceFeatures2`]
- [`get_physical_device_features`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        