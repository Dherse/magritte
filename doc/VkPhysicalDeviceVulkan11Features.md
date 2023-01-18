[VkPhysicalDeviceVulkan11Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html) - Structure describing the Vulkan 1.1 features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceVulkan11Features`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDeviceVulkan11Features {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           storageBuffer16BitAccess;
    VkBool32           uniformAndStorageBuffer16BitAccess;
    VkBool32           storagePushConstant16;
    VkBool32           storageInputOutput16;
    VkBool32           multiview;
    VkBool32           multiviewGeometryShader;
    VkBool32           multiviewTessellationShader;
    VkBool32           variablePointersStorageBuffer;
    VkBool32           variablePointers;
    VkBool32           protectedMemory;
    VkBool32           samplerYcbcrConversion;
    VkBool32           shaderDrawParameters;
} VkPhysicalDeviceVulkan11Features;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`storage_buffer16_bit_access`] specifies whether objects in the     `StorageBuffer`, `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block` decoration  **can**  have 16-bit integer     and 16-bit floating-point members.     If this feature is not enabled, 16-bit integer or 16-bit floating-point     members  **must**  not be used in such objects.     This also specifies whether shader modules  **can**  declare the     `StorageBuffer16BitAccess` capability.
- [`uniform_and_storage_buffer16_bit_access`] specifies whether objects in the `Uniform` storage class with the `Block` decoration  **can**  have 16-bit integer and 16-bit floating-point members. If this feature is not enabled, 16-bit integer or 16-bit floating-point members  **must**  not be used in such objects. This also specifies whether shader modules  **can**  declare the `UniformAndStorageBuffer16BitAccess` capability.
- [`storage_push_constant16`] specifies whether objects in the `PushConstant` storage class  **can**  have 16-bit integer and 16-bit floating-point members. If this feature is not enabled, 16-bit integer or floating-point members  **must**  not be used in such objects. This also specifies whether shader modules  **can**  declare the `StoragePushConstant16` capability.
- [`storage_input_output16`] specifies whether objects in the `Input` and `Output` storage classes  **can**  have 16-bit integer and 16-bit floating-point members. If this feature is not enabled, 16-bit integer or 16-bit floating-point members  **must**  not be used in such objects. This also specifies whether shader modules  **can**  declare the `StorageInputOutput16` capability.
- [`multiview`] specifies whether the implementation supports multiview rendering within a render pass. If this feature is not enabled, the view mask of each subpass  **must**  always be zero.
- [`multiview_geometry_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [geometry shaders](). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include a geometry shader.
- [`multiview_tessellation_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [tessellation shaders](). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include any tessellation shaders.
- [`variable_pointers_storage_buffer`] specifies whether the implementation supports the SPIR-V `VariablePointersStorageBuffer` capability. When this feature is not enabled, shader modules  **must**  not declare the `SPV_KHR_variable_pointers` extension or the `VariablePointersStorageBuffer` capability.
- [`variable_pointers`] specifies whether the implementation supports the SPIR-V `VariablePointers` capability. When this feature is not enabled, shader modules  **must**  not declare the `VariablePointers` capability.
- [`protected_memory`] specifies whether protected memory is supported.
- [`sampler_ycbcr_conversion`] specifies whether the implementation supports [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](). If [`sampler_ycbcr_conversion`] is [`FALSE`], sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is not supported, and samplers using sampler Y′C<sub>B</sub>C<sub>R</sub> conversion  **must**  not be used.
- [`shader_draw_parameters`] specifies whether the implementation supports the SPIR-V `DrawParameters` capability. When this feature is not enabled, shader modules  **must**  not declare the `SPV_KHR_shader_draw_parameters` extension or the `DrawParameters` capability.
If the [`PhysicalDeviceVulkan11Features`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceVulkan11Features`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`

# Related
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        