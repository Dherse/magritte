[VkPipelineColorBlendAdvancedStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) - Structure specifying parameters that affect advanced blend operations

# C Specifications
If the [`p_next`] chain of [`PipelineColorBlendStateCreateInfo`]
includes a [`PipelineColorBlendAdvancedStateCreateInfoEXT`] structure,
then that structure includes parameters that affect advanced blend
operations.The [`PipelineColorBlendAdvancedStateCreateInfoEXT`] structure is
defined as:
```c
// Provided by VK_EXT_blend_operation_advanced
typedef struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    VkStructureType      sType;
    const void*          pNext;
    VkBool32             srcPremultiplied;
    VkBool32             dstPremultiplied;
    VkBlendOverlapEXT    blendOverlap;
} VkPipelineColorBlendAdvancedStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`src_premultiplied`] specifies whether the source color of the blend operation is treated as premultiplied.
- [`dst_premultiplied`] specifies whether the destination color of the blend operation is treated as premultiplied.
- [`blend_overlap`] is a [`BlendOverlapEXT`] value specifying how the source and destination sample’s coverage is correlated.

# Description
If this structure is not present, [`src_premultiplied`] and
[`dst_premultiplied`] are both considered to be `VK_TRUE`, and
[`blend_overlap`] is considered to be
`VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
## Valid Usage
-    If the [non-premultiplied source color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedSrcColor) property is not supported, [`src_premultiplied`] **must**  be `VK_TRUE`
-    If the [non-premultiplied destination color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedDstColor) property is not supported, [`dst_premultiplied`] **must**  be `VK_TRUE`
-    If the [correlated overlap](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendCorrelatedOverlap) property is not supported, [`blend_overlap`] **must**  be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
-  [`blend_overlap`] **must**  be a valid [`BlendOverlapEXT`] value

# Related
- [`ext_blend_operation_advanced`]
- [`BlendOverlapEXT`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        