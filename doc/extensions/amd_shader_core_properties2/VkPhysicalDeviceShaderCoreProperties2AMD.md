[VkPhysicalDeviceShaderCoreProperties2AMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html) - Structure describing shader core properties that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceShaderCoreProperties2AMD`] structure is defined as:
```c
// Provided by VK_AMD_shader_core_properties2
typedef struct VkPhysicalDeviceShaderCoreProperties2AMD {
    VkStructureType                   sType;
    void*                             pNext;
    VkShaderCorePropertiesFlagsAMD    shaderCoreFeatures;
    uint32_t                          activeComputeUnitCount;
} VkPhysicalDeviceShaderCoreProperties2AMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`shader_core_features`] is a bitmask of [`ShaderCorePropertiesFlagBitsAMD`] indicating the set of features supported by the shader core.
- [`active_compute_unit_count`] is an unsigned integer value indicating the number of compute units that have been enabled.

# Description
If the [`PhysicalDeviceShaderCoreProperties2AMD`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`

# Related
- [`VK_AMD_shader_core_properties2`]
- [`ShaderCorePropertiesFlagsAMD`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        