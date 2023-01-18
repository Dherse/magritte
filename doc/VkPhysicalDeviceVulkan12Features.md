[VkPhysicalDeviceVulkan12Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html) - Structure describing the Vulkan 1.2 features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceVulkan12Features`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceVulkan12Features {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           samplerMirrorClampToEdge;
    VkBool32           drawIndirectCount;
    VkBool32           storageBuffer8BitAccess;
    VkBool32           uniformAndStorageBuffer8BitAccess;
    VkBool32           storagePushConstant8;
    VkBool32           shaderBufferInt64Atomics;
    VkBool32           shaderSharedInt64Atomics;
    VkBool32           shaderFloat16;
    VkBool32           shaderInt8;
    VkBool32           descriptorIndexing;
    VkBool32           shaderInputAttachmentArrayDynamicIndexing;
    VkBool32           shaderUniformTexelBufferArrayDynamicIndexing;
    VkBool32           shaderStorageTexelBufferArrayDynamicIndexing;
    VkBool32           shaderUniformBufferArrayNonUniformIndexing;
    VkBool32           shaderSampledImageArrayNonUniformIndexing;
    VkBool32           shaderStorageBufferArrayNonUniformIndexing;
    VkBool32           shaderStorageImageArrayNonUniformIndexing;
    VkBool32           shaderInputAttachmentArrayNonUniformIndexing;
    VkBool32           shaderUniformTexelBufferArrayNonUniformIndexing;
    VkBool32           shaderStorageTexelBufferArrayNonUniformIndexing;
    VkBool32           descriptorBindingUniformBufferUpdateAfterBind;
    VkBool32           descriptorBindingSampledImageUpdateAfterBind;
    VkBool32           descriptorBindingStorageImageUpdateAfterBind;
    VkBool32           descriptorBindingStorageBufferUpdateAfterBind;
    VkBool32           descriptorBindingUniformTexelBufferUpdateAfterBind;
    VkBool32           descriptorBindingStorageTexelBufferUpdateAfterBind;
    VkBool32           descriptorBindingUpdateUnusedWhilePending;
    VkBool32           descriptorBindingPartiallyBound;
    VkBool32           descriptorBindingVariableDescriptorCount;
    VkBool32           runtimeDescriptorArray;
    VkBool32           samplerFilterMinmax;
    VkBool32           scalarBlockLayout;
    VkBool32           imagelessFramebuffer;
    VkBool32           uniformBufferStandardLayout;
    VkBool32           shaderSubgroupExtendedTypes;
    VkBool32           separateDepthStencilLayouts;
    VkBool32           hostQueryReset;
    VkBool32           timelineSemaphore;
    VkBool32           bufferDeviceAddress;
    VkBool32           bufferDeviceAddressCaptureReplay;
    VkBool32           bufferDeviceAddressMultiDevice;
    VkBool32           vulkanMemoryModel;
    VkBool32           vulkanMemoryModelDeviceScope;
    VkBool32           vulkanMemoryModelAvailabilityVisibilityChains;
    VkBool32           shaderOutputViewportIndex;
    VkBool32           shaderOutputLayer;
    VkBool32           subgroupBroadcastDynamicId;
} VkPhysicalDeviceVulkan12Features;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`sampler_mirror_clamp_to_edge`] indicates whether the implementation supports the `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode. If this feature is not enabled, the `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode  **must**  not be used.
- [`draw_indirect_count`] indicates whether the implementation supports the [`cmd_draw_indirect_count`] and [`cmd_draw_indexed_indirect_count`] functions. If this feature is not enabled, these functions  **must**  not be used.
- [`storage_buffer8_bit_access`] indicates whether objects in the     `StorageBuffer`, `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block` decoration  **can**  have 8-bit integer     members.     If this feature is not enabled, 8-bit integer members  **must**  not be used     in such objects.     This also indicates whether shader modules  **can**  declare the     `StorageBuffer8BitAccess` capability.
- [`uniform_and_storage_buffer8_bit_access`] indicates whether objects in the `Uniform` storage class with the `Block` decoration  **can**  have 8-bit integer members. If this feature is not enabled, 8-bit integer members  **must**  not be used in such objects. This also indicates whether shader modules  **can**  declare the `UniformAndStorageBuffer8BitAccess` capability.
- [`storage_push_constant8`] indicates whether objects in the `PushConstant` storage class  **can**  have 8-bit integer members. If this feature is not enabled, 8-bit integer members  **must**  not be used in such objects. This also indicates whether shader modules  **can**  declare the `StoragePushConstant8` capability.
- [`shader_buffer_int64_atomics`] indicates whether shaders  **can**  perform 64-bit unsigned and signed integer atomic operations on buffers.
- [`shader_shared_int64_atomics`] indicates whether shaders  **can**  perform 64-bit unsigned and signed integer atomic operations on shared memory.
- [`shader_float16`] indicates whether 16-bit floats (halfs) are supported in shader code. This also indicates whether shader modules  **can**  declare the `Float16` capability. However, this only enables a subset of the storage classes that SPIR-V allows for the `Float16` SPIR-V capability: Declaring and using 16-bit floats in the `Private`, `Workgroup` (for non-Block variables), and `Function` storage classes is enabled, while declaring them in the interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`, `Output`, and `PushConstant`) is not enabled.
- [`shader_int8`] indicates whether 8-bit integers (signed and unsigned) are supported in shader code. This also indicates whether shader modules  **can**  declare the `Int8` capability. However, this only enables a subset of the storage classes that SPIR-V allows for the `Int8` SPIR-V capability: Declaring and using 8-bit integers in the `Private`, `Workgroup` (for non-Block variables), and `Function` storage classes is enabled, while declaring them in the interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`, `Output`, and `PushConstant`) is not enabled.
- [`descriptor_indexing`] indicates whether the implementation supports the minimum set of descriptor indexing features as described in the [Feature Requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-requirements) section. Enabling the [`descriptor_indexing`] member when [`create_device`] is called does not imply the other minimum descriptor indexing features are also enabled. Those other descriptor indexing features  **must**  be enabled individually as needed by the application.
- [`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays of input attachments  **can**  be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  be indexed only by constant integral expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `InputAttachmentArrayDynamicIndexing` capability.
- [`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether arrays of uniform texel buffers  **can**  be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  be indexed only by constant integral expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `UniformTexelBufferArrayDynamicIndexing` capability.
- [`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether arrays of storage texel buffers  **can**  be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  be indexed only by constant integral expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `StorageTexelBufferArrayDynamicIndexing` capability.
- [`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**  not be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `UniformBufferArrayNonUniformIndexing` capability.
- [`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays of samplers or sampled images  **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**  not be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `SampledImageArrayNonUniformIndexing` capability.
- [`shader_storage_buffer_array_non_uniform_indexing`] indicates whether arrays of storage buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**  not be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `StorageBufferArrayNonUniformIndexing` capability.
- [`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays of storage images  **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  not be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `StorageImageArrayNonUniformIndexing` capability.
- [`shader_input_attachment_array_non_uniform_indexing`] indicates whether arrays of input attachments  **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  not be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `InputAttachmentArrayNonUniformIndexing` capability.
- [`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform texel buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `UniformTexelBufferArrayNonUniformIndexing` capability.
- [`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of storage texel buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  not be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This also indicates whether shader modules  **can**  declare the `StorageTexelBufferArrayNonUniformIndexing` capability.
- [`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether the implementation supports updating uniform buffer descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
- [`descriptor_binding_sampled_image_update_after_bind`] indicates whether the implementation supports updating sampled image descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
- [`descriptor_binding_storage_image_update_after_bind`] indicates whether the implementation supports updating storage image descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
- [`descriptor_binding_storage_buffer_update_after_bind`] indicates whether the implementation supports updating storage buffer descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
- [`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates whether the implementation supports updating uniform texel buffer descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
- [`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates whether the implementation supports updating storage texel buffer descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
- [`descriptor_binding_update_unused_while_pending`] indicates whether the implementation supports updating descriptors while the set is in use. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT` **must**  not be used.
- [`descriptor_binding_partially_bound`] indicates whether the implementation supports statically using a descriptor set binding in which some descriptors are not valid. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT` **must**  not be used.
- [`descriptor_binding_variable_descriptor_count`] indicates whether the implementation supports descriptor sets with a variable-sized last binding. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT` **must**  not be used.
- [`runtime_descriptor_array`] indicates whether the implementation supports the SPIR-V `RuntimeDescriptorArray` capability. If this feature is not enabled, descriptors  **must**  not be declared in runtime arrays.
- [`sampler_filter_minmax`] indicates whether the implementation supports a minimum set of required formats supporting min/max filtering as defined by the [`filterMinmaxSingleComponentFormats`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-filterMinmaxSingleComponentFormats-minimum-requirements) property minimum requirements. If this feature is not enabled, then no [`SamplerCreateInfo`][`p_next`] chain can include a [`SamplerReductionModeCreateInfo`] structure.
- [`scalar_block_layout`] indicates that the implementation supports the layout of resource blocks in shaders using [scalar alignment]().
- [`imageless_framebuffer`] indicates that the implementation supports specifying the image view for attachments at render pass begin time via [`RenderPassAttachmentBeginInfo`].
- [`uniform_buffer_standard_layout`] indicates that the implementation supports the same layouts for uniform buffers as for storage and other kinds of buffers. See [Standard Buffer Layout]().
- [`shader_subgroup_extended_types`] is a boolean specifying whether subgroup operations can use 8-bit integer, 16-bit integer, 64-bit integer, 16-bit floating-point, and vectors of these types in [group operations]() with [subgroup scope](), if the implementation supports the types.
- [`separate_depth_stencil_layouts`] indicates whether the implementation supports a [`ImageMemoryBarrier`] for a depth/stencil image with only one of `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT` set, and whether `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` can be used.
- [`host_query_reset`] indicates that the implementation supports resetting queries from the host with [`reset_query_pool`].
- [`timeline_semaphore`] indicates whether semaphores created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE` are supported.
- [`buffer_device_address`] indicates that the implementation supports accessing buffer memory in shaders as storage buffers via an address queried from [`get_buffer_device_address`].
- [`buffer_device_address_capture_replay`] indicates that the implementation supports saving and reusing buffer and device addresses, e.g. for trace capture and replay.
- [`buffer_device_address_multi_device`] indicates that the implementation supports the [`buffer_device_address`] , `rayTracingPipeline` and `rayQuery` features for logical devices created with multiple physical devices. If this feature is not supported, buffer and acceleration structure addresses  **must**  not be queried on a logical device created with more than one physical device.
- [`vulkan_memory_model`] indicates whether the Vulkan Memory Model is supported, as defined in [Vulkan Memory Model](). This also indicates whether shader modules  **can**  declare the `VulkanMemoryModel` capability.
- [`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory Model can use [`Device`] scope synchronization. This also indicates whether shader modules  **can**  declare the `VulkanMemoryModelDeviceScope` capability.
- [`vulkan_memory_model_availability_visibility_chains`] indicates whether the Vulkan Memory Model can use [availability and visibility chains]() with more than one element.
- [`shader_output_viewport_index`] indicates whether the implementation supports the `ShaderViewportIndex` SPIR-V capability enabling variables decorated with the `ViewportIndex` built-in to be exported from vertex or tessellation evaluation shaders. If this feature is not enabled, the `ViewportIndex` built-in decoration  **must**  not be used on outputs in vertex or tessellation evaluation shaders.
- [`shader_output_layer`] indicates whether the implementation supports the `ShaderLayer` SPIR-V capability enabling variables decorated with the `Layer` built-in to be exported from vertex or tessellation evaluation shaders. If this feature is not enabled, the `Layer` built-in decoration  **must**  not be used on outputs in vertex or tessellation evaluation shaders.
-  If [`subgroup_broadcast_dynamic_id`] is [`TRUE`], the “Id” operand of `OpGroupNonUniformBroadcast` **can**  be dynamically uniform within a subgroup, and the “Index” operand of `OpGroupNonUniformQuadBroadcast` **can**  be dynamically uniform within the derivative group. If it is [`FALSE`], these operands  **must**  be constants.
If the [`PhysicalDeviceVulkan12Features`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceVulkan12Features`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`

# Related
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        