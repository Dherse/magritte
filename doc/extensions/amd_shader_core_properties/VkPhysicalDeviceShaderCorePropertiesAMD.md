[VkPhysicalDeviceShaderCorePropertiesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html) - Structure describing shader core properties that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceShaderCorePropertiesAMD`] structure is defined as:
```c
// Provided by VK_AMD_shader_core_properties
typedef struct VkPhysicalDeviceShaderCorePropertiesAMD {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           shaderEngineCount;
    uint32_t           shaderArraysPerEngineCount;
    uint32_t           computeUnitsPerShaderArray;
    uint32_t           simdPerComputeUnit;
    uint32_t           wavefrontsPerSimd;
    uint32_t           wavefrontSize;
    uint32_t           sgprsPerSimd;
    uint32_t           minSgprAllocation;
    uint32_t           maxSgprAllocation;
    uint32_t           sgprAllocationGranularity;
    uint32_t           vgprsPerSimd;
    uint32_t           minVgprAllocation;
    uint32_t           maxVgprAllocation;
    uint32_t           vgprAllocationGranularity;
} VkPhysicalDeviceShaderCorePropertiesAMD;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`shader_engine_count`] is an unsigned integer value indicating the number of shader engines found inside the shader core of the physical device.
- [`shader_arrays_per_engine_count`] is an unsigned integer value indicating the number of shader arrays inside a shader engine. Each shader array has its own scan converter, set of compute units, and a render back end (color and depth attachments). Shader arrays within a shader engine share shader processor input (wave launcher) and shader export (export buffer) units. Currently, a shader engine can have one or two shader arrays.
- [`compute_units_per_shader_array`] is an unsigned integer value indicating the physical number of compute units within a shader array. The active number of compute units in a shader array  **may**  be lower. A compute unit houses a set of SIMDs along with a sequencer module and a local data store.
- [`simd_per_compute_unit`] is an unsigned integer value indicating the number of SIMDs inside a compute unit. Each SIMD processes a single instruction at a time.
- [`wavefront_size`] is an unsigned integer value indicating the maximum size of a subgroup.
- [`sgprs_per_simd`] is an unsigned integer value indicating the number of physical Scalar General Purpose Registers (SGPRs) per SIMD.
- [`min_sgpr_allocation`] is an unsigned integer value indicating the minimum number of SGPRs allocated for a wave.
- [`max_sgpr_allocation`] is an unsigned integer value indicating the maximum number of SGPRs allocated for a wave.
- [`sgpr_allocation_granularity`] is an unsigned integer value indicating the granularity of SGPR allocation for a wave.
- [`vgprs_per_simd`] is an unsigned integer value indicating the number of physical Vector General Purpose Registers (VGPRs) per SIMD.
- [`min_vgpr_allocation`] is an unsigned integer value indicating the minimum number of VGPRs allocated for a wave.
- [`max_vgpr_allocation`] is an unsigned integer value indicating the maximum number of VGPRs allocated for a wave.
- [`vgpr_allocation_granularity`] is an unsigned integer value indicating the granularity of VGPR allocation for a wave.

# Description
If the [`PhysicalDeviceShaderCorePropertiesAMD`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`

# Related
- [`amd_shader_core_properties`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        