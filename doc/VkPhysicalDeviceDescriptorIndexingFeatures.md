[VkPhysicalDeviceDescriptorIndexingFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html) - Structure describing descriptor indexing features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceDescriptorIndexingFeatures`] structure is defined
as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceDescriptorIndexingFeatures {
    VkStructureType    sType;
    void*              pNext;
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
} VkPhysicalDeviceDescriptorIndexingFeatures;
```
or the equivalent
```c
// Provided by VK_EXT_descriptor_indexing
typedef VkPhysicalDeviceDescriptorIndexingFeatures VkPhysicalDeviceDescriptorIndexingFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

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
If the [`PhysicalDeviceDescriptorIndexingFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceDescriptorIndexingFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`

# Related
- [`ext_descriptor_indexing`]
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        