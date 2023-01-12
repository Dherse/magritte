[VkRenderPassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassBeginInfo.html) - Structure specifying render pass begin information

# C Specifications
The [`RenderPassBeginInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkRenderPassBeginInfo {
    VkStructureType        sType;
    const void*            pNext;
    VkRenderPass           renderPass;
    VkFramebuffer          framebuffer;
    VkRect2D               renderArea;
    uint32_t               clearValueCount;
    const VkClearValue*    pClearValues;
} VkRenderPassBeginInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`render_pass`] is the render pass to begin an instance of.
- [`framebuffer`] is the framebuffer containing the attachments that are used with the render pass.
- [`render_area`] is the render area that is affected by the render pass instance, and is described in more detail below.
- [`clear_value_count`] is the number of elements in [`clear_values`].
- [`clear_values`] is a pointer to an array of [`clear_value_count`][`ClearValue`] structures containing clear values for each attachment, if the attachment uses a `loadOp` value of `VK_ATTACHMENT_LOAD_OP_CLEAR` or if the attachment has a depth/stencil format and uses a `stencilLoadOp` value of `VK_ATTACHMENT_LOAD_OP_CLEAR`. The array is indexed by attachment number. Only elements corresponding to cleared attachments are used. Other elements of [`clear_values`] are ignored.

# Description
[`render_area`] is the render area that is affected by the render pass
instance.
The effects of attachment load, store and multisample resolve operations are
restricted to the pixels whose x and y coordinates fall within the render
area on all attachments.
The render area extends to all layers of [`framebuffer`].
The application  **must**  ensure (using scissor if necessary) that all rendering
is contained within the render area.
The render area, after any transform specified by
[`RenderPassTransformBeginInfoQCOM::transform`] is applied,  **must** 
be contained within the framebuffer dimensions.If [render pass transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-renderpass-transform) is
enabled, then [`render_area`] **must**  equal the framebuffer pre-transformed
dimensions.
After [`render_area`] has been transformed by
[`RenderPassTransformBeginInfoQCOM::transform`], the resulting
render area  **must**  be equal to the framebuffer dimensions.If [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-subpassShading) is enabled, then
[`render_area`] **must**  equal the framebuffer dimensions.When multiview is enabled, the resolve operation at the end of a subpass
applies to all views in the view mask.
## Valid Usage
-  [`clear_value_count`] **must**  be greater than the largest attachment index in [`render_pass`] specifying a `loadOp` (or `stencilLoadOp`, if the attachment has a depth/stencil format) of `VK_ATTACHMENT_LOAD_OP_CLEAR`
-    If [`clear_value_count`] is not `0`, [`clear_values`] **must**  be a valid pointer to an array of [`clear_value_count`][`ClearValue`] unions
-  [`render_pass`] **must**  be [compatible](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) with the [`render_pass`] member of the [`FramebufferCreateInfo`] structure specified when creating [`framebuffer`]
-    If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its `deviceRenderAreaCount` member is equal to 0, `renderArea.offset.x` **must**  be greater than or equal to 0
-    If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its `deviceRenderAreaCount` member is equal to 0, `renderArea.offset.y` **must**  be greater than or equal to 0
-    If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its `deviceRenderAreaCount` member is equal to 0, `renderArea.offset.x` +  `renderArea.extent.width` **must**  be less than or equal to [`FramebufferCreateInfo::width`] the [`framebuffer`] was created with
-    If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its `deviceRenderAreaCount` member is equal to 0, `renderArea.offset.y` +  `renderArea.extent.height` **must**  be less than or equal to [`FramebufferCreateInfo::height`] the [`framebuffer`] was created with
-    If the [`p_next`] chain contains [`DeviceGroupRenderPassBeginInfo`], `offset.x` +  `extent.width` of each element of `pDeviceRenderAreas` **must**  be less than or equal to [`FramebufferCreateInfo::width`] the [`framebuffer`] was created with
-    If the [`p_next`] chain contains [`DeviceGroupRenderPassBeginInfo`], `offset.y` +  `extent.height` of each element of `pDeviceRenderAreas` **must**  be less than or equal to [`FramebufferCreateInfo::height`] the [`framebuffer`] was created with
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that did not include `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, and the [`p_next`] chain includes a [`RenderPassAttachmentBeginInfo`] structure, its `attachmentCount` **must**  be zero
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, the `attachmentCount` of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be equal to the value of [`FramebufferAttachmentsCreateInfo::attachment_image_info_count`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  have been created on the same [`Device`] as [`framebuffer`] and [`render_pass`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] of an image created with a value of [`ImageCreateInfo::flags`] equal to the `flags` member of the corresponding element of [`FramebufferAttachmentsCreateInfo::attachment_image_infos`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] with [an inherited usage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-inherited-usage) equal to the `usage` member of the corresponding element of [`FramebufferAttachmentsCreateInfo::attachment_image_infos`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] with a width equal to the `width` member of the corresponding element of [`FramebufferAttachmentsCreateInfo::attachment_image_infos`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] with a height equal to the `height` member of the corresponding element of [`FramebufferAttachmentsCreateInfo::attachment_image_infos`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] of an image created with a value of [`ImageViewCreateInfo`]::`subresourceRange.layerCount` equal to the `layerCount` member of the corresponding element of [`FramebufferAttachmentsCreateInfo::attachment_image_infos`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] of an image created with a value of [`ImageFormatListCreateInfo::view_format_count`] equal to the `viewFormatCount` member of the corresponding element of [`FramebufferAttachmentsCreateInfo::attachment_image_infos`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] of an image created with a set of elements in [`ImageFormatListCreateInfo::view_formats`] equal to the set of elements in the `pViewFormats` member of the corresponding element of [`FramebufferAttachmentsCreateInfo::attachment_image_infos`] used to create [`framebuffer`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] of an image created with a value of [`ImageViewCreateInfo::format`] equal to the corresponding value of [`AttachmentDescription::format`] in [`render_pass`]
-    If [`framebuffer`] was created with a [`FramebufferCreateInfo::flags`] value that included `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT`, each element of the `pAttachments` member of a [`RenderPassAttachmentBeginInfo`] structure included in the [`p_next`] chain  **must**  be a [`ImageView`] of an image created with a value of [`ImageCreateInfo::samples`] equal to the corresponding value of [`AttachmentDescription::samples`] in [`render_pass`]
-    If the [`p_next`] chain includes [`RenderPassTransformBeginInfoQCOM`], `renderArea.offset` **must**  equal (0,0)
-    If the [`p_next`] chain includes [`RenderPassTransformBeginInfoQCOM`], `renderArea.extent` transformed by [`RenderPassTransformBeginInfoQCOM::transform`] **must**  equal the [`framebuffer`] dimensions

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DeviceGroupRenderPassBeginInfo`], [`RenderPassAttachmentBeginInfo`], [`RenderPassSampleLocationsBeginInfoEXT`], or [`RenderPassTransformBeginInfoQCOM`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`render_pass`] **must**  be a valid [`RenderPass`] handle
-  [`framebuffer`] **must**  be a valid [`Framebuffer`] handle
-    Both of [`framebuffer`], and [`render_pass`] **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`crate::vulkan1_0`]
- [`ClearValue`]
- [`Framebuffer`]
- [`Rect2D`]
- [`RenderPass`]
- [`StructureType`]
- [`cmd_begin_render_pass`]
- [`cmd_begin_render_pass2`]
- [`cmd_begin_render_pass2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        