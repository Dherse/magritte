[VkPhysicalDevicePortabilitySubsetFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html) - Structure describing the features that may not be supported by an implementation of the Vulkan 1.0 Portability Subset

# C Specifications
The [`PhysicalDevicePortabilitySubsetFeaturesKHR`] structure is defined
as:
```c
// Provided by VK_KHR_portability_subset
typedef struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           constantAlphaColorBlendFactors;
    VkBool32           events;
    VkBool32           imageViewFormatReinterpretation;
    VkBool32           imageViewFormatSwizzle;
    VkBool32           imageView2DOn3DImage;
    VkBool32           multisampleArrayImage;
    VkBool32           mutableComparisonSamplers;
    VkBool32           pointPolygons;
    VkBool32           samplerMipLodBias;
    VkBool32           separateStencilMaskRef;
    VkBool32           shaderSampleRateInterpolationFunctions;
    VkBool32           tessellationIsolines;
    VkBool32           tessellationPointMode;
    VkBool32           triangleFans;
    VkBool32           vertexAttributeAccessBeyondStride;
} VkPhysicalDevicePortabilitySubsetFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`constant_alpha_color_blend_factors`] indicates whether this implementation supports constant *alpha*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendfactors](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendfactors) used as source or destination *color*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending).
- [`events`] indicates whether this implementation supports synchronization using [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-events](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-events).
- [`image_view_format_reinterpretation`] indicates whether this implementation supports a [`ImageView`] being created with a texel format containing a different number of components, or a different number of bits in each component, than the texel format of the underlying [`Image`].
- [`image_view_format_swizzle`] indicates whether this implementation supports remapping format components using [`ImageViewCreateInfo::components`].
- [`image_view2_d_on3_d_image`] indicates whether this implementation supports a [`Image`] being created with the `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT` flag set, permitting a 2D or 2D array image view to be created on a 3D [`Image`].
- [`multisample_array_image`] indicates whether this implementation supports a [`Image`] being created as a 2D array with multiple samples per texel.
- [`mutable_comparison_samplers`] indicates whether this implementation allows descriptors with comparison samplers to be [updated](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates).
- [`point_polygons`] indicates whether this implementation supports [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast) using a *point*[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-polygonmode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-polygonmode).
- [`sampler_mip_lod_bias`] indicates whether this implementation supports setting a [mipmap LOD bias value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-mipLodBias) when [creating a sampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers).
- [`separate_stencil_mask_ref`] indicates whether this implementation supports separate front and back [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil) reference values.
- [`shader_sample_rate_interpolation_functions`] indicates whether this implementation supports fragment shaders which use the [`InterpolationFunction`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities-table-InterpolationFunction) capability and the extended instructions `InterpolateAtCentroid`, `InterpolateAtOffset`, and `InterpolateAtSample` from the `GLSL.std.450` extended instruction set. This member is only meaningful if the [sampleRateShading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sampleRateShading) feature is supported.
- [`tessellation_isolines`] indicates whether this implementation supports [isoline output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation-isoline-tessellation) from the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation) stage of a graphics pipeline. This member is only meaningful if [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) are supported.
- [`tessellation_point_mode`] indicates whether this implementation supports [point output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation-point-mode) from the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation) stage of a graphics pipeline. This member is only meaningful if [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) are supported.
- [`triangle_fans`] indicates whether this implementation supports [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-triangle-fans) primitive topology.
- [`vertex_attribute_access_beyond_stride`] indicates whether this implementation supports accessing a vertex input attribute beyond the stride of the corresponding vertex input binding.
If the [`PhysicalDevicePortabilitySubsetFeaturesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDevicePortabilitySubsetFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR`

# Related
- [`VK_KHR_portability_subset`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        