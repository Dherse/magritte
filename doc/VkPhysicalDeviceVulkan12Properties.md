[VkPhysicalDeviceVulkan12Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html) - Structure specifying physical device properties for functionality promoted to Vulkan 1.2

# C Specifications
The [`PhysicalDeviceVulkan12Properties`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceVulkan12Properties {
    VkStructureType                      sType;
    void*                                pNext;
    VkDriverId                           driverID;
    char                                 driverName[VK_MAX_DRIVER_NAME_SIZE];
    char                                 driverInfo[VK_MAX_DRIVER_INFO_SIZE];
    VkConformanceVersion                 conformanceVersion;
    VkShaderFloatControlsIndependence    denormBehaviorIndependence;
    VkShaderFloatControlsIndependence    roundingModeIndependence;
    VkBool32                             shaderSignedZeroInfNanPreserveFloat16;
    VkBool32                             shaderSignedZeroInfNanPreserveFloat32;
    VkBool32                             shaderSignedZeroInfNanPreserveFloat64;
    VkBool32                             shaderDenormPreserveFloat16;
    VkBool32                             shaderDenormPreserveFloat32;
    VkBool32                             shaderDenormPreserveFloat64;
    VkBool32                             shaderDenormFlushToZeroFloat16;
    VkBool32                             shaderDenormFlushToZeroFloat32;
    VkBool32                             shaderDenormFlushToZeroFloat64;
    VkBool32                             shaderRoundingModeRTEFloat16;
    VkBool32                             shaderRoundingModeRTEFloat32;
    VkBool32                             shaderRoundingModeRTEFloat64;
    VkBool32                             shaderRoundingModeRTZFloat16;
    VkBool32                             shaderRoundingModeRTZFloat32;
    VkBool32                             shaderRoundingModeRTZFloat64;
    uint32_t                             maxUpdateAfterBindDescriptorsInAllPools;
    VkBool32                             shaderUniformBufferArrayNonUniformIndexingNative;
    VkBool32                             shaderSampledImageArrayNonUniformIndexingNative;
    VkBool32                             shaderStorageBufferArrayNonUniformIndexingNative;
    VkBool32                             shaderStorageImageArrayNonUniformIndexingNative;
    VkBool32                             shaderInputAttachmentArrayNonUniformIndexingNative;
    VkBool32                             robustBufferAccessUpdateAfterBind;
    VkBool32                             quadDivergentImplicitLod;
    uint32_t                             maxPerStageDescriptorUpdateAfterBindSamplers;
    uint32_t                             maxPerStageDescriptorUpdateAfterBindUniformBuffers;
    uint32_t                             maxPerStageDescriptorUpdateAfterBindStorageBuffers;
    uint32_t                             maxPerStageDescriptorUpdateAfterBindSampledImages;
    uint32_t                             maxPerStageDescriptorUpdateAfterBindStorageImages;
    uint32_t                             maxPerStageDescriptorUpdateAfterBindInputAttachments;
    uint32_t                             maxPerStageUpdateAfterBindResources;
    uint32_t                             maxDescriptorSetUpdateAfterBindSamplers;
    uint32_t                             maxDescriptorSetUpdateAfterBindUniformBuffers;
    uint32_t                             maxDescriptorSetUpdateAfterBindUniformBuffersDynamic;
    uint32_t                             maxDescriptorSetUpdateAfterBindStorageBuffers;
    uint32_t                             maxDescriptorSetUpdateAfterBindStorageBuffersDynamic;
    uint32_t                             maxDescriptorSetUpdateAfterBindSampledImages;
    uint32_t                             maxDescriptorSetUpdateAfterBindStorageImages;
    uint32_t                             maxDescriptorSetUpdateAfterBindInputAttachments;
    VkResolveModeFlags                   supportedDepthResolveModes;
    VkResolveModeFlags                   supportedStencilResolveModes;
    VkBool32                             independentResolveNone;
    VkBool32                             independentResolve;
    VkBool32                             filterMinmaxSingleComponentFormats;
    VkBool32                             filterMinmaxImageComponentMapping;
    uint64_t                             maxTimelineSemaphoreValueDifference;
    VkSampleCountFlags                   framebufferIntegerColorSampleCounts;
} VkPhysicalDeviceVulkan12Properties;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

# Description
- [`driver_id`] is a unique identifier for the driver of the physical device.
- [`driver_name`] is an array of [`MAX_DRIVER_NAME_SIZE`]`char` containing a null-terminated UTF-8 string which is the name of the driver.
- [`driver_info`] is an array of [`MAX_DRIVER_INFO_SIZE`]`char` containing a null-terminated UTF-8 string with additional information about the driver.
- [`conformance_version`] is the version of the Vulkan conformance test this driver is conformant against (see [`ConformanceVersion`]).
- [`denorm_behavior_independence`] is a [`ShaderFloatControlsIndependence`] value indicating whether, and how, denorm behavior can be set independently for different bit widths.
- [`rounding_mode_independence`] is a [`ShaderFloatControlsIndependence`] value indicating whether, and how, rounding modes can be set independently for different bit widths.
- [`shader_signed_zero_inf_nan_preserve_float16`] is a boolean value indicating whether sign of a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span class="mord">∞</span></span></span></span> **can**  be preserved in 16-bit floating-point computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_signed_zero_inf_nan_preserve_float32`] is a boolean value indicating whether sign of a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span style="height:0.66666em;vertical-align:-0.08333em;" class="strut"></span><span class="mord">±</span><span class="mord">∞</span></span></span></span> **can**  be preserved in 32-bit floating-point computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_signed_zero_inf_nan_preserve_float64`] is a boolean value indicating whether sign of a zero, Nans and <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span style="height:0.66666em;vertical-align:-0.08333em;" class="strut"></span><span class="mord">±</span><span class="mord">∞</span></span></span></span> **can**  be preserved in 64-bit floating-point computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_denorm_preserve_float16`] is a boolean value indicating whether denormals  **can**  be preserved in 16-bit floating-point computations. It also indicates whether the `DenormPreserve` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_denorm_preserve_float32`] is a boolean value indicating whether denormals  **can**  be preserved in 32-bit floating-point computations. It also indicates whether the `DenormPreserve` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_denorm_preserve_float64`] is a boolean value indicating whether denormals  **can**  be preserved in 64-bit floating-point computations. It also indicates whether the `DenormPreserve` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_denorm_flush_to_zero_float16`] is a boolean value indicating whether denormals  **can**  be flushed to zero in 16-bit floating-point computations. It also indicates whether the `DenormFlushToZero` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_denorm_flush_to_zero_float32`] is a boolean value indicating whether denormals  **can**  be flushed to zero in 32-bit floating-point computations. It also indicates whether the `DenormFlushToZero` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_denorm_flush_to_zero_float64`] is a boolean value indicating whether denormals  **can**  be flushed to zero in 64-bit floating-point computations. It also indicates whether the `DenormFlushToZero` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_rounding_mode_rte_float16`] is a boolean value indicating whether an implementation supports the round-to-nearest-even rounding mode for 16-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_rounding_mode_rte_float32`] is a boolean value indicating whether an implementation supports the round-to-nearest-even rounding mode for 32-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_rounding_mode_rte_float64`] is a boolean value indicating whether an implementation supports the round-to-nearest-even rounding mode for 64-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode  **can**  be used for 64-bit floating-point types.
- [`shader_rounding_mode_rtz_float16`] is a boolean value indicating whether an implementation supports the round-towards-zero rounding mode for 16-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode  **can**  be used for 16-bit floating-point types.
- [`shader_rounding_mode_rtz_float32`] is a boolean value indicating whether an implementation supports the round-towards-zero rounding mode for 32-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode  **can**  be used for 32-bit floating-point types.
- [`shader_rounding_mode_rtz_float64`] is a boolean value indicating whether an implementation supports the round-towards-zero rounding mode for 64-bit floating-point arithmetic and conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode  **can**  be used for 64-bit floating-point types.
- [`max_update_after_bind_descriptors_in_all_pools`] is the maximum number of descriptors (summed over all descriptor types) that  **can**  be created across all pools that are created with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit set. Pool creation  **may**  fail when this limit is exceeded, or when the space this limit represents is unable to satisfy a pool creation due to fragmentation.
- [`shader_uniform_buffer_array_non_uniform_indexing_native`] is a boolean value indicating whether uniform buffer descriptors natively support nonuniform indexing. If this is [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array of uniform buffers  **may**  execute multiple times in order to access all the descriptors.
- [`shader_sampled_image_array_non_uniform_indexing_native`] is a boolean value indicating whether sampler and image descriptors natively support nonuniform indexing. If this is [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array of samplers or images  **may**  execute multiple times in order to access all the descriptors.
- [`shader_storage_buffer_array_non_uniform_indexing_native`] is a boolean value indicating whether storage buffer descriptors natively support nonuniform indexing. If this is [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array of storage buffers  **may**  execute multiple times in order to access all the descriptors.
- [`shader_storage_image_array_non_uniform_indexing_native`] is a boolean value indicating whether storage image descriptors natively support nonuniform indexing. If this is [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array of storage images  **may**  execute multiple times in order to access all the descriptors.
- [`shader_input_attachment_array_non_uniform_indexing_native`] is a boolean value indicating whether input attachment descriptors natively support nonuniform indexing. If this is [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array of input attachments  **may**  execute multiple times in order to access all the descriptors.
- [`robust_buffer_access_update_after_bind`] is a boolean value indicating whether [`robustBufferAccess`]() **can**  be enabled in a device simultaneously with `descriptorBindingUniformBufferUpdateAfterBind`, `descriptorBindingStorageBufferUpdateAfterBind`, `descriptorBindingUniformTexelBufferUpdateAfterBind`, and/or `descriptorBindingStorageTexelBufferUpdateAfterBind`. If this is [`FALSE`], then either `robustBufferAccess` **must**  be disabled or all of these update-after-bind features  **must**  be disabled.
- [`quad_divergent_implicit_lod`] is a boolean value indicating whether implicit level of detail calculations for image operations have well-defined results when the image and/or sampler objects used for the instruction are not uniform within a quad. See [Derivative Image Operations]().
- [`max_per_stage_descriptor_update_after_bind_samplers`] is similar to `maxPerStageDescriptorSamplers` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_per_stage_descriptor_update_after_bind_uniform_buffers`] is similar to `maxPerStageDescriptorUniformBuffers` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_per_stage_descriptor_update_after_bind_storage_buffers`] is similar to `maxPerStageDescriptorStorageBuffers` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_per_stage_descriptor_update_after_bind_sampled_images`] is similar to `maxPerStageDescriptorSampledImages` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_per_stage_descriptor_update_after_bind_storage_images`] is similar to `maxPerStageDescriptorStorageImages` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_per_stage_descriptor_update_after_bind_input_attachments`] is similar to `maxPerStageDescriptorInputAttachments` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_per_stage_update_after_bind_resources`] is similar to `maxPerStageResources` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_update_after_bind_samplers`] is similar to `maxDescriptorSetSamplers` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_update_after_bind_uniform_buffers`] is similar to `maxDescriptorSetUniformBuffers` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_update_after_bind_uniform_buffers_dynamic`] is similar to `maxDescriptorSetUniformBuffersDynamic` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set. While an application  **can**  allocate dynamic uniform buffer descriptors from a pool created with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these descriptors  **must**  not be present in any descriptor set layout that includes bindings created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
- [`max_descriptor_set_update_after_bind_storage_buffers`] is similar to `maxDescriptorSetStorageBuffers` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_update_after_bind_storage_buffers_dynamic`] is similar to `maxDescriptorSetStorageBuffersDynamic` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set. While an application  **can**  allocate dynamic storage buffer descriptors from a pool created with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these descriptors  **must**  not be present in any descriptor set layout that includes bindings created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
- [`max_descriptor_set_update_after_bind_sampled_images`] is similar to `maxDescriptorSetSampledImages` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_update_after_bind_storage_images`] is similar to `maxDescriptorSetStorageImages` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_update_after_bind_input_attachments`] is similar to `maxDescriptorSetInputAttachments` but counts descriptors from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`supported_depth_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set of supported depth resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in the set but implementations  **may**  support additional modes.
- [`supported_stencil_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set of supported stencil resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in the set but implementations  **may**  support additional modes. `VK_RESOLVE_MODE_AVERAGE_BIT` **must**  not be included in the set.
- [`independent_resolve_none`] is [`TRUE`] if the implementation supports setting the depth and stencil resolve modes to different values when one of those modes is `VK_RESOLVE_MODE_NONE`. Otherwise the implementation only supports setting both modes to the same value.
- [`independent_resolve`] is [`TRUE`] if the implementation supports all combinations of the supported depth and stencil resolve modes, including setting either depth or stencil resolve mode to `VK_RESOLVE_MODE_NONE`. An implementation that supports [`independent_resolve`] **must**  also support [`independent_resolve_none`].
- [`filter_minmax_single_component_formats`] is a boolean value indicating whether a minimum set of required formats support min/max filtering.
- [`filter_minmax_image_component_mapping`] is a boolean value indicating whether the implementation supports non-identity component mapping of the image when doing min/max filtering.
- [`max_timeline_semaphore_value_difference`] indicates the maximum difference allowed by the implementation between the current value of a timeline semaphore and any pending signal or wait operations.
- [`framebuffer_integer_color_sample_counts`] is a bitmask of [`SampleCountFlagBits`] indicating the color sample counts that are supported for all framebuffer color attachments with integer formats.
If the [`PhysicalDeviceVulkan12Properties`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.These properties correspond to Vulkan 1.2 functionality.The members of [`PhysicalDeviceVulkan12Properties`] **must**  have the same
values as the corresponding members of
[`PhysicalDeviceDriverProperties`],
[`PhysicalDeviceFloatControlsProperties`],
[`PhysicalDeviceDescriptorIndexingProperties`],
[`PhysicalDeviceDepthStencilResolveProperties`],
[`PhysicalDeviceSamplerFilterMinmaxProperties`], and
[`PhysicalDeviceTimelineSemaphoreProperties`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`

# Related
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`ConformanceVersion`]
- [`DriverId`]
- [`ResolveModeFlags`]
- [`SampleCountFlags`]
- [`ShaderFloatControlsIndependence`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        