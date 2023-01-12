[VkPipelineLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineLayout.html) - Opaque handle to a pipeline layout object

# C Specifications
Access to descriptor sets from a pipeline is accomplished through a
*pipeline layout*.
Zero or more descriptor set layouts and zero or more push constant ranges
are combined to form a pipeline layout object describing the complete set of
resources that  **can**  be accessed by a pipeline.
The pipeline layout represents a sequence of descriptor sets with each
having a specific layout.
This sequence of layouts is used to determine the interface between shader
stages and shader resources.
Each pipeline is created using a pipeline layout.Pipeline layout objects are represented by [`PipelineLayout`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipelineLayout)
```

# Related
- [`crate::vulkan1_0`]
- [`ComputePipelineCreateInfo`]
- [`DescriptorUpdateTemplateCreateInfo`]
- [`GraphicsPipelineCreateInfo`]
- [`IndirectCommandsLayoutTokenNV`]
- [`RayTracingPipelineCreateInfoKHR`]
- [`RayTracingPipelineCreateInfoNV`]
- [`cmd_bind_descriptor_sets`]
- [`cmd_push_constants`]
- [`cmd_push_descriptor_set_khr`]
- [`cmd_push_descriptor_set_with_template_khr`]
- [`create_pipeline_layout`]
- [`destroy_pipeline_layout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        