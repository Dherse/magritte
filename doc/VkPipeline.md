[VkPipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipeline.html) - Opaque handle to a pipeline object

# C Specifications
Compute,
ray tracing,
and graphics pipelines are each represented by [`Pipeline`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipeline)
```

# Related
- [`crate::vulkan1_0`]
- [`ComputePipelineCreateInfo`]
- [`GeneratedCommandsInfoNV`]
- [`GeneratedCommandsMemoryRequirementsInfoNV`]
- [`GraphicsPipelineCreateInfo`]
- [`GraphicsPipelineShaderGroupsCreateInfoNV`]
- [`PipelineExecutableInfoKHR`]
- [`PipelineInfoKHR`]
- [`PipelineLibraryCreateInfoKHR`]
- [`RayTracingPipelineCreateInfoKHR`]
- [`RayTracingPipelineCreateInfoNV`]
- [`cmd_bind_pipeline`]
- [`cmd_bind_pipeline_shader_group_nv`]
- [`compile_deferred_nv`]
- [`create_compute_pipelines`]
- [`create_graphics_pipelines`]
- [`create_ray_tracing_pipelines_khr`]
- [`create_ray_tracing_pipelines_nv`]
- [`destroy_pipeline`]
- [`get_ray_tracing_capture_replay_shader_group_handles_khr`]
- [`get_ray_tracing_shader_group_handles_khr`]
- [`get_ray_tracing_shader_group_handles_nv`]
- [`get_ray_tracing_shader_group_stack_size_khr`]
- [`get_shader_info_amd`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        