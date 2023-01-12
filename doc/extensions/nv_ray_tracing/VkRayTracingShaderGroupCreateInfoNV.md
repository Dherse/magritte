[VkRayTracingShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupCreateInfoNV.html) - Structure specifying shaders in a shader group

# C Specifications
The [`RayTracingShaderGroupCreateInfoNV`] structure is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkRayTracingShaderGroupCreateInfoNV {
    VkStructureType                   sType;
    const void*                       pNext;
    VkRayTracingShaderGroupTypeKHR    type;
    uint32_t                          generalShader;
    uint32_t                          closestHitShader;
    uint32_t                          anyHitShader;
    uint32_t                          intersectionShader;
} VkRayTracingShaderGroupCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`type_`] is the type of hit group specified in this structure.
- [`general_shader`] is the index of the ray generation, miss, or callable shader from [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV`, and `VK_SHADER_UNUSED_NV` otherwise.
- [`closest_hit_shader`] is the optional index of the closest hit shader from [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and `VK_SHADER_UNUSED_NV` otherwise.
- [`any_hit_shader`] is the optional index of the any-hit shader from [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` or `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and `VK_SHADER_UNUSED_NV` otherwise.
- [`intersection_shader`] is the index of the intersection shader from [`RayTracingPipelineCreateInfoNV::stages`] in the group if the shader group has [`type_`] of `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV`, and `VK_SHADER_UNUSED_NV` otherwise.

# Description
## Valid Usage
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV` then [`general_shader`] **must**  be a valid index into [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of `VK_SHADER_STAGE_RAYGEN_BIT_NV`, `VK_SHADER_STAGE_MISS_BIT_NV`, or `VK_SHADER_STAGE_CALLABLE_BIT_NV`
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV` then [`closest_hit_shader`], [`any_hit_shader`], and [`intersection_shader`] **must**  be `VK_SHADER_UNUSED_NV`
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV` then [`intersection_shader`] **must**  be a valid index into [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of `VK_SHADER_STAGE_INTERSECTION_BIT_NV`
-    If [`type_`] is `VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV` then [`intersection_shader`] **must**  be `VK_SHADER_UNUSED_NV`
-  [`closest_hit_shader`] **must**  be either `VK_SHADER_UNUSED_NV` or a valid index into [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of `VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV`
-  [`any_hit_shader`] **must**  be either `VK_SHADER_UNUSED_NV` or a valid index into [`RayTracingPipelineCreateInfoNV::stages`] referring to a shader of `VK_SHADER_STAGE_ANY_HIT_BIT_NV`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`type_`] **must**  be a valid [`RayTracingShaderGroupTypeKHR`] value

# Related
- [`nv_ray_tracing`]
- [`RayTracingPipelineCreateInfoNV`]
- [`RayTracingShaderGroupTypeKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        