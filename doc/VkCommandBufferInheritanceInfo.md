[VkCommandBufferInheritanceInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceInfo.html) - Structure specifying command buffer inheritance information

# C Specifications
If the command buffer is a secondary command buffer, then the
[`CommandBufferInheritanceInfo`] structure defines any state that will
be inherited from the primary command buffer:
```c
// Provided by VK_VERSION_1_0
typedef struct VkCommandBufferInheritanceInfo {
    VkStructureType                  sType;
    const void*                      pNext;
    VkRenderPass                     renderPass;
    uint32_t                         subpass;
    VkFramebuffer                    framebuffer;
    VkBool32                         occlusionQueryEnable;
    VkQueryControlFlags              queryFlags;
    VkQueryPipelineStatisticFlags    pipelineStatistics;
} VkCommandBufferInheritanceInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`render_pass`] is a [`RenderPass`] object defining which render passes the [`CommandBuffer`] will be [compatible](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) with and  **can**  be executed within.
- [`subpass`] is the index of the subpass within the render pass instance that the [`CommandBuffer`] will be executed within.
- [`framebuffer`] **can**  refer to the [`Framebuffer`] object that the [`CommandBuffer`] will be rendering to if it is executed within a render pass instance. It  **can**  be [`crate::Handle::null`] if the framebuffer is not known.
- [`occlusion_query_enable`] specifies whether the command buffer  **can**  be executed while an occlusion query is active in the primary command buffer. If this is [`TRUE`], then this command buffer  **can**  be executed whether the primary command buffer has an occlusion query active or not. If this is [`FALSE`], then the primary command buffer  **must**  not have an occlusion query active.
- [`query_flags`] specifies the query flags that  **can**  be used by an active occlusion query in the primary command buffer when this secondary command buffer is executed. If this value includes the `VK_QUERY_CONTROL_PRECISE_BIT` bit, then the active query  **can**  return boolean results or actual sample counts. If this bit is not set, then the active query  **must**  not use the `VK_QUERY_CONTROL_PRECISE_BIT` bit.
- [`pipeline_statistics`] is a bitmask of [`QueryPipelineStatisticFlagBits`] specifying the set of pipeline statistics that  **can**  be counted by an active query in the primary command buffer when this secondary command buffer is executed. If this value includes a given bit, then this command buffer  **can**  be executed whether the primary command buffer has a pipeline statistics query active that includes this bit or not. If this value excludes a given bit, then the active pipeline statistics query  **must**  not be from a query pool that counts that statistic.

# Description
If the [`CommandBuffer`] will not be executed within a render pass
instance,
or if the render pass instance was begun with [`cmd_begin_rendering`],
[`render_pass`], [`subpass`], and [`framebuffer`] are ignored.
## Valid Usage
-    If the [inherited queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedQueries) feature is not enabled, [`occlusion_query_enable`] **must**  be [`FALSE`]
-    If the [inherited queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedQueries) feature is enabled, [`query_flags`] **must**  be a valid combination of [`QueryControlFlagBits`] values
-    If the [inherited queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedQueries) feature is not enabled, [`query_flags`] **must**  be `0`
-    If the [pipeline statistics queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineStatisticsQuery) feature is enabled, [`pipeline_statistics`] **must**  be a valid combination of [`QueryPipelineStatisticFlagBits`] values
-    If the [pipeline statistics queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineStatisticsQuery) feature is not enabled, [`pipeline_statistics`] **must**  be `0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`AttachmentSampleCountInfoAMD`], [`CommandBufferInheritanceConditionalRenderingInfoEXT`], [`CommandBufferInheritanceRenderPassTransformInfoQCOM`], [`CommandBufferInheritanceRenderingInfo`], [`CommandBufferInheritanceViewportScissorInfoNV`], or [`MultiviewPerViewAttributesInfoNVX`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-    Both of [`framebuffer`], and [`render_pass`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`CommandBufferBeginInfo`]
- [`Framebuffer`]
- [`QueryControlFlags`]
- [`QueryPipelineStatisticFlags`]
- [`RenderPass`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        