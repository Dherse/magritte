[VK_VERSION_1_2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_2.html) - Vulkan version 1.2

# Description
Vulkan Version 1.2 [promoted](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-compatibility-promotion) a
number of key extensions into the core API:
- `[`VK_KHR_8bit_storage`]`
- `[`VK_KHR_buffer_device_address`]`
- `[`VK_KHR_create_renderpass2`]`
- `[`VK_KHR_depth_stencil_resolve`]`
- `[`VK_KHR_draw_indirect_count`]`
- `[`VK_KHR_driver_properties`]`
- `[`VK_KHR_image_format_list`]`
- `[`VK_KHR_imageless_framebuffer`]`
- `[`VK_KHR_sampler_mirror_clamp_to_edge`]`
- `[`VK_KHR_separate_depth_stencil_layouts`]`
- `[`VK_KHR_shader_atomic_int64`]`
- `[`VK_KHR_shader_float16_int8`]`
- `[`VK_KHR_shader_float_controls`]`
- `[`VK_KHR_shader_subgroup_extended_types`]`
- `[`VK_KHR_spirv_1_4`]`
- `[`VK_KHR_timeline_semaphore`]`
- `[`VK_KHR_uniform_buffer_standard_layout`]`
- `[`VK_KHR_vulkan_memory_model`]`
- `[`VK_EXT_descriptor_indexing`]`
- `[`VK_EXT_host_query_reset`]`
- `[`VK_EXT_sampler_filter_minmax`]`
- `[`VK_EXT_scalar_block_layout`]`
- `[`VK_EXT_separate_stencil_usage`]`
- `[`VK_EXT_shader_viewport_index_layer`]`
All differences in behavior between these extensions and the corresponding
Vulkan 1.2 functionality are summarized in the [Vulkan 1.2 specification appendix](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions).
### New Macros

- [`crate::Version::VULKAN1_2`]

### New Commands

- [`cmd_begin_render_pass2`]
- [`cmd_draw_indexed_indirect_count`]
- [`cmd_draw_indirect_count`]
- [`cmd_end_render_pass2`]
- [`cmd_next_subpass2`]
- [`create_render_pass2`]
- [`get_buffer_device_address`]
- [`get_buffer_opaque_capture_address`]
- [`get_device_memory_opaque_capture_address`]
- [`get_semaphore_counter_value`]
- [`reset_query_pool`]
- [`signal_semaphore`]
- [`wait_semaphores`]

### New Structures

- [`AttachmentDescription2`]
- [`AttachmentReference2`]
- [`BufferDeviceAddressInfo`]
- [`ConformanceVersion`]
- [`DeviceMemoryOpaqueCaptureAddressInfo`]
- [`FramebufferAttachmentImageInfo`]
- [`RenderPassCreateInfo2`]
- [`SemaphoreSignalInfo`]
- [`SemaphoreWaitInfo`]
- [`SubpassBeginInfo`]
- [`SubpassDependency2`]
- [`SubpassDescription2`]
- [`SubpassEndInfo`]
- Extending [`AttachmentDescription2`]:  - [`AttachmentDescriptionStencilLayout`] 
- Extending [`AttachmentReference2`]:  - [`AttachmentReferenceStencilLayout`] 
- Extending [`BufferCreateInfo`]:  - [`BufferOpaqueCaptureAddressCreateInfo`] 
- Extending [`DescriptorSetAllocateInfo`]:  - [`DescriptorSetVariableDescriptorCountAllocateInfo`] 
- Extending [`DescriptorSetLayoutCreateInfo`]:  - [`DescriptorSetLayoutBindingFlagsCreateInfo`] 
- Extending [`DescriptorSetLayoutSupport`]:  - [`DescriptorSetVariableDescriptorCountLayoutSupport`] 
- Extending [`FramebufferCreateInfo`]:  - [`FramebufferAttachmentsCreateInfo`] 
- Extending [`ImageCreateInfo`], [`PhysicalDeviceImageFormatInfo2`]:  - [`ImageStencilUsageCreateInfo`] 
- Extending [`ImageCreateInfo`], [`SwapchainCreateInfoKHR`], [`PhysicalDeviceImageFormatInfo2`]:  - [`ImageFormatListCreateInfo`] 
- Extending [`MemoryAllocateInfo`]:  - [`MemoryOpaqueCaptureAddressAllocateInfo`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevice8BitStorageFeatures`]  - [`PhysicalDeviceBufferDeviceAddressFeatures`]  - [`PhysicalDeviceDescriptorIndexingFeatures`]  - [`PhysicalDeviceHostQueryResetFeatures`]  - [`PhysicalDeviceImagelessFramebufferFeatures`]  - [`PhysicalDeviceScalarBlockLayoutFeatures`]  - [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`]  - [`PhysicalDeviceShaderAtomicInt64Features`]  - [`PhysicalDeviceShaderFloat16Int8Features`]  - [`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`]  - [`PhysicalDeviceTimelineSemaphoreFeatures`]  - [`PhysicalDeviceUniformBufferStandardLayoutFeatures`]  - [`PhysicalDeviceVulkan11Features`]  - [`PhysicalDeviceVulkan12Features`]  - [`PhysicalDeviceVulkanMemoryModelFeatures`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceDepthStencilResolveProperties`]  - [`PhysicalDeviceDescriptorIndexingProperties`]  - [`PhysicalDeviceDriverProperties`]  - [`PhysicalDeviceFloatControlsProperties`]  - [`PhysicalDeviceSamplerFilterMinmaxProperties`]  - [`PhysicalDeviceTimelineSemaphoreProperties`]  - [`PhysicalDeviceVulkan11Properties`]  - [`PhysicalDeviceVulkan12Properties`] 
- Extending [`RenderPassBeginInfo`]:  - [`RenderPassAttachmentBeginInfo`] 
- Extending [`SamplerCreateInfo`]:  - [`SamplerReductionModeCreateInfo`] 
- Extending [`SemaphoreCreateInfo`], [`PhysicalDeviceExternalSemaphoreInfo`]:  - [`SemaphoreTypeCreateInfo`] 
- Extending [`SubmitInfo`], [`BindSparseInfo`]:  - [`TimelineSemaphoreSubmitInfo`] 
- Extending [`SubpassDescription2`]:  - [`SubpassDescriptionDepthStencilResolve`] 

### New Enums

- [`DescriptorBindingFlagBits`]
- [`DriverId`]
- [`ResolveModeFlagBits`]
- [`SamplerReductionMode`]
- [`SemaphoreType`]
- [`SemaphoreWaitFlagBits`]
- [`ShaderFloatControlsIndependence`]

### New Bitmasks

- [`DescriptorBindingFlags`]
- [`ResolveModeFlags`]
- [`SemaphoreWaitFlags`]

### New Enum Constants

- [`MAX_DRIVER_INFO_SIZE`]
- [`MAX_DRIVER_NAME_SIZE`]
- Extending [`BufferCreateFlagBits`]:  - `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT` 
- Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` 
- Extending [`DescriptorPoolCreateFlagBits`]:  - `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` 
- Extending [`DescriptorSetLayoutCreateFlagBits`]:  - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` 
- Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT` 
- Extending [`FramebufferCreateFlagBits`]:  - `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT` 
- Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`  - `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`  - `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`  - `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` 
- Extending [`MemoryAllocateFlagBits`]:  - `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`  - `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT` 
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_FRAGMENTATION`  - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS` 
- Extending [`SamplerAddressMode`]:  - `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`  - `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`  - `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`  - `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`  - `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`  - `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`  - `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`  - `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`  - `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`  - `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`  - `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`  - `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`  - `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`  - `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`  - `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`  - `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`  - `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`  - `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`  - `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`  - `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`  - `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`

# Related
- [`crate::vulkan1_0`]
- [`crate::vulkan1_1`]
- [`crate::vulkan1_3`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        