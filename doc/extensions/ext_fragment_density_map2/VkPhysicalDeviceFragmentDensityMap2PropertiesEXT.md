[VkPhysicalDeviceFragmentDensityMap2PropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html) - Structure describing additional fragment density map properties that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_fragment_density_map2
typedef struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           subsampledLoads;
    VkBool32           subsampledCoarseReconstructionEarlyAccess;
    uint32_t           maxSubsampledArrayLayers;
    uint32_t           maxDescriptorSetSubsampledSamplers;
} VkPhysicalDeviceFragmentDensityMap2PropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`subsampled_loads`] specifies if performing image data read with load operations on subsampled attachments will be resampled to the fragment density of the render pass
- [`subsampled_coarse_reconstruction_early_access`] specifies if performing image data read with samplers created with `flags` containing `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT` in fragment shader will trigger additional reads during `VK_PIPELINE_STAGE_VERTEX_SHADER_BIT`
- [`max_subsampled_array_layers`] is the maximum number of [`ImageView`] array layers for usages supporting subsampled samplers
- [`max_descriptor_set_subsampled_samplers`] is the maximum number of subsampled samplers that  **can**  be included in a [`PipelineLayout`]

# Description
If the [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT`

# Related
- [`ext_fragment_density_map2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        