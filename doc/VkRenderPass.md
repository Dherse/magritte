[VkRenderPass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPass.html) - Opaque handle to a render pass object

# C Specifications
A render pass object represents a collection of attachments, subpasses, and
dependencies between the subpasses, and describes how the attachments are
used over the course of the subpasses.Render passes are represented by [`RenderPass`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkRenderPass)
```

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferInheritanceInfo`]
- [`FramebufferCreateInfo`]
- [`GraphicsPipelineCreateInfo`]
- [`RenderPassBeginInfo`]
- [`SubpassShadingPipelineCreateInfoHUAWEI`]
- [`create_render_pass`]
- [`create_render_pass2`]
- [`create_render_pass2_khr`]
- [`destroy_render_pass`]
- [`get_device_subpass_shading_max_workgroup_size_huawei`]
- [`get_render_area_granularity`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        