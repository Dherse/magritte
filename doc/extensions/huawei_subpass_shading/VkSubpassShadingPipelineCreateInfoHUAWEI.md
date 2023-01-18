[VkSubpassShadingPipelineCreateInfoHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html) - Structure specifying parameters of a newly created subpass shading pipeline

# C Specifications
A subpass shading pipeline is a compute pipeline which  **must**  be called only
in a subpass of a render pass with work dimensions specified by render area
size.
The subpass shading pipeline shader is a compute shader allowed to access
input attachments specified in the calling subpass.
To create a subpass shading pipeline, call [`create_compute_pipelines`]
with [`SubpassShadingPipelineCreateInfoHUAWEI`] in the [`p_next`] chain
of [`ComputePipelineCreateInfo`].The [`SubpassShadingPipelineCreateInfoHUAWEI`] structure is defined as:
```c
// Provided by VK_HUAWEI_subpass_shading
typedef struct VkSubpassShadingPipelineCreateInfoHUAWEI {
    VkStructureType    sType;
    void*              pNext;
    VkRenderPass       renderPass;
    uint32_t           subpass;
} VkSubpassShadingPipelineCreateInfoHUAWEI;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`render_pass`] is a handle to a render pass object describing the environment in which the pipeline will be used. The pipeline  **must**  only be used with a render pass instance compatible with the one provided. See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more information.
- [`subpass`] is the index of the subpass in the render pass where this pipeline will be used.

# Description
## Valid Usage
-  [`subpass`] **must**  be created with `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI` bind point

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI`

# Related
- [`VK_HUAWEI_subpass_shading`]
- [`RenderPass`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        