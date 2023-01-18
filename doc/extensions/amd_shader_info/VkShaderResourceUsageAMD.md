[VkShaderResourceUsageAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderResourceUsageAMD.html) - Resource usage information about a particular shader within a pipeline

# C Specifications
The [`ShaderResourceUsageAMD`] structure is defined as:
```c
// Provided by VK_AMD_shader_info
typedef struct VkShaderResourceUsageAMD {
    uint32_t    numUsedVgprs;
    uint32_t    numUsedSgprs;
    uint32_t    ldsSizePerLocalWorkGroup;
    size_t      ldsUsageSizeInBytes;
    size_t      scratchMemUsageInBytes;
} VkShaderResourceUsageAMD;
```

# Members
- [`num_used_vgprs`] is the number of vector instruction general-purpose registers used by this shader.
- [`num_used_sgprs`] is the number of scalar instruction general-purpose registers used by this shader.
- [`lds_size_per_local_work_group`] is the maximum local data store size per work group in bytes.
- [`lds_usage_size_in_bytes`] is the LDS usage size in bytes per work group by this shader.
- [`scratch_mem_usage_in_bytes`] is the scratch memory usage in bytes by this shader.

# Related
- [`VK_AMD_shader_info`]
- [`ShaderStatisticsInfoAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        