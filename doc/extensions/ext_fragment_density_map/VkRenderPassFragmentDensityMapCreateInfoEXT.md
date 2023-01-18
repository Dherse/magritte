[VkRenderPassFragmentDensityMapCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) - Structure containing fragment density map attachment for render pass

# C Specifications
If the [`RenderPassCreateInfo`]::[`p_next`] chain includes a
[`RenderPassFragmentDensityMapCreateInfoEXT`] structure, then that
structure includes a fragment density map attachment for the render pass.The [`RenderPassFragmentDensityMapCreateInfoEXT`] structure is defined
as:
```c
// Provided by VK_EXT_fragment_density_map
typedef struct VkRenderPassFragmentDensityMapCreateInfoEXT {
    VkStructureType          sType;
    const void*              pNext;
    VkAttachmentReference    fragmentDensityMapAttachment;
} VkRenderPassFragmentDensityMapCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`fragment_density_map_attachment`] is the fragment density map to use for the render pass.

# Description
The fragment density map is read at an implementation-dependent time with
the following constraints determined by the attachment’s image view
`flags`:
- `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` specifies that the fragment density map will be read by the device during `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
- `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT` specifies that the fragment density map will be read by the host during [`end_command_buffer`] of the primary command buffer that the render pass is recorded into
- Otherwise the fragment density map will be read by the host during [`cmd_begin_render_pass`]
The fragment density map  **may**  additionally be read by the device during
`VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT` for any mode.If this structure is not present, it is as if
[`fragment_density_map_attachment`] was given as [`ATTACHMENT_UNUSED`].
## Valid Usage
-    If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`], [`fragment_density_map_attachment`] **must**  not be an element of [`SubpassDescription::input_attachments`], [`SubpassDescription::color_attachments`], [`SubpassDescription::resolve_attachments`], [`SubpassDescription::depth_stencil_attachment`], or [`SubpassDescription::preserve_attachments`] for any subpass
-    If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`], `layout` **must**  be equal to `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`, or `VK_IMAGE_LAYOUT_GENERAL`
-    If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`], [`fragment_density_map_attachment`] **must**  reference an attachment with a `loadOp` equal to `VK_ATTACHMENT_LOAD_OP_LOAD` or `VK_ATTACHMENT_LOAD_OP_DONT_CARE`
-    If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`], [`fragment_density_map_attachment`] **must**  reference an attachment with a `storeOp` equal to `VK_ATTACHMENT_STORE_OP_DONT_CARE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`
-  [`fragment_density_map_attachment`] **must**  be a valid [`AttachmentReference`] structure

# Related
- [`VK_EXT_fragment_density_map`]
- [`AttachmentReference`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        