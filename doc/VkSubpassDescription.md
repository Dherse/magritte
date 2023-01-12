[VkSubpassDescription](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription.html) - Structure specifying a subpass description

# C Specifications
The [`SubpassDescription`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkSubpassDescription {
    VkSubpassDescriptionFlags       flags;
    VkPipelineBindPoint             pipelineBindPoint;
    uint32_t                        inputAttachmentCount;
    const VkAttachmentReference*    pInputAttachments;
    uint32_t                        colorAttachmentCount;
    const VkAttachmentReference*    pColorAttachments;
    const VkAttachmentReference*    pResolveAttachments;
    const VkAttachmentReference*    pDepthStencilAttachment;
    uint32_t                        preserveAttachmentCount;
    const uint32_t*                 pPreserveAttachments;
} VkSubpassDescription;
```

# Members
- [`flags`] is a bitmask of [`SubpassDescriptionFlagBits`] specifying usage of the subpass.
- [`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying the pipeline type supported for this subpass.
- [`input_attachment_count`] is the number of input attachments.
- [`input_attachments`] is a pointer to an array of [`AttachmentReference`] structures defining the input attachments for this subpass and their layouts.
- [`color_attachment_count`] is the number of color attachments.
- [`color_attachments`] is a pointer to an array of [`color_attachment_count`][`AttachmentReference`] structures defining the color attachments for this subpass and their layouts.
- [`resolve_attachments`] is `NULL` or a pointer to an array of [`color_attachment_count`][`AttachmentReference`] structures defining the resolve attachments for this subpass and their layouts.
- [`depth_stencil_attachment`] is a pointer to a [`AttachmentReference`] structure specifying the depth/stencil attachment for this subpass and its layout.
- [`preserve_attachment_count`] is the number of preserved attachments.
- [`preserve_attachments`] is a pointer to an array of [`preserve_attachment_count`] render pass attachment indices identifying attachments that are not used by this subpass, but whose contents  **must**  be preserved throughout the subpass.

# Description
Each element of the [`input_attachments`] array corresponds to an input
attachment index in a fragment shader, i.e. if a shader declares an image
variable decorated with a `InputAttachmentIndex` value of  **X** , then it
uses the attachment provided in [`input_attachments`][ **X** ].
Input attachments  **must**  also be bound to the pipeline in a descriptor set.
If the `attachment` member of any element of [`input_attachments`] is
`VK_ATTACHMENT_UNUSED`, the application  **must**  not read from the
corresponding input attachment index.
Fragment shaders  **can**  use subpass input variables to access the contents of
an input attachment at the fragment’s (x, y, layer) framebuffer coordinates.
Input attachments  **must**  not be used by any subpasses within a render pass
that enables [render pass transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-renderpass-transform).Each element of the [`color_attachments`] array corresponds to an output
location in the shader, i.e. if the shader declares an output variable
decorated with a `Location` value of  **X** , then it uses the attachment
provided in [`color_attachments`][ **X** ].
If the `attachment` member of any element of [`color_attachments`] is
`VK_ATTACHMENT_UNUSED`,
or if [Color Write Enable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-color-write-enable) has been
disabled for the corresponding attachment index,
then writes to the corresponding location by a fragment shader are
discarded.If
[`flags`] does not include
`VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if
[`resolve_attachments`] is not `NULL`, each of its elements corresponds to
a color attachment (the element in [`color_attachments`] at the same
index), and a multisample resolve operation is defined for each attachment.
At the end of each subpass, multisample resolve operations read the
subpass’s color attachments, and resolve the samples for each pixel within
the render area to the same pixel location in the corresponding resolve
attachments, unless the resolve attachment index is
`VK_ATTACHMENT_UNUSED`.Similarly, if
[`flags`] does not include
`VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and
[`SubpassDescriptionDepthStencilResolve::depth_stencil_resolve_attachment`]
is not `NULL` and does not have the value `VK_ATTACHMENT_UNUSED`, it
corresponds to the depth/stencil attachment in
[`depth_stencil_attachment`], and multisample resolve operations for depth
and stencil are defined by
[`SubpassDescriptionDepthStencilResolve::depth_resolve_mode`] and
[`SubpassDescriptionDepthStencilResolve::stencil_resolve_mode`],
respectively.
At the end of each subpass, multisample resolve operations read the
subpass’s depth/stencil attachment, and resolve the samples for each pixel
to the same pixel location in the corresponding resolve attachment.
If [`SubpassDescriptionDepthStencilResolve::depth_resolve_mode`] is
`VK_RESOLVE_MODE_NONE`, then the depth component of the resolve
attachment is not written to and its contents are preserved.
Similarly, if
[`SubpassDescriptionDepthStencilResolve::stencil_resolve_mode`] is
`VK_RESOLVE_MODE_NONE`, then the stencil component of the resolve
attachment is not written to and its contents are preserved.
[`SubpassDescriptionDepthStencilResolve::depth_resolve_mode`] is
ignored if the [`Format`] of the `pDepthStencilResolveAttachment`
does not have a depth component.
Similarly,
[`SubpassDescriptionDepthStencilResolve::stencil_resolve_mode`] is
ignored if the [`Format`] of the `pDepthStencilResolveAttachment`
does not have a stencil component.If the image subresource range referenced by the depth/stencil attachment is
created with
`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT`, then the
multisample resolve operation uses the sample locations state specified in
the `sampleLocationsInfo` member of the element of the
[`RenderPassSampleLocationsBeginInfoEXT::post_subpass_sample_locations`]
for the subpass.If [`depth_stencil_attachment`] is `NULL`, or if its attachment index is
`VK_ATTACHMENT_UNUSED`, it indicates that no depth/stencil attachment
will be used in the subpass.The contents of an attachment within the render area become undefined at
the start of a subpass  **S**  if all of the following conditions are true:
- The attachment is used as a color, depth/stencil, or resolve attachment in any subpass in the render pass.
- There is a subpass  **S<sub>1</sub>**  that uses or preserves the attachment, and a subpass dependency from  **S<sub>1</sub>**  to  **S** .
- The attachment is not used or preserved in subpass  **S** .
In addition, the contents of an attachment within the render area become
undefined at the start of a subpass  **S**  if all of the following conditions
are true:
- `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM` is set.
- The attachment is used as a color or depth/stencil in the subpass.
Once the contents of an attachment become undefined in subpass  **S** , they
remain undefined for subpasses in subpass dependency chains starting with
subpass  **S**  until they are written again.
However, they remain valid for subpasses in other subpass dependency chains
starting with subpass  **S<sub>1</sub>**  if those subpasses use or preserve the
attachment.
## Valid Usage
-  [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS` or `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`
-  [`color_attachment_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_color_attachments`]
-    If the first use of an attachment in this render pass is as an input attachment, and the attachment is not also used as a color or depth/stencil attachment in the same subpass, then `loadOp` **must**  not be `VK_ATTACHMENT_LOAD_OP_CLEAR`
-    If [`resolve_attachments`] is not `NULL`, for each resolve attachment that is not `VK_ATTACHMENT_UNUSED`, the corresponding color attachment  **must**  not be `VK_ATTACHMENT_UNUSED`
-    If [`resolve_attachments`] is not `NULL`, for each resolve attachment that is not `VK_ATTACHMENT_UNUSED`, the corresponding color attachment  **must**  not have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    If [`resolve_attachments`] is not `NULL`, each resolve attachment that is not `VK_ATTACHMENT_UNUSED` **must**  have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    If [`resolve_attachments`] is not `NULL`, each resolve attachment that is not `VK_ATTACHMENT_UNUSED` **must**  have the same [`Format`] as its corresponding color attachment
-    All attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have the same sample count
-    All attachments in [`input_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain at least `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    All attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    All attachments in [`resolve_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
-    If [`depth_stencil_attachment`] is not `NULL` and the attachment is not `VK_ATTACHMENT_UNUSED` then it  **must**  have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`input_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`resolve_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
-    If the `[`amd_mixed_attachment_samples`]` extension is enabled, and all attachments in [`color_attachments`] that are not `VK_ATTACHMENT_UNUSED` **must**  have a sample count that is smaller than or equal to the sample count of [`depth_stencil_attachment`] if it is not `VK_ATTACHMENT_UNUSED`
-    If neither the `[`amd_mixed_attachment_samples`]` nor the `[`nv_framebuffer_mixed_samples`]` extensions are enabled, and if [`depth_stencil_attachment`] is not `VK_ATTACHMENT_UNUSED` and any attachments in [`color_attachments`] are not `VK_ATTACHMENT_UNUSED`, they  **must**  have the same sample count
-    Each element of [`preserve_attachments`] **must**  not be `VK_ATTACHMENT_UNUSED`
-    Each element of [`preserve_attachments`] **must**  not also be an element of any other member of the subpass description
-    If any attachment is used by more than one [`AttachmentReference`] member, then each use  **must**  use the same `layout`
-    Each attachment  **must**  follow the [image layout requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#attachment-type-imagelayout) specified for its attachment type
-    If [`flags`] includes `VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX`, it  **must**  also include `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX`
-    If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if [`resolve_attachments`] is not `NULL`, then each resolve attachment  **must**  be `VK_ATTACHMENT_UNUSED`
-    If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, then the subpass  **must**  be the last subpass in a subpass dependency chain
-    If the render pass is created with `VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM` each of the elements of [`input_attachments`] **must**  be `VK_ATTACHMENT_UNUSED`
-  [`depth_stencil_attachment`] and [`color_attachments`] must not contain references to the same attachment

## Valid Usage (Implicit)
-  [`flags`] **must**  be a valid combination of [`SubpassDescriptionFlagBits`] values
-  [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
-    If [`input_attachment_count`] is not `0`, [`input_attachments`] **must**  be a valid pointer to an array of [`input_attachment_count`] valid [`AttachmentReference`] structures
-    If [`color_attachment_count`] is not `0`, [`color_attachments`] **must**  be a valid pointer to an array of [`color_attachment_count`] valid [`AttachmentReference`] structures
-    If [`color_attachment_count`] is not `0`, and [`resolve_attachments`] is not `NULL`, [`resolve_attachments`] **must**  be a valid pointer to an array of [`color_attachment_count`] valid [`AttachmentReference`] structures
-    If [`depth_stencil_attachment`] is not `NULL`, [`depth_stencil_attachment`] **must**  be a valid pointer to a valid [`AttachmentReference`] structure
-    If [`preserve_attachment_count`] is not `0`, [`preserve_attachments`] **must**  be a valid pointer to an array of [`preserve_attachment_count`]`uint32_t` values

# Related
- [`crate::vulkan1_0`]
- [`AttachmentReference`]
- [`PipelineBindPoint`]
- [`RenderPassCreateInfo`]
- [VkSubpassDescriptionFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        