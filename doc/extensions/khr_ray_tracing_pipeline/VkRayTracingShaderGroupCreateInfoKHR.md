[VkRayTracingShaderGroupCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoKHR.html) - Structure specifying shaders in a shader group

# C Specifications
The [`RayTracingShaderGroupCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef struct VkRayTracingShaderGroupCreateInfoKHR {
    VkStructureType                   sType;
    const void*                       pNext;
    VkRayTracingShaderGroupTypeKHR    type;
    uint32_t                          generalShader;
    uint32_t                          closestHitShader;
    uint32_t                          anyHitShader;
    uint32_t                          intersectionShader;
    const void*                       pShaderGroupCaptureReplayHandle;
} VkRayTracingShaderGroupCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`type_`] is the type of hit group specified in this structure.
- [`general_shader`] is the index of the ray generation, miss, or callable shader from [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR`, and [`SHADER_UNUSED_KHR`] otherwise.
- [`closest_hit_shader`] is the optional index of the closest hit shader from [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`] otherwise.
- [`any_hit_shader`] is the optional index of the any-hit shader from [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` or `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`] otherwise.
- [`intersection_shader`] is the index of the intersection shader from [`RayTracingPipelineCreateInfoKHR::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR`, and [`SHADER_UNUSED_KHR`] otherwise.
- [`shader_group_capture_replay_handle`] is `NULL` or a pointer to replay information for this shader group. Ignored if [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`] is [`FALSE`].

# Description
## Valid Usage
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR` then [`general_shader`] **must**  be a valid index into [`RayTracingPipelineCreateInfoKHR::stages`] referring to a shader of `VK_SHADER_STAGE_RAYGEN_BIT_KHR`, `VK_SHADER_STAGE_MISS_BIT_KHR`, or `VK_SHADER_STAGE_CALLABLE_BIT_KHR`
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR` then [`closest_hit_shader`], [`any_hit_shader`], and [`intersection_shader`] **must**  be [`SHADER_UNUSED_KHR`]
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR` then [`intersection_shader`] **must**  be a valid index into [`RayTracingPipelineCreateInfoKHR::stages`] referring to a shader of `VK_SHADER_STAGE_INTERSECTION_BIT_KHR`
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR` then [`intersection_shader`] **must**  be [`SHADER_UNUSED_KHR`]
-  [`closest_hit_shader`] **must**  be either [`SHADER_UNUSED_KHR`] or a valid index into [`RayTracingPipelineCreateInfoKHR::stages`] referring to a shader of `VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR`
-  [`any_hit_shader`] **must**  be either [`SHADER_UNUSED_KHR`] or a valid index into [`RayTracingPipelineCreateInfoKHR::stages`] referring to a shader of `VK_SHADER_STAGE_ANY_HIT_BIT_KHR`
-    If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`FALSE`] then [`shader_group_capture_replay_handle`] **must**  not be provided if it has not been provided on a previous call to ray tracing pipeline creation
-    If [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is [`FALSE`] then the caller  **must**  guarantee that no ray tracing pipeline creation commands with [`shader_group_capture_replay_handle`] provided execute simultaneously with ray tracing pipeline creation commands without [`shader_group_capture_replay_handle`] provided

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`type_`] **must**  be a valid [`RayTracingShaderGroupTypeKHR`] value

# Related
- [`VK_KHR_ray_tracing_pipeline`]
- [`RayTracingPipelineCreateInfoKHR`]
- [`RayTracingShaderGroupTypeKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        