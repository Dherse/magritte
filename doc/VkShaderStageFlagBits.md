[VkShaderStageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderStageFlagBits.html) - Bitmask specifying a pipeline stage

# C Specifications
Bits which  **can**  be set by commands and structures, specifying one or more
shader stages, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkShaderStageFlagBits {
    VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,
    VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
    VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
    VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
    VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
    VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,
    VK_SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
    VK_SHADER_STAGE_ALL = 0x7FFFFFFF,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_SHADER_STAGE_RAYGEN_BIT_KHR = 0x00000100,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_SHADER_STAGE_ANY_HIT_BIT_KHR = 0x00000200,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR = 0x00000400,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_SHADER_STAGE_MISS_BIT_KHR = 0x00000800,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_SHADER_STAGE_INTERSECTION_BIT_KHR = 0x00001000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_SHADER_STAGE_CALLABLE_BIT_KHR = 0x00002000,
  // Provided by VK_NV_mesh_shader
    VK_SHADER_STAGE_TASK_BIT_NV = 0x00000040,
  // Provided by VK_NV_mesh_shader
    VK_SHADER_STAGE_MESH_BIT_NV = 0x00000080,
  // Provided by VK_HUAWEI_subpass_shading
    VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI = 0x00004000,
  // Provided by VK_NV_ray_tracing
    VK_SHADER_STAGE_RAYGEN_BIT_NV = VK_SHADER_STAGE_RAYGEN_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_SHADER_STAGE_ANY_HIT_BIT_NV = VK_SHADER_STAGE_ANY_HIT_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV = VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_SHADER_STAGE_MISS_BIT_NV = VK_SHADER_STAGE_MISS_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_SHADER_STAGE_INTERSECTION_BIT_NV = VK_SHADER_STAGE_INTERSECTION_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_SHADER_STAGE_CALLABLE_BIT_NV = VK_SHADER_STAGE_CALLABLE_BIT_KHR,
} VkShaderStageFlagBits;
```

# Description
- [`VK_SHADER_STAGE_FLAG_BITS`] specifies the vertex stage.
- [`VK_SHADER_STAGE_FLAG_BITS`] specifies the tessellation control stage.
- [`VK_SHADER_STAGE_FLAG_BITS`] specifies the tessellation evaluation stage.
- [`VK_SHADER_STAGE_FLAG_BITS`] specifies the geometry stage.
- [`VK_SHADER_STAGE_FLAG_BITS`] specifies the fragment stage.
- [`VK_SHADER_STAGE_FLAG_BITS`] specifies the compute stage.
- [`VK_SHADER_STAGE_FLAG_BITS`] is a combination of bits used as shorthand to specify all graphics stages defined above (excluding the compute stage).
- [`VK_SHADER_STAGE_FLAG_BITS`] is a combination of bits used as shorthand to specify all shader stages supported by the device, including all additional stages which are introduced by extensions.
- [`TASK_NV`] specifies the task stage.
- [`MESH_NV`] specifies the mesh stage.
- [`RAYGEN_KHR`] specifies the ray generation stage.
- [`ANY_HIT_KHR`] specifies the any-hit stage.
- [`CLOSEST_HIT_KHR`] specifies the closest hit stage.
- [`MISS_KHR`] specifies the miss stage.
- [`INTERSECTION_KHR`] specifies the intersection stage.
- [`CALLABLE_KHR`] specifies the callable stage.

# Related
- [`crate::vulkan1_0`]
- [`PipelineShaderStageCreateInfo`]
- [VkShaderStageFlags]()
- [`get_shader_info_amd`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        