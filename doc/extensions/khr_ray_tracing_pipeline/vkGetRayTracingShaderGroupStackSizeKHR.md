[vkGetRayTracingShaderGroupStackSizeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html) - Query ray tracing pipeline shader group shader stack size

# C Specifications
To query the pipeline stack size of shaders in a shader group in the ray
tracing pipeline, call:
```c
// Provided by VK_KHR_ray_tracing_pipeline
VkDeviceSize vkGetRayTracingShaderGroupStackSizeKHR(
    VkDevice                                    device,
    VkPipeline                                  pipeline,
    uint32_t                                    group,
    VkShaderGroupShaderKHR                      groupShader);
```

# Parameters
- [`device`] is the logical device containing the ray tracing pipeline.
- [`pipeline`] is the ray tracing pipeline object containing the shaders groups.
- [`group`] is the index of the shader group to query.
- [`group_shader`] is the type of shader from the group to query.

# Description
The return value is the ray tracing pipeline stack size in bytes for the
specified shader as called from the specified shader group.
## Valid Usage
-  [`pipeline`] **must**  be a ray tracing pipeline
-    The value of [`group`] must be less than the number of shader groups in [`pipeline`]
-    The shader identified by [`group_shader`] in [`group`] **must**  not be [`SHADER_UNUSED_KHR`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`group_shader`] **must**  be a valid [`ShaderGroupShaderKHR`] value
-  [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`VK_KHR_ray_tracing_pipeline`]
- [`Device`]
- [`Pipeline`]
- [`ShaderGroupShaderKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        