[VkPipelineLayoutCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayoutCreateInfo.html) - Structure specifying the parameters of a newly created pipeline layout object

# C Specifications
The [`PipelineLayoutCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineLayoutCreateInfo {
    VkStructureType                 sType;
    const void*                     pNext;
    VkPipelineLayoutCreateFlags     flags;
    uint32_t                        setLayoutCount;
    const VkDescriptorSetLayout*    pSetLayouts;
    uint32_t                        pushConstantRangeCount;
    const VkPushConstantRange*      pPushConstantRanges;
} VkPipelineLayoutCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`set_layout_count`] is the number of descriptor sets included in the pipeline layout.
- [`set_layouts`] is a pointer to an array of [`DescriptorSetLayout`] objects.
- [`push_constant_range_count`] is the number of push constant ranges included in the pipeline layout.
- [`push_constant_ranges`] is a pointer to an array of [`PushConstantRange`] structures defining a set of push constant ranges for use in a single pipeline layout. In addition to descriptor set layouts, a pipeline layout also describes how many push constants  **can**  be accessed by each stage of the pipeline.

# Description
## Valid Usage
-  [`set_layout_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_bound_descriptor_sets`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_SAMPLER` and `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_descriptor_samplers`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` and `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_descriptor_uniform_buffers`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` and `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_descriptor_storage_buffers`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, and `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_descriptor_sampled_images`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, and `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_descriptor_storage_images`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_per_stage_descriptor_input_attachments`]
-    The total number of bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceInlineUniformBlockProperties::max_per_stage_descriptor_inline_uniform_blocks`]
-    The total number of descriptors with a `descriptorType` of `VK_DESCRIPTOR_TYPE_SAMPLER` and `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_samplers`]
-    The total number of descriptors with a `descriptorType` of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` and `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
-    The total number of descriptors with a `descriptorType` of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` and `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_storage_buffers`]
-    The total number of descriptors with a `descriptorType` of `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, and `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_sampled_images`]
-    The total number of descriptors with a `descriptorType` of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, and `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_storage_images`]
-    The total number of descriptors with a `descriptorType` of `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_per_stage_descriptor_update_after_bind_input_attachments`]
-    The total number of bindings with a `descriptorType` of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceInlineUniformBlockProperties::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_SAMPLER` and `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_samplers`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_uniform_buffers`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_uniform_buffers_dynamic`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_storage_buffers`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_storage_buffers_dynamic`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, and `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_sampled_images`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, and `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_storage_images`]
-    The total number of descriptors in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_descriptor_set_input_attachments`]
-    The total number of bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceInlineUniformBlockProperties::max_descriptor_set_inline_uniform_blocks`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_SAMPLER` and `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_samplers`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_uniform_buffers`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_storage_buffers`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, and `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_sampled_images`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`, and `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_storage_images`]
-    The total number of descriptors of the type `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceDescriptorIndexingProperties::max_descriptor_set_update_after_bind_input_attachments`]
-    The total number of bindings with a `descriptorType` of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceInlineUniformBlockProperties::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
-    Any two elements of [`push_constant_ranges`] **must**  not include the same stage in `stageFlags`
-  [`set_layouts`] **must**  not contain more than one descriptor set layout that was created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set
-    The total number of bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_per_stage_descriptor_acceleration_structures`]
-    The total number of bindings with a `descriptorType` of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` accessible to any given shader stage across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_per_stage_descriptor_update_after_bind_acceleration_structures`]
-    The total number of bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set with a `descriptorType` of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_descriptor_set_acceleration_structures`]
-    The total number of bindings with a `descriptorType` of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_descriptor_set_update_after_bind_acceleration_structures`]
-    The total number of bindings with a `descriptorType` of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV` accessible across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_descriptor_set_acceleration_structures`]
-    The total number of `pImmutableSamplers` created with [`flags`] containing `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT` or `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT` across all shader stages and across all elements of [`set_layouts`] **must**  be less than or equal to [[`PhysicalDeviceFragmentDensityMap2PropertiesEXT::max_descriptor_set_subsampled_samplers`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxDescriptorSetSubsampledSamplers)
-    Any element of [`set_layouts`] **must**  not have been created with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE` bit set

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-    If [`set_layout_count`] is not `0`, [`set_layouts`] **must**  be a valid pointer to an array of [`set_layout_count`] valid [`DescriptorSetLayout`] handles
-    If [`push_constant_range_count`] is not `0`, [`push_constant_ranges`] **must**  be a valid pointer to an array of [`push_constant_range_count`] valid [`PushConstantRange`] structures

# Related
- [`crate::vulkan1_0`]
- [`DescriptorSetLayout`]
- [`PipelineLayoutCreateFlags`]
- [`PushConstantRange`]
- [`StructureType`]
- [`create_pipeline_layout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        