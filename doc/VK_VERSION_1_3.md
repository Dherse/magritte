[VK_VERSION_1_3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_3.html) - Vulkan version 1.3

# Description
Vulkan Version 1.3 [promoted](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-compatibility-promotion) a
number of key extensions into the core API:
- `[`VK_KHR_copy_commands2`]`
- `[`VK_KHR_dynamic_rendering`]`
- `[`VK_KHR_format_feature_flags2`]`
- `[`VK_KHR_maintenance4`]`
- `[`VK_KHR_shader_integer_dot_product`]`
- `[`VK_KHR_shader_non_semantic_info`]`
- `[`VK_KHR_shader_terminate_invocation`]`
- `[`VK_KHR_synchronization2`]`
- `[`VK_KHR_zero_initialize_workgroup_memory`]`
- `[`VK_EXT_4444_formats`]`
- `[`VK_EXT_extended_dynamic_state`]`
- `[`VK_EXT_extended_dynamic_state2`]`
- `[`VK_EXT_image_robustness`]`
- `[`VK_EXT_inline_uniform_block`]`
- `[`VK_EXT_pipeline_creation_cache_control`]`
- `[`VK_EXT_pipeline_creation_feedback`]`
- `[`VK_EXT_private_data`]`
- `[`VK_EXT_shader_demote_to_helper_invocation`]`
- `[`VK_EXT_subgroup_size_control`]`
- `[`VK_EXT_texel_buffer_alignment`]`
- `[`VK_EXT_texture_compression_astc_hdr`]`
- `[`VK_EXT_tooling_info`]`
- `[`VK_EXT_ycbcr_2plane_444_formats`]`
All differences in behavior between these extensions and the corresponding
Vulkan 1.3 functionality are summarized in the [Vulkan 1.3 specification appendix](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions).
### New Macros

- [`crate::Version::VULKAN1_3`]

### New Object Types

- [`PrivateDataSlot`]

### New Commands

- [`cmd_begin_rendering`]
- [`cmd_bind_vertex_buffers2`]
- [`cmd_blit_image2`]
- [`cmd_copy_buffer2`]
- [`cmd_copy_buffer_to_image2`]
- [`cmd_copy_image2`]
- [`cmd_copy_image_to_buffer2`]
- [`cmd_end_rendering`]
- [`cmd_pipeline_barrier2`]
- [`cmd_reset_event2`]
- [`cmd_resolve_image2`]
- [`cmd_set_cull_mode`]
- [`cmd_set_depth_bias_enable`]
- [`cmd_set_depth_bounds_test_enable`]
- [`cmd_set_depth_compare_op`]
- [`cmd_set_depth_test_enable`]
- [`cmd_set_depth_write_enable`]
- [`cmd_set_event2`]
- [`cmd_set_front_face`]
- [`cmd_set_primitive_restart_enable`]
- [`cmd_set_primitive_topology`]
- [`cmd_set_rasterizer_discard_enable`]
- [`cmd_set_scissor_with_count`]
- [`cmd_set_stencil_op`]
- [`cmd_set_stencil_test_enable`]
- [`cmd_set_viewport_with_count`]
- [`cmd_wait_events2`]
- [`cmd_write_timestamp2`]
- [`create_private_data_slot`]
- [`destroy_private_data_slot`]
- [`get_device_buffer_memory_requirements`]
- [`get_device_image_memory_requirements`]
- [`get_device_image_sparse_memory_requirements`]
- [`get_physical_device_tool_properties`]
- [`get_private_data`]
- [`queue_submit2`]
- [`set_private_data`]

### New Structures

- [`BlitImageInfo2`]
- [`BufferCopy2`]
- [`BufferImageCopy2`]
- [`BufferMemoryBarrier2`]
- [`CommandBufferSubmitInfo`]
- [`CopyBufferInfo2`]
- [`CopyBufferToImageInfo2`]
- [`CopyImageInfo2`]
- [`CopyImageToBufferInfo2`]
- [`DependencyInfo`]
- [`DeviceBufferMemoryRequirements`]
- [`DeviceImageMemoryRequirements`]
- [`ImageBlit2`]
- [`ImageCopy2`]
- [`ImageMemoryBarrier2`]
- [`ImageResolve2`]
- [`PhysicalDeviceToolProperties`]
- [`PipelineCreationFeedback`]
- [`PrivateDataSlotCreateInfo`]
- [`RenderingAttachmentInfo`]
- [`RenderingInfo`]
- [`ResolveImageInfo2`]
- [`SemaphoreSubmitInfo`]
- [`SubmitInfo2`]
- Extending [`CommandBufferInheritanceInfo`]:  - [`CommandBufferInheritanceRenderingInfo`] 
- Extending [`DescriptorPoolCreateInfo`]:  - [`DescriptorPoolInlineUniformBlockCreateInfo`] 
- Extending [`DeviceCreateInfo`]:  - [`DevicePrivateDataCreateInfo`] 
- Extending [`FormatProperties2`]:  - [`FormatProperties3`] 
- Extending [`GraphicsPipelineCreateInfo`]:  - [`PipelineRenderingCreateInfo`] 
- Extending [`GraphicsPipelineCreateInfo`], [`ComputePipelineCreateInfo`], [`RayTracingPipelineCreateInfoNV`], [`RayTracingPipelineCreateInfoKHR`]:  - [`PipelineCreationFeedbackCreateInfo`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceDynamicRenderingFeatures`]  - [`PhysicalDeviceImageRobustnessFeatures`]  - [`PhysicalDeviceInlineUniformBlockFeatures`]  - [`PhysicalDeviceMaintenance4Features`]  - [`PhysicalDevicePipelineCreationCacheControlFeatures`]  - [`PhysicalDevicePrivateDataFeatures`]  - [`PhysicalDeviceShaderDemoteToHelperInvocationFeatures`]  - [`PhysicalDeviceShaderIntegerDotProductFeatures`]  - [`PhysicalDeviceShaderTerminateInvocationFeatures`]  - [`PhysicalDeviceSubgroupSizeControlFeatures`]  - [`PhysicalDeviceSynchronization2Features`]  - [`PhysicalDeviceTextureCompressionAstchdrFeatures`]  - [`PhysicalDeviceVulkan13Features`]  - [`PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceInlineUniformBlockProperties`]  - [`PhysicalDeviceMaintenance4Properties`]  - [`PhysicalDeviceShaderIntegerDotProductProperties`]  - [`PhysicalDeviceSubgroupSizeControlProperties`]  - [`PhysicalDeviceTexelBufferAlignmentProperties`]  - [`PhysicalDeviceVulkan13Properties`] 
- Extending [`PipelineShaderStageCreateInfo`]:  - [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] 
- Extending [`SubpassDependency2`]:  - [`MemoryBarrier2`] 
- Extending [`WriteDescriptorSet`]:  - [`WriteDescriptorSetInlineUniformBlock`] 

### New Enums

- [`AccessFlagBits2`]
- [`FormatFeatureFlagBits2`]
- [`PipelineCreationFeedbackFlagBits`]
- [`PipelineStageFlagBits2`]
- [`RenderingFlagBits`]
- [`SubmitFlagBits`]
- [`ToolPurposeFlagBits`]

### New Bitmasks

- [`AccessFlags2`]
- [`FormatFeatureFlags2`]
- [`PipelineCreationFeedbackFlags`]
- [`PipelineStageFlags2`]
- [`PrivateDataSlotCreateFlags`]
- [`RenderingFlags`]
- [`SubmitFlags`]
- [`ToolPurposeFlags`]

### New Enum Constants

- Extending [`AccessFlagBits`]:  - `VK_ACCESS_NONE` 
- Extending [`AttachmentStoreOp`]:  - `VK_ATTACHMENT_STORE_OP_NONE` 
- Extending [`DescriptorType`]:  - `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` 
- Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_CULL_MODE`  - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`  - `VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE`  - `VK_DYNAMIC_STATE_DEPTH_COMPARE_OP`  - `VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE`  - `VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE`  - `VK_DYNAMIC_STATE_FRONT_FACE`  - `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE`  - `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY`  - `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE`  - `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT`  - `VK_DYNAMIC_STATE_STENCIL_OP`  - `VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE`  - `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE`  - `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` 
- Extending [`EventCreateFlagBits`]:  - `VK_EVENT_CREATE_DEVICE_ONLY_BIT` 
- Extending [`Format`]:  - `VK_FORMAT_A4B4G4R4_UNORM_PACK16`  - `VK_FORMAT_A4R4G4B4_UNORM_PACK16`  - `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`  - `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16`  - `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16`  - `VK_FORMAT_G16_B16R16_2PLANE_444_UNORM`  - `VK_FORMAT_G8_B8R8_2PLANE_444_UNORM` 
- Extending [`ImageAspectFlagBits`]:  - `VK_IMAGE_ASPECT_NONE` 
- Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL`  - `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL` 
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_PRIVATE_DATA_SLOT` 
- Extending [`PipelineCacheCreateFlagBits`]:  - `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT` 
- Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`  - `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT` 
- Extending [`PipelineShaderStageCreateFlagBits`]:  - `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT`  - `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` 
- Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_NONE` 
- Extending [`VulkanResultCodes`]:  - `VK_PIPELINE_COMPILE_REQUIRED` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2`  - `VK_STRUCTURE_TYPE_BUFFER_COPY_2`  - `VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2`  - `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2`  - `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO`  - `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO`  - `VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2`  - `VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2`  - `VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2`  - `VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2`  - `VK_STRUCTURE_TYPE_DEPENDENCY_INFO`  - `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO`  - `VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS`  - `VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS`  - `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO`  - `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3`  - `VK_STRUCTURE_TYPE_IMAGE_BLIT_2`  - `VK_STRUCTURE_TYPE_IMAGE_COPY_2`  - `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2`  - `VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2`  - `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES`  - `VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO`  - `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO`  - `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO`  - `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO`  - `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO`  - `VK_STRUCTURE_TYPE_RENDERING_INFO`  - `VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2`  - `VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO`  - `VK_STRUCTURE_TYPE_SUBMIT_INFO_2`  - `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK`

# Related
- [`crate::vulkan1_0`]
- [`crate::vulkan1_1`]
- [`crate::vulkan1_2`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        