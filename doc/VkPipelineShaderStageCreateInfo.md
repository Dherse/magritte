[VkPipelineShaderStageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageCreateInfo.html) - Structure specifying parameters of a newly created pipeline shader stage

# C Specifications
The [`PipelineShaderStageCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineShaderStageCreateInfo {
    VkStructureType                     sType;
    const void*                         pNext;
    VkPipelineShaderStageCreateFlags    flags;
    VkShaderStageFlagBits               stage;
    VkShaderModule                      module;
    const char*                         pName;
    const VkSpecializationInfo*         pSpecializationInfo;
} VkPipelineShaderStageCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`PipelineShaderStageCreateFlagBits`] specifying how the pipeline shader stage will be generated.
- [`stage`] is a [`ShaderStageFlagBits`] value specifying a single pipeline stage.
- [`module`] is a [`ShaderModule`] object containing the shader for this stage.
- [`name`] is a pointer to a null-terminated UTF-8 string specifying the entry point name of the shader for this stage.
- [`specialization_info`] is a pointer to a [`SpecializationInfo`] structure, as described in [Specialization Constants](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-specialization-constants), or `NULL`.

# Description
## Valid Usage
-    If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not enabled, [`stage`] **must**  not be `VK_SHADER_STAGE_GEOMETRY_BIT`
-    If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) feature is not enabled, [`stage`] **must**  not be `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT` or `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`
-    If the [mesh shader](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-meshShader) feature is not enabled, [`stage`] **must**  not be `VK_SHADER_STAGE_MESH_BIT_NV`
-    If the [task shader](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-taskShader) feature is not enabled, [`stage`] **must**  not be `VK_SHADER_STAGE_TASK_BIT_NV`
-  [`stage`] **must**  not be `VK_SHADER_STAGE_ALL_GRAPHICS`, or `VK_SHADER_STAGE_ALL`
-  [`name`] **must**  be the name of an `OpEntryPoint` in [`module`] with an execution model that matches [`stage`]
-    If the identified entry point includes any variable in its interface that is declared with the `ClipDistance``BuiltIn` decoration, that variable  **must**  not have an array size greater than [`PhysicalDeviceLimits::max_clip_distances`]
-    If the identified entry point includes any variable in its interface that is declared with the `CullDistance``BuiltIn` decoration, that variable  **must**  not have an array size greater than [`PhysicalDeviceLimits::max_cull_distances`]
-    If the identified entry point includes any variables in its interface that are declared with the `ClipDistance` or `CullDistance``BuiltIn` decoration, those variables  **must**  not have array sizes which sum to more than [`PhysicalDeviceLimits::max_combined_clip_and_cull_distances`]
-    If the identified entry point includes any variable in its interface that is declared with the [`SampleMask`]`BuiltIn` decoration, that variable  **must**  not have an array size greater than [`PhysicalDeviceLimits::max_sample_mask_words`]
-    If [`stage`] is `VK_SHADER_STAGE_VERTEX_BIT`, the identified entry point  **must**  not include any input variable in its interface that is decorated with `CullDistance`
-    If [`stage`] is `VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT` or `VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`, and the identified entry point has an `OpExecutionMode` instruction specifying a patch size with `OutputVertices`, the patch size  **must**  be greater than `0` and less than or equal to [`PhysicalDeviceLimits::max_tessellation_patch_size`]
-    If [`stage`] is `VK_SHADER_STAGE_GEOMETRY_BIT`, the identified entry point  **must**  have an `OpExecutionMode` instruction specifying a maximum output vertex count that is greater than `0` and less than or equal to [`PhysicalDeviceLimits::max_geometry_output_vertices`]
-    If [`stage`] is `VK_SHADER_STAGE_GEOMETRY_BIT`, the identified entry point  **must**  have an `OpExecutionMode` instruction specifying an invocation count that is greater than `0` and less than or equal to [`PhysicalDeviceLimits::max_geometry_shader_invocations`]
-    If [`stage`] is a [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization), and the identified entry point writes to `Layer` for any primitive, it  **must**  write the same value to `Layer` for all vertices of a given primitive
-    If [`stage`] is a [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization), and the identified entry point writes to `ViewportIndex` for any primitive, it  **must**  write the same value to `ViewportIndex` for all vertices of a given primitive
-    If [`stage`] is `VK_SHADER_STAGE_FRAGMENT_BIT`, the identified entry point  **must**  not include any output variables in its interface decorated with `CullDistance`
-    If [`stage`] is `VK_SHADER_STAGE_FRAGMENT_BIT`, and the identified entry point writes to `FragDepth` in any execution path, it  **must**  write to `FragDepth` in all execution paths
-    If [`stage`] is `VK_SHADER_STAGE_FRAGMENT_BIT`, and the identified entry point writes to `FragStencilRefEXT` in any execution path, it  **must**  write to `FragStencilRefEXT` in all execution paths
-    If [`stage`] is `VK_SHADER_STAGE_MESH_BIT_NV`, the identified entry point  **must**  have an `OpExecutionMode` instruction specifying a maximum output vertex count, `OutputVertices`, that is greater than `0` and less than or equal to [`PhysicalDeviceMeshShaderPropertiesNV::max_mesh_output_vertices`]
-    If [`stage`] is `VK_SHADER_STAGE_MESH_BIT_NV`, the identified entry point  **must**  have an `OpExecutionMode` instruction specifying a maximum output primitive count, `OutputPrimitivesNV`, that is greater than `0` and less than or equal to [`PhysicalDeviceMeshShaderPropertiesNV::max_mesh_output_primitives`]
-    If [`flags`] has the `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT` flag set, the [`subgroupSizeControl`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-subgroupSizeControl) feature  **must**  be enabled
-    If [`flags`] has the `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag set, the [`computeFullSubgroups`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-computeFullSubgroups) feature  **must**  be enabled
-    If a [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in the [`p_next`] chain, [`flags`] **must**  not have the `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT` flag set
-    If a [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in the [`p_next`] chain, the [`subgroupSizeControl`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-subgroupSizeControl) feature  **must**  be enabled, and [`stage`] **must**  be a valid bit specified in [`requiredSubgroupSizeStages`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-requiredSubgroupSizeStages)
-    If a [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in the [`p_next`] chain and [`stage`] is `VK_SHADER_STAGE_COMPUTE_BIT`, the local workgroup size of the shader  **must**  be less than or equal to the product of [`PipelineShaderStageRequiredSubgroupSizeCreateInfo::required_subgroup_size`] and [`maxComputeWorkgroupSubgroups`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxComputeWorkgroupSubgroups)
-    If a [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in the [`p_next`] chain, and [`flags`] has the `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag set, the local workgroup size in the X dimension of the pipeline  **must**  be a multiple of [`PipelineShaderStageRequiredSubgroupSizeCreateInfo::required_subgroup_size`]
-    If [`flags`] has both the `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` and `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT` flags set, the local workgroup size in the X dimension of the pipeline  **must**  be a multiple of [`maxSubgroupSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxSubgroupSize)
-    If [`flags`] has the `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag set and [`flags`] does not have the `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT` flag set and no [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is included in the [`p_next`] chain, the local workgroup size in the X dimension of the pipeline  **must**  be a multiple of [`subgroupSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-subgroup-size)
-    The SPIR-V code that was used to create [`module`] **must**  be valid as described by the [Khronos SPIR-V Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirv-spec) after applying the specializations provided in [`specialization_info`], if any, and then converting all specialization constants into fixed constants

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be a valid combination of [`PipelineShaderStageCreateFlagBits`] values
-  [`stage`] **must**  be a valid [`ShaderStageFlagBits`] value
-  [`module`] **must**  be a valid [`ShaderModule`] handle
-  [`name`] **must**  be a null-terminated UTF-8 string
-    If [`specialization_info`] is not `NULL`, [`specialization_info`] **must**  be a valid pointer to a valid [`SpecializationInfo`] structure

# Related
- [`crate::vulkan1_0`]
- [`ComputePipelineCreateInfo`]
- [`GraphicsPipelineCreateInfo`]
- [`GraphicsShaderGroupCreateInfoNV`]
- [VkPipelineShaderStageCreateFlags]()
- [`RayTracingPipelineCreateInfoKHR`]
- [`RayTracingPipelineCreateInfoNV`]
- [`ShaderModule`]
- [`ShaderStageFlagBits`]
- [`SpecializationInfo`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        