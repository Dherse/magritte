[VkMultiviewPerViewAttributesInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html) - Structure specifying the multiview per-attribute properties

# C Specifications
The [`MultiviewPerViewAttributesInfoNVX`] structure is defined as:
```c
// Provided by VK_KHR_dynamic_rendering with VK_NVX_multiview_per_view_attributes
typedef struct VkMultiviewPerViewAttributesInfoNVX {
    VkStructureType    sType;
    const void*        pNext;
    VkBool32           perViewAttributes;
    VkBool32           perViewAttributesPositionXOnly;
} VkMultiviewPerViewAttributesInfoNVX;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`per_view_attributes`] specifies that shaders compiled for this pipeline write the attributes for all views in a single invocation of each vertex processing stage. All pipelines executed within a render pass instance that includes this bit  **must**  write per-view attributes to the `*PerViewNV[]` shader outputs, in addition to the non-per-view (e.g. `Position`) outputs.
- [`per_view_attributes_position_x_only`] specifies that shaders compiled for this pipeline use per-view positions which only differ in value in the x component. Per-view viewport mask  **can**  also be used.

# Description
When dynamic render pass instances are being used, instead of specifying
`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` or
`VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX` in the subpass
description flags, the per-attibute properties of the render pass instance
 **must**  be specified by the [`MultiviewPerViewAttributesInfoNVX`]
structure Include the [`MultiviewPerViewAttributesInfoNVX`] structure in
the [`p_next`] chain of [`GraphicsPipelineCreateInfo`] when creating a
graphics pipeline for dynamic rendering, [`RenderingInfo`] when starting
a dynamic render pass instance, and [`CommandBufferInheritanceInfo`]
when specifying the dynamic render pass instance parameters for secondary
command buffers.
## Valid Usage
-    If [`per_view_attributes_position_x_only`] is [`TRUE`] then [`per_view_attributes`] **must**  also be [`TRUE`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX`

# Related
- [`VK_KHR_dynamic_rendering`]
- [`VK_NVX_multiview_per_view_attributes`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        