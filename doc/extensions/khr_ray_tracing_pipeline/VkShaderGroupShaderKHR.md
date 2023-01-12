[VkShaderGroupShaderKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderGroupShaderKHR.html) - Shader group shaders

# C Specifications
Possible values of `groupShader` in
[`get_ray_tracing_shader_group_stack_size_khr`] are:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef enum VkShaderGroupShaderKHR {
    VK_SHADER_GROUP_SHADER_GENERAL_KHR = 0,
    VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR = 1,
    VK_SHADER_GROUP_SHADER_ANY_HIT_KHR = 2,
    VK_SHADER_GROUP_SHADER_INTERSECTION_KHR = 3,
} VkShaderGroupShaderKHR;
```

# Description
- [`VK_SHADER_GROUP_SHADER_KHR`] uses the shader specified in the group with [`RayTracingShaderGroupCreateInfoKHR::general_shader`]
- [`VK_SHADER_GROUP_SHADER_KHR`] uses the shader specified in the group with [`RayTracingShaderGroupCreateInfoKHR::closest_hit_shader`]
- [`VK_SHADER_GROUP_SHADER_KHR`] uses the shader specified in the group with [`RayTracingShaderGroupCreateInfoKHR::any_hit_shader`]
- [`VK_SHADER_GROUP_SHADER_KHR`] uses the shader specified in the group with [`RayTracingShaderGroupCreateInfoKHR::intersection_shader`]

# Related
- [`khr_ray_tracing_pipeline`]
- [`get_ray_tracing_shader_group_stack_size_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        