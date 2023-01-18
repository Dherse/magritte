[VkPipelineRasterizationProvokingVertexStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html) - Structure specifying provoking vertex mode used by a graphics pipeline

# C Specifications
For a given primitive topology, the pipeline’s provoking vertex mode
determines which vertex is the provoking vertex.
To specify the provoking vertex mode, include a
[`PipelineRasterizationProvokingVertexStateCreateInfoEXT`] structure in
the [`PipelineRasterizationStateCreateInfo`]::[`p_next`] chain when
creating the pipeline.The [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`] structure
is defined as:
```c
// Provided by VK_EXT_provoking_vertex
typedef struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
    VkStructureType             sType;
    const void*                 pNext;
    VkProvokingVertexModeEXT    provokingVertexMode;
} VkPipelineRasterizationProvokingVertexStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`provoking_vertex_mode`] is a [`ProvokingVertexModeEXT`] value selecting the provoking vertex mode.

# Description
If this struct is not provided when creating the pipeline, the pipeline will
use the `VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT` mode.If the
[provokingVertexModePerPipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-provokingVertexModePerPipeline)
limit is [`FALSE`], then all pipelines bound within a render pass
instance  **must**  have the same [`provoking_vertex_mode`].
## Valid Usage
-    If [`provoking_vertex_mode`] is `VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`, then the [provokingVertexLast](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-provokingVertexLast) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT`
-  [`provoking_vertex_mode`] **must**  be a valid [`ProvokingVertexModeEXT`] value

# Related
- [`VK_EXT_provoking_vertex`]
- [`ProvokingVertexModeEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        