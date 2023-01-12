[VkPipelineShaderStageRequiredSubgroupSizeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html) - Structure specifying the required subgroup size of a newly created pipeline shader stage

# C Specifications
The [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is
defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           requiredSubgroupSize;
} VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_subgroup_size_control
typedef VkPipelineShaderStageRequiredSubgroupSizeCreateInfo VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`required_subgroup_size`] is an unsigned integer value specifying the required subgroup size for the newly created pipeline shader stage.

# Description
If a [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is
included in the [`p_next`] chain of [`PipelineShaderStageCreateInfo`],
it specifies that the pipeline shader stage being compiled has a required
subgroup size.
## Valid Usage
-  [`required_subgroup_size`] **must**  be a power-of-two integer
-  [`required_subgroup_size`] **must**  be greater or equal to [minSubgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minSubgroupSize)
-  [`required_subgroup_size`] **must**  be less than or equal to [maxSubgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxSubgroupSize)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO`

# Related
- [`ext_subgroup_size_control`]
- [`crate::vulkan1_3`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        