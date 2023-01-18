[vkCmdTraceRaysNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html) - Initialize a ray tracing dispatch

# C Specifications
To dispatch ray tracing use:
```c
// Provided by VK_NV_ray_tracing
void vkCmdTraceRaysNV(
    VkCommandBuffer                             commandBuffer,
    VkBuffer                                    raygenShaderBindingTableBuffer,
    VkDeviceSize                                raygenShaderBindingOffset,
    VkBuffer                                    missShaderBindingTableBuffer,
    VkDeviceSize                                missShaderBindingOffset,
    VkDeviceSize                                missShaderBindingStride,
    VkBuffer                                    hitShaderBindingTableBuffer,
    VkDeviceSize                                hitShaderBindingOffset,
    VkDeviceSize                                hitShaderBindingStride,
    VkBuffer                                    callableShaderBindingTableBuffer,
    VkDeviceSize                                callableShaderBindingOffset,
    VkDeviceSize                                callableShaderBindingStride,
    uint32_t                                    width,
    uint32_t                                    height,
    uint32_t                                    depth);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`raygen_shader_binding_table_buffer`] is the buffer object that holds the shader binding table data for the ray generation shader stage.
- [`raygen_shader_binding_offset`] is the offset in bytes (relative to [`raygen_shader_binding_table_buffer`]) of the ray generation shader being used for the trace.
- [`miss_shader_binding_table_buffer`] is the buffer object that holds the shader binding table data for the miss shader stage.
- [`miss_shader_binding_offset`] is the offset in bytes (relative to [`miss_shader_binding_table_buffer`]) of the miss shader being used for the trace.
- [`miss_shader_binding_stride`] is the size in bytes of each shader binding table record in [`miss_shader_binding_table_buffer`].
- [`hit_shader_binding_table_buffer`] is the buffer object that holds the shader binding table data for the hit shader stages.
- [`hit_shader_binding_offset`] is the offset in bytes (relative to [`hit_shader_binding_table_buffer`]) of the hit shader group being used for the trace.
- [`hit_shader_binding_stride`] is the size in bytes of each shader binding table record in [`hit_shader_binding_table_buffer`].
- [`callable_shader_binding_table_buffer`] is the buffer object that holds the shader binding table data for the callable shader stage.
- [`callable_shader_binding_offset`] is the offset in bytes (relative to [`callable_shader_binding_table_buffer`]) of the callable shader being used for the trace.
- [`callable_shader_binding_stride`] is the size in bytes of each shader binding table record in [`callable_shader_binding_table_buffer`].
- [`width`] is the width of the ray trace query dimensions.
- [`height`] is height of the ray trace query dimensions.
- [`depth`] is depth of the ray trace query dimensions.

# Description
When the command is executed, a ray generation group of [`width`]
× [`height`] × [`depth`] rays is assembled.
## Valid Usage
-    If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
-    If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
-    If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
-    If a [`ImageView`] is accessed using atomic operations as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
-    If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the image view’s [format features]() **must**  contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
-    Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command  **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by [`get_physical_device_image_format_properties2`]
-    Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command  **must**  have a [`ImageViewType`] and format that supports cubic filtering together with minmax filtering, as specified by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by [`get_physical_device_image_format_properties2`]
-    Any [`Image`] created with a [`ImageCreateInfo::flags`] containing `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
-    Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
-    Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
-    For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the [`PipelineLayout`] used to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
-    If the [`maintenance4`]() feature is not enabled, then for each push constant that is statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a push constant value  **must**  have been set for the same pipeline bind point, with a [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
-    Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],  **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind point used by this command
-    A valid pipeline  **must**  be bound to the pipeline bind point used by this command
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any dynamic state, that state  **must**  have been set or inherited (if the `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done so after any previously bound pipeline with the corresponding state not specified as dynamic
-    There  **must**  not have been any calls to dynamic state setting commands for any state not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this command, since that pipeline was bound
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a LOD bias or any offset values, in any shader stage
-    If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not access values outside of the range of the buffer as specified in the descriptor set bound to the same pipeline bind point
-    If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not access values outside of the range of the buffer as specified in the descriptor set bound to the same pipeline bind point
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point used by this command  **must**  not be a protected resource
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](), that object  **must**  only be used with `OpImageSample*` or `OpImageSparseSample*` instructions
-    If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
-    If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the `Type` of the `Texel` operand of that instruction  **must**  have at least as many components as the image view’s format
-    If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the `Type` of the `Texel` operand of that instruction  **must**  have at least as many components as the buffer view’s format
-    If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 64
-    If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 32
-    If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 64
-    If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**  have a `Width` of 32
-    If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this command
-    If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this command
-    Any shader group handle referenced by this call  **must**  have been queried from the currently bound ray tracing pipeline

-  [`command_buffer`] **must**  not be a protected command buffer
-    This command  **must**  not cause a pipeline trace ray instruction to be executed from a shader invocation with a [recursion depth](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing-recursion-depth) greater than the value of `maxRecursionDepth` used to create the bound ray tracing pipeline
-    If [`raygen_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`raygen_shader_binding_offset`] **must**  be less than the size of [`raygen_shader_binding_table_buffer`]
-  [`raygen_shader_binding_offset`] **must**  be a multiple of [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
-    If [`miss_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`miss_shader_binding_offset`] **must**  be less than the size of [`miss_shader_binding_table_buffer`]
-  [`miss_shader_binding_offset`] **must**  be a multiple of [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
-    If [`hit_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`hit_shader_binding_offset`] **must**  be less than the size of [`hit_shader_binding_table_buffer`]
-  [`hit_shader_binding_offset`] **must**  be a multiple of [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
-    If [`callable_shader_binding_table_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-  [`callable_shader_binding_offset`] **must**  be less than the size of [`callable_shader_binding_table_buffer`]
-  [`callable_shader_binding_offset`] **must**  be a multiple of [`PhysicalDeviceRayTracingPropertiesNV::shader_group_base_alignment`]
-  [`miss_shader_binding_stride`] **must**  be a multiple of [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
-  [`hit_shader_binding_stride`] **must**  be a multiple of [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
-  [`callable_shader_binding_stride`] **must**  be a multiple of [`PhysicalDeviceRayTracingPropertiesNV::shader_group_handle_size`]
-  [`miss_shader_binding_stride`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
-  [`hit_shader_binding_stride`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
-  [`callable_shader_binding_stride`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_shader_group_stride`]
-  [`width`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][0]
-  [`height`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][1]
-  [`depth`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][2]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`raygen_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
-    If [`miss_shader_binding_table_buffer`] is not [`crate::Handle::null`], [`miss_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
-    If [`hit_shader_binding_table_buffer`] is not [`crate::Handle::null`], [`hit_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
-    If [`callable_shader_binding_table_buffer`] is not [`crate::Handle::null`], [`callable_shader_binding_table_buffer`] **must**  be a valid [`Buffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Each of [`callable_shader_binding_table_buffer`], [`command_buffer`], [`hit_shader_binding_table_buffer`], [`miss_shader_binding_table_buffer`], and [`raygen_shader_binding_table_buffer`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_NV_ray_tracing`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        