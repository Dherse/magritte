[VkPipelineRasterizationDepthClipStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html) - Structure specifying depth clipping state

# C Specifications
If the [`p_next`] chain of [`PipelineRasterizationStateCreateInfo`]
includes a [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
structure, then that structure controls whether depth clipping is enabled or
disabled.The [`PipelineRasterizationDepthClipStateCreateInfoEXT`] structure is
defined as:
```c
// Provided by VK_EXT_depth_clip_enable
typedef struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
    VkStructureType                                        sType;
    const void*                                            pNext;
    VkPipelineRasterizationDepthClipStateCreateFlagsEXT    flags;
    VkBool32                                               depthClipEnable;
} VkPipelineRasterizationDepthClipStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`depth_clip_enable`] controls whether depth clipping is enabled as described in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping).

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`
-  [`flags`] **must**  be `0`

# Related
- [`ext_depth_clip_enable`]
- [`Bool32`]
- [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        