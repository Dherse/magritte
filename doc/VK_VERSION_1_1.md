[VK_VERSION_1_1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html) - Vulkan version 1.1

# Description
Vulkan Version 1.1 [promoted](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-compatibility-promotion) a
number of key extensions into the core API:
- `[`VK_KHR_16bit_storage`]`
- `[`VK_KHR_bind_memory2`]`
- `[`VK_KHR_dedicated_allocation`]`
- `[`VK_KHR_descriptor_update_template`]`
- `[`VK_KHR_device_group`]`
- `[`VK_KHR_device_group_creation`]`
- `[`VK_KHR_external_fence`]`
- `[`VK_KHR_external_fence_capabilities`]`
- `[`VK_KHR_external_memory`]`
- `[`VK_KHR_external_memory_capabilities`]`
- `[`VK_KHR_external_semaphore`]`
- `[`VK_KHR_external_semaphore_capabilities`]`
- `[`VK_KHR_get_memory_requirements2`]`
- `[`VK_KHR_get_physical_device_properties2`]`
- `[`VK_KHR_maintenance1`]`
- `[`VK_KHR_maintenance2`]`
- `[`VK_KHR_maintenance3`]`
- `[`VK_KHR_multiview`]`
- `[`VK_KHR_relaxed_block_layout`]`
- `[`VK_KHR_sampler_ycbcr_conversion`]`
- `[`VK_KHR_shader_draw_parameters`]`
- `[`VK_KHR_storage_buffer_storage_class`]`
- `[`VK_KHR_variable_pointers`]`
All differences in behavior between these extensions and the corresponding
Vulkan 1.1 functionality are summarized in the [Vulkan 1.1 specification appendix](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions).
### New Macros

- [`crate::Version::VULKAN1_1`]

### New Object Types

- [`DescriptorUpdateTemplate`]
- [`SamplerYcbcrConversion`]

### New Commands

- [`bind_buffer_memory2`]
- [`bind_image_memory2`]
- [`cmd_dispatch_base`]
- [`cmd_set_device_mask`]
- [`create_descriptor_update_template`]
- [`create_sampler_ycbcr_conversion`]
- [`destroy_descriptor_update_template`]
- [`destroy_sampler_ycbcr_conversion`]
- [`enumerate_instance_version`]
- [`enumerate_physical_device_groups`]
- [`get_buffer_memory_requirements2`]
- [`get_descriptor_set_layout_support`]
- [`get_device_group_peer_memory_features`]
- [`get_device_queue2`]
- [`get_image_memory_requirements2`]
- [`get_image_sparse_memory_requirements2`]
- [`get_physical_device_external_buffer_properties`]
- [`get_physical_device_external_fence_properties`]
- [`get_physical_device_external_semaphore_properties`]
- [`get_physical_device_features2`]
- [`get_physical_device_format_properties2`]
- [`get_physical_device_image_format_properties2`]
- [`get_physical_device_memory_properties2`]
- [`get_physical_device_properties2`]
- [`get_physical_device_queue_family_properties2`]
- [`get_physical_device_sparse_image_format_properties2`]
- [`trim_command_pool`]
- [`update_descriptor_set_with_template`]

### New Structures

- [`BindBufferMemoryInfo`]
- [`BindImageMemoryInfo`]
- [`BufferMemoryRequirementsInfo2`]
- [`DescriptorSetLayoutSupport`]
- [`DescriptorUpdateTemplateCreateInfo`]
- [`DescriptorUpdateTemplateEntry`]
- [`DeviceQueueInfo2`]
- [`ExternalBufferProperties`]
- [`ExternalFenceProperties`]
- [`ExternalMemoryProperties`]
- [`ExternalSemaphoreProperties`]
- [`FormatProperties2`]
- [`ImageFormatProperties2`]
- [`ImageMemoryRequirementsInfo2`]
- [`ImageSparseMemoryRequirementsInfo2`]
- [`InputAttachmentAspectReference`]
- [`MemoryRequirements2`]
- [`PhysicalDeviceExternalBufferInfo`]
- [`PhysicalDeviceExternalFenceInfo`]
- [`PhysicalDeviceExternalSemaphoreInfo`]
- [`PhysicalDeviceGroupProperties`]
- [`PhysicalDeviceImageFormatInfo2`]
- [`PhysicalDeviceMemoryProperties2`]
- [`PhysicalDeviceProperties2`]
- [`PhysicalDeviceSparseImageFormatInfo2`]
- [`QueueFamilyProperties2`]
- [`SamplerYcbcrConversionCreateInfo`]
- [`SparseImageFormatProperties2`]
- [`SparseImageMemoryRequirements2`]
- Extending [`BindBufferMemoryInfo`]:  - [`BindBufferMemoryDeviceGroupInfo`] 
- Extending [`BindImageMemoryInfo`]:  - [`BindImageMemoryDeviceGroupInfo`]  - [`BindImagePlaneMemoryInfo`] 
- Extending [`BindSparseInfo`]:  - [`DeviceGroupBindSparseInfo`] 
- Extending [`BufferCreateInfo`]:  - [`ExternalMemoryBufferCreateInfo`] 
- Extending [`CommandBufferBeginInfo`]:  - [`DeviceGroupCommandBufferBeginInfo`] 
- Extending [`DeviceCreateInfo`]:  - [`DeviceGroupDeviceCreateInfo`]  - [`PhysicalDeviceFeatures2`] 
- Extending [`FenceCreateInfo`]:  - [`ExportFenceCreateInfo`] 
- Extending [`ImageCreateInfo`]:  - [`ExternalMemoryImageCreateInfo`] 
- Extending [`ImageFormatProperties2`]:  - [`ExternalImageFormatProperties`]  - [`SamplerYcbcrConversionImageFormatProperties`] 
- Extending [`ImageMemoryRequirementsInfo2`]:  - [`ImagePlaneMemoryRequirementsInfo`] 
- Extending [`ImageViewCreateInfo`]:  - [`ImageViewUsageCreateInfo`] 
- Extending [`MemoryAllocateInfo`]:  - [`ExportMemoryAllocateInfo`]  - [`MemoryAllocateFlagsInfo`]  - [`MemoryDedicatedAllocateInfo`] 
- Extending [`MemoryRequirements2`]:  - [`MemoryDedicatedRequirements`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevice16BitStorageFeatures`]  - [`PhysicalDeviceMultiviewFeatures`]  - [`PhysicalDeviceProtectedMemoryFeatures`]  - [`PhysicalDeviceSamplerYcbcrConversionFeatures`]  - [`PhysicalDeviceShaderDrawParameterFeatures`]  - [`PhysicalDeviceShaderDrawParametersFeatures`]  - [`PhysicalDeviceVariablePointerFeatures`]  - [`PhysicalDeviceVariablePointersFeatures`] 
- Extending [`PhysicalDeviceImageFormatInfo2`]:  - [`PhysicalDeviceExternalImageFormatInfo`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceIdProperties`]  - [`PhysicalDeviceMaintenance3Properties`]  - [`PhysicalDeviceMultiviewProperties`]  - [`PhysicalDevicePointClippingProperties`]  - [`PhysicalDeviceProtectedMemoryProperties`]  - [`PhysicalDeviceSubgroupProperties`] 
- Extending [`PipelineTessellationStateCreateInfo`]:  - [`PipelineTessellationDomainOriginStateCreateInfo`] 
- Extending [`RenderPassBeginInfo`], [`RenderingInfo`]:  - [`DeviceGroupRenderPassBeginInfo`] 
- Extending [`RenderPassCreateInfo`]:  - [`RenderPassInputAttachmentAspectCreateInfo`]  - [`RenderPassMultiviewCreateInfo`] 
- Extending [`SamplerCreateInfo`], [`ImageViewCreateInfo`]:  - [`SamplerYcbcrConversionInfo`] 
- Extending [`SemaphoreCreateInfo`]:  - [`ExportSemaphoreCreateInfo`] 
- Extending [`SubmitInfo`]:  - [`DeviceGroupSubmitInfo`]  - [`ProtectedSubmitInfo`] 

### New Enums

- [`ChromaLocation`]
- [`DescriptorUpdateTemplateType`]
- [`DeviceQueueCreateFlagBits`]
- [`ExternalFenceFeatureFlagBits`]
- [`ExternalFenceHandleTypeFlagBits`]
- [`ExternalMemoryFeatureFlagBits`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`ExternalSemaphoreFeatureFlagBits`]
- [`ExternalSemaphoreHandleTypeFlagBits`]
- [`FenceImportFlagBits`]
- [`MemoryAllocateFlagBits`]
- [`PeerMemoryFeatureFlagBits`]
- [`PointClippingBehavior`]
- [`SamplerYcbcrModelConversion`]
- [`SamplerYcbcrRange`]
- [`SemaphoreImportFlagBits`]
- [`SubgroupFeatureFlagBits`]
- [`TessellationDomainOrigin`]

### New Bitmasks

- [`CommandPoolTrimFlags`]
- [`DescriptorUpdateTemplateCreateFlags`]
- [`ExternalFenceFeatureFlags`]
- [`ExternalFenceHandleTypeFlags`]
- [`ExternalMemoryFeatureFlags`]
- [`ExternalMemoryHandleTypeFlags`]
- [`ExternalSemaphoreFeatureFlags`]
- [`ExternalSemaphoreHandleTypeFlags`]
- [`FenceImportFlags`]
- [`MemoryAllocateFlags`]
- [`PeerMemoryFeatureFlags`]
- [`SemaphoreImportFlags`]
- [`SubgroupFeatureFlags`]

### New Enum Constants

- [`LUID_SIZE`]
- [`MAX_DEVICE_GROUP_SIZE`]
- [`QUEUE_FAMILY_EXTERNAL`]
- Extending [`BufferCreateFlagBits`]:  - `VK_BUFFER_CREATE_PROTECTED_BIT` 
- Extending [`CommandPoolCreateFlagBits`]:  - `VK_COMMAND_POOL_CREATE_PROTECTED_BIT` 
- Extending [`DependencyFlagBits`]:  - `VK_DEPENDENCY_DEVICE_GROUP_BIT`  - `VK_DEPENDENCY_VIEW_LOCAL_BIT` 
- Extending [`DeviceQueueCreateFlagBits`]:  - `VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT` 
- Extending [`Format`]:  - `VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16`  - `VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16`  - `VK_FORMAT_B16G16R16G16_422_UNORM`  - `VK_FORMAT_B8G8R8G8_422_UNORM`  - `VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16`  - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16`  - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16`  - `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16`  - `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16`  - `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16`  - `VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16`  - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16`  - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16`  - `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16`  - `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16`  - `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16`  - `VK_FORMAT_G16B16G16R16_422_UNORM`  - `VK_FORMAT_G16_B16R16_2PLANE_420_UNORM`  - `VK_FORMAT_G16_B16R16_2PLANE_422_UNORM`  - `VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM`  - `VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM`  - `VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM`  - `VK_FORMAT_G8B8G8R8_422_UNORM`  - `VK_FORMAT_G8_B8R8_2PLANE_420_UNORM`  - `VK_FORMAT_G8_B8R8_2PLANE_422_UNORM`  - `VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM`  - `VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM`  - `VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM`  - `VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`  - `VK_FORMAT_R10X6G10X6_UNORM_2PACK16`  - `VK_FORMAT_R10X6_UNORM_PACK16`  - `VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16`  - `VK_FORMAT_R12X4G12X4_UNORM_2PACK16`  - `VK_FORMAT_R12X4_UNORM_PACK16` 
- Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`  - `VK_FORMAT_FEATURE_DISJOINT_BIT`  - `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`  - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`  - `VK_FORMAT_FEATURE_TRANSFER_DST_BIT`  - `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT` 
- Extending [`ImageAspectFlagBits`]:  - `VK_IMAGE_ASPECT_PLANE_0_BIT`  - `VK_IMAGE_ASPECT_PLANE_1_BIT`  - `VK_IMAGE_ASPECT_PLANE_2_BIT` 
- Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT`  - `VK_IMAGE_CREATE_ALIAS_BIT`  - `VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT`  - `VK_IMAGE_CREATE_DISJOINT_BIT`  - `VK_IMAGE_CREATE_EXTENDED_USAGE_BIT`  - `VK_IMAGE_CREATE_PROTECTED_BIT`  - `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT` 
- Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`  - `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL` 
- Extending [`MemoryHeapFlagBits`]:  - `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` 
- Extending [`MemoryPropertyFlagBits`]:  - `VK_MEMORY_PROPERTY_PROTECTED_BIT` 
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE`  - `VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION` 
- Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_DISPATCH_BASE`  - `VK_PIPELINE_CREATE_DISPATCH_BASE_BIT`  - `VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT` 
- Extending [`QueueFlagBits`]:  - `VK_QUEUE_PROTECTED_BIT` 
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`  - `VK_ERROR_OUT_OF_POOL_MEMORY` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`  - `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`  - `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`  - `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`  - `VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`  - `VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`  - `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`  - `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`  - `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`  - `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`  - `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`  - `VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`  - `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`  - `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`  - `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`  - `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`  - `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`  - `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`  - `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`  - `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`  - `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`  - `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`  - `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES`  - `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`  - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`  - `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`  - `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`  - `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`  - `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`  - `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`  - `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`  - `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`

# Related
- [`crate::vulkan1_0`]
- [`crate::vulkan1_2`]
- [`crate::vulkan1_3`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        