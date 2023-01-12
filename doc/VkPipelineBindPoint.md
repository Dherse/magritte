[VkPipelineBindPoint](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineBindPoint.html) - Specify the bind point of a pipeline object to a command buffer

# C Specifications
Possible values of [`cmd_bind_pipeline`]`::pipelineBindPoint`,
specifying the bind point of a pipeline object, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkPipelineBindPoint {
    VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
    VK_PIPELINE_BIND_POINT_COMPUTE = 1,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR = 1000165000,
  // Provided by VK_HUAWEI_subpass_shading
    VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI = 1000369003,
  // Provided by VK_NV_ray_tracing
    VK_PIPELINE_BIND_POINT_RAY_TRACING_NV = VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR,
} VkPipelineBindPoint;
```

# Description
- [`VK_PIPELINE_BIND_POINT`] specifies binding as a compute pipeline.
- [`VK_PIPELINE_BIND_POINT`] specifies binding as a graphics pipeline.
- [`RAY_TRACING_KHR`] specifies binding as a ray tracing pipeline.
- [`SUBPASS_SHADING_HUAWEI`] specifies binding as a subpass shading pipeline.

# Related
- [`crate::vulkan1_0`]
- [`DescriptorUpdateTemplateCreateInfo`]
- [`GeneratedCommandsInfoNV`]
- [`GeneratedCommandsMemoryRequirementsInfoNV`]
- [`IndirectCommandsLayoutCreateInfoNV`]
- [`SubpassDescription`]
- [`SubpassDescription2`]
- [`cmd_bind_descriptor_sets`]
- [`cmd_bind_pipeline`]
- [`cmd_bind_pipeline_shader_group_nv`]
- [`cmd_push_descriptor_set_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        