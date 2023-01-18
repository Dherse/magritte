[VkPhysicalDeviceVulkan13Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html) - Structure describing the Vulkan 1.3 features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceVulkan13Features`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPhysicalDeviceVulkan13Features {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           robustImageAccess;
    VkBool32           inlineUniformBlock;
    VkBool32           descriptorBindingInlineUniformBlockUpdateAfterBind;
    VkBool32           pipelineCreationCacheControl;
    VkBool32           privateData;
    VkBool32           shaderDemoteToHelperInvocation;
    VkBool32           shaderTerminateInvocation;
    VkBool32           subgroupSizeControl;
    VkBool32           computeFullSubgroups;
    VkBool32           synchronization2;
    VkBool32           textureCompressionASTC_HDR;
    VkBool32           shaderZeroInitializeWorkgroupMemory;
    VkBool32           dynamicRendering;
    VkBool32           shaderIntegerDotProduct;
    VkBool32           maintenance4;
} VkPhysicalDeviceVulkan13Features;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`robust_image_access`] indicates whether image accesses are tightly bounds-checked against the dimensions of the image view. [Invalid texels]() resulting from out of bounds image loads will be replaced as described in [Texel Replacement](), with either (0,0,1) or (0,0,0) values inserted for missing G, B, or A components based on the format.
- [`inline_uniform_block`] indicates whether the implementation supports inline uniform block descriptors. If this feature is not enabled, `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` **must**  not be used.
- [`descriptor_binding_inline_uniform_block_update_after_bind`] indicates whether the implementation supports updating inline uniform block descriptors after a set is bound. If this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
- [`pipeline_creation_cache_control`] indicates that the implementation supports:  - The following  **can**  be used in `Vk*PipelineCreateInfo`::`flags`:   - `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`   - `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`   - The following  **can**  be used in [`PipelineCacheCreateInfo::flags`]:   - `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`  
- [`private_data`] indicates whether the implementation supports private data. See [Private Data]().
- [`shader_demote_to_helper_invocation`] indicates whether the implementation supports the SPIR-V `DemoteToHelperInvocationEXT` capability.
- [`shader_terminate_invocation`] specifies whether the implementation supports SPIR-V modules that use the `SPV_KHR_terminate_invocation` extension.
- [`subgroup_size_control`] indicates whether the implementation supports controlling shader subgroup sizes via the `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT` flag and the [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure.
- [`compute_full_subgroups`] indicates whether the implementation supports requiring full subgroups in compute shaders via the `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag.
- [`synchronization2`] indicates whether the implementation supports the new set of synchronization commands introduced in `[`VK_KHR_synchronization2`]`.
- [`texture_compression_astc_hdr`] indicates whether all of the ASTC HDR compressed texture formats are supported. If this feature is enabled, then the `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must**  be supported in `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK` To query for additional properties, or if the feature is not enabled, [`get_physical_device_format_properties`] and [`get_physical_device_image_format_properties`] **can**  be used to check for supported properties of individual formats as normal.
- [`shader_zero_initialize_workgroup_memory`] specifies whether the implementation supports initializing a variable in Workgroup storage class.
- [`dynamic_rendering`] specifies that the implementation supports dynamic render pass instances using the [`cmd_begin_rendering`] command.
- [`shader_integer_dot_product`] specifies whether shader modules  **can**  declare the `DotProductInputAllKHR`, `DotProductInput4x8BitKHR`, `DotProductInput4x8BitPackedKHR` and `DotProductKHR` capabilities.
- [`maintenance4`] indicates that the implementation supports the following:  - The application  **may**  destroy a [`PipelineLayout`] object immediately after using it to create another object.  - `LocalSizeId` **can**  be used as an alternative to `LocalSize` to specify the local workgroup size with specialization constants.  - Images created with identical creation parameters will always have the same alignment requirements.  - The size memory requirement of a buffer or image is never greater than that of another buffer or image created with a greater or equal size.  - Push constants do not have to be initialized before they are dynamically accessed.  - The interface matching rules allow a larger output vector to match with a smaller input vector, with additional values being discarded. 
If the [`PhysicalDeviceVulkan13Features`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceVulkan13Features`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES`

# Related
- [`crate::vulkan1_3`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        