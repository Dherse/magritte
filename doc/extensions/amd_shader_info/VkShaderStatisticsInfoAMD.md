[VkShaderStatisticsInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStatisticsInfoAMD.html) - Statistical information about a particular shader within a pipeline

# C Specifications
The [`ShaderStatisticsInfoAMD`] structure is defined as:
```c
// Provided by VK_AMD_shader_info
typedef struct VkShaderStatisticsInfoAMD {
    VkShaderStageFlags          shaderStageMask;
    VkShaderResourceUsageAMD    resourceUsage;
    uint32_t                    numPhysicalVgprs;
    uint32_t                    numPhysicalSgprs;
    uint32_t                    numAvailableVgprs;
    uint32_t                    numAvailableSgprs;
    uint32_t                    computeWorkGroupSize[3];
} VkShaderStatisticsInfoAMD;
```

# Members
- [`shader_stage_mask`] are the combination of logical shader stages contained within this shader.
- [`resource_usage`] is a [`ShaderResourceUsageAMD`] structure describing internal physical device resources used by this shader.
- [`num_physical_vgprs`] is the maximum number of vector instruction general-purpose registers (VGPRs) available to the physical device.
- [`num_physical_sgprs`] is the maximum number of scalar instruction general-purpose registers (SGPRs) available to the physical device.
- [`num_available_vgprs`] is the maximum limit of VGPRs made available to the shader compiler.
- [`num_available_sgprs`] is the maximum limit of SGPRs made available to the shader compiler.
- [`compute_work_group_size`] is the local workgroup size of this shader in { X, Y, Z } dimensions.

# Description
Some implementations may merge multiple logical shader stages together in a
single shader.
In such cases, [`shader_stage_mask`] will contain a bitmask of all of the
stages that are active within that shader.
Consequently, if specifying those stages as input to
[`get_shader_info_amd`], the same output information  **may**  be returned for
all such shader stage queries.The number of available VGPRs and SGPRs ([`num_available_vgprs`] and
[`num_available_sgprs`] respectively) are the shader-addressable subset of
physical registers that is given as a limit to the compiler for register
assignment.
These values  **may**  further be limited by implementations due to performance
optimizations where register pressure is a bottleneck.

# Related
- [`amd_shader_info`]
- [`ShaderResourceUsageAMD`]
- [VkShaderStageFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        