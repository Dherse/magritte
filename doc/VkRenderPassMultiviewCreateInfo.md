[VkRenderPassMultiviewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassMultiviewCreateInfo.html) - Structure containing multiview information for all subpasses

# C Specifications
If the [`RenderPassCreateInfo`]::[`p_next`] chain includes a
[`RenderPassMultiviewCreateInfo`] structure, then that structure
includes an array of view masks, view offsets, and correlation masks for the
render pass.The [`RenderPassMultiviewCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkRenderPassMultiviewCreateInfo {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           subpassCount;
    const uint32_t*    pViewMasks;
    uint32_t           dependencyCount;
    const int32_t*     pViewOffsets;
    uint32_t           correlationMaskCount;
    const uint32_t*    pCorrelationMasks;
} VkRenderPassMultiviewCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_multiview
typedef VkRenderPassMultiviewCreateInfo VkRenderPassMultiviewCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`subpass_count`] is zero or the number of subpasses in the render pass.
- [`view_masks`] is a pointer to an array of [`subpass_count`] view masks, where each mask is a bitfield of view indices describing which views rendering is broadcast to in each subpass, when multiview is enabled. If [`subpass_count`] is zero, each view mask is treated as zero.
- [`dependency_count`] is zero or the number of dependencies in the render pass.
- [`view_offsets`] is a pointer to an array of [`dependency_count`] view offsets, one for each dependency. If [`dependency_count`] is zero, each dependency’s view offset is treated as zero. Each view offset controls which views in the source subpass the views in the destination subpass depend on.
- [`correlation_mask_count`] is zero or the number of correlation masks.
- [`correlation_masks`] is a pointer to an array of [`correlation_mask_count`] view masks indicating sets of views that  **may**  be more efficient to render concurrently.

# Description
When a subpass uses a non-zero view mask, *multiview* functionality is
considered to be enabled.
Multiview is all-or-nothing for a render pass - that is, either all
subpasses  **must**  have a non-zero view mask (though some subpasses  **may**  have
only one view) or all  **must**  be zero.
Multiview causes all drawing and clear commands in the subpass to behave as
if they were broadcast to each view, where a view is represented by one
layer of the framebuffer attachments.
All draws and clears are broadcast to each *view index* whose bit is set in
the view mask.
The view index is provided in the `ViewIndex` shader input variable, and
color, depth/stencil, and input attachments all read/write the layer of the
framebuffer corresponding to the view index.If the view mask is zero for all subpasses, multiview is considered to be
disabled and all drawing commands execute normally, without this additional
broadcasting.Some implementations  **may**  not support multiview in conjunction with
[geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview-gs) or
[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview-tess).When multiview is enabled, the `VK_DEPENDENCY_VIEW_LOCAL_BIT` bit in a
dependency  **can**  be used to express a view-local dependency, meaning that
each view in the destination subpass depends on a single view in the source
subpass.
Unlike pipeline barriers, a subpass dependency  **can**  potentially have a
different view mask in the source subpass and the destination subpass.
If the dependency is view-local, then each view (dstView) in the
destination subpass depends on the view dstView + 
[`view_offsets`][dependency] in the source subpass.
If there is not such a view in the source subpass, then this dependency does
not affect that view in the destination subpass.
If the dependency is not view-local, then all views in the destination
subpass depend on all views in the source subpass, and the view offset is
ignored.
A non-zero view offset is not allowed in a self-dependency.The elements of [`correlation_masks`] are a set of masks of views
indicating that views in the same mask  **may**  exhibit spatial coherency
between the views, making it more efficient to render them concurrently.
Correlation masks  **must**  not have a functional effect on the results of the
multiview rendering.When multiview is enabled, at the beginning of each subpass all non-render
pass state is undefined.
In particular, each time [`cmd_begin_render_pass`] or
[`cmd_next_subpass`] is called the graphics pipeline  **must**  be bound, any
relevant descriptor sets or vertex/index buffers  **must**  be bound, and any
relevant dynamic state or push constants  **must**  be set before they are used.A multiview subpass  **can**  declare that its shaders will write per-view
attributes for all views in a single invocation, by setting the
`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit in the subpass
description.
The only supported per-view attributes are position and viewport mask, and
per-view position and viewport masks are written to output array variables
decorated with `PositionPerViewNV` and `ViewportMaskPerViewNV`,
respectively.
If `[`nv_viewport_array2`]` is not supported and enabled,
`ViewportMaskPerViewNV` **must**  not be used.
Values written to elements of `PositionPerViewNV` and
`ViewportMaskPerViewNV` **must**  not depend on the `ViewIndex`.
The shader  **must**  also write to an output variable decorated with
`Position`, and the value written to `Position` **must**  equal the value
written to `PositionPerViewNV`[`ViewIndex`].
Similarly, if `ViewportMaskPerViewNV` is written to then the shader  **must** 
also write to an output variable decorated with `ViewportMaskNV`, and the
value written to `ViewportMaskNV` **must**  equal the value written to
`ViewportMaskPerViewNV`[`ViewIndex`].
Implementations will either use values taken from `Position` and
`ViewportMaskNV` and invoke the shader once for each view, or will use
values taken from `PositionPerViewNV` and `ViewportMaskPerViewNV` and
invoke the shader fewer times.
The values written to `Position` and `ViewportMaskNV` **must**  not depend
on the values written to `PositionPerViewNV` and
`ViewportMaskPerViewNV`, or vice versa (to allow compilers to eliminate
the unused outputs).
All attributes that do not have `*PerViewNV` counterparts  **must**  not depend
on `ViewIndex`.Per-view attributes are all-or-nothing for a subpass.
That is, all pipelines compiled against a subpass that includes the
`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit  **must**  write
per-view attributes to the `*PerViewNV[]` shader outputs, in addition to the
non-per-view (e.g. `Position`) outputs.
Pipelines compiled against a subpass that does not include this bit  **must** 
not include the `*PerViewNV[]` outputs in their interfaces.
## Valid Usage
-    Each view index  **must**  not be set in more than one element of [`correlation_masks`]
-    If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature is not enabled, each element of [`view_masks`] **must**  be `0`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`
-    If [`subpass_count`] is not `0`, [`view_masks`] **must**  be a valid pointer to an array of [`subpass_count`]`uint32_t` values
-    If [`dependency_count`] is not `0`, [`view_offsets`] **must**  be a valid pointer to an array of [`dependency_count`]`int32_t` values
-    If [`correlation_mask_count`] is not `0`, [`correlation_masks`] **must**  be a valid pointer to an array of [`correlation_mask_count`]`uint32_t` values

# Related
- [`crate::vulkan1_1`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        