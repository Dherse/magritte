[VkRayTracingShaderGroupTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingShaderGroupTypeKHR.html) - Shader group types

# C Specifications
Possible values of `type` in [`RayTracingShaderGroupCreateInfoKHR`]
are:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef enum VkRayTracingShaderGroupTypeKHR {
    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR = 0,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR = 1,
    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR = 2,
  // Provided by VK_NV_ray_tracing
    VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV = VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR,
  // Provided by VK_NV_ray_tracing
    VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV = VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR,
  // Provided by VK_NV_ray_tracing
    VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV = VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR,
} VkRayTracingShaderGroupTypeKHR;
```
or the equivalent
```c
// Provided by VK_NV_ray_tracing
typedef VkRayTracingShaderGroupTypeKHR VkRayTracingShaderGroupTypeNV;
```

# Description
- [`GENERAL`] indicates a shader group with a single `VK_SHADER_STAGE_RAYGEN_BIT_KHR`, `VK_SHADER_STAGE_MISS_BIT_KHR`, or `VK_SHADER_STAGE_CALLABLE_BIT_KHR` shader in it.
- [`TRIANGLES_HIT_GROUP`] specifies a shader group that only hits triangles and  **must**  not contain an intersection shader, only closest hit and any-hit shaders.
- [`PROCEDURAL_HIT_GROUP`] specifies a shader group that only intersects with custom geometry and  **must**  contain an intersection shader and  **may**  contain closest hit and any-hit shaders.

# Related
- [`VK_KHR_ray_tracing_pipeline`]
- [`VK_NV_ray_tracing`]
- [`RayTracingShaderGroupCreateInfoKHR`]
- [`RayTracingShaderGroupCreateInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        