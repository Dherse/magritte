[vkCmdBeginRenderPass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass.html) - Begin a new render pass

# C Specifications
To begin a render pass instance, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdBeginRenderPass(
    VkCommandBuffer                             commandBuffer,
    const VkRenderPassBeginInfo*                pRenderPassBegin,
    VkSubpassContents                           contents);
```

# Parameters
- [`command_buffer`] is the command buffer in which to record the command.
- [`p_render_pass_begin`] is a pointer to a [`RenderPassBeginInfo`] structure specifying the render pass to begin an instance of, and the framebuffer the instance uses.
- [`contents`] is a [`SubpassContents`] value specifying how the commands in the first subpass will be provided.

# Description
After beginning a render pass instance, the command buffer is ready to
record the commands for the first subpass of that render pass.
## Valid Usage
-    If any of the `initialLayout` or `finalLayout` member of the [`AttachmentDescription`] structures or the `layout` member of the [`AttachmentReference`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL` then the corresponding attachment image view of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`] **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`
-    If any of the `initialLayout` or `finalLayout` member of the [`AttachmentDescription`] structures or the `layout` member of the [`AttachmentReference`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL` then the corresponding attachment image view of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`] **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If any of the `initialLayout` or `finalLayout` member of the [`AttachmentDescription`] structures or the `layout` member of the [`AttachmentReference`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` then the corresponding attachment image view of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`] **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If any of the `stencilInitialLayout` or `stencilFinalLayout` member of the [`AttachmentDescriptionStencilLayout`] structures or the `stencilLayout` member of the [`AttachmentReferenceStencilLayout`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` then the corresponding attachment image view of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`] **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If any of the `initialLayout` or `finalLayout` member of the [`AttachmentDescription`] structures or the `layout` member of the [`AttachmentReference`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL` then the corresponding attachment image view of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`] **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_SAMPLED_BIT` or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
-    If any of the `initialLayout` or `finalLayout` member of the [`AttachmentDescription`] structures or the `layout` member of the [`AttachmentReference`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` then the corresponding attachment image view of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`] **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_TRANSFER_SRC_BIT`
-    If any of the `initialLayout` or `finalLayout` member of the [`AttachmentDescription`] structures or the `layout` member of the [`AttachmentReference`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` then the corresponding attachment image view of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`] **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_TRANSFER_DST_BIT`
-    If the `initialLayout` member of any of the [`AttachmentDescription`] structures specified when creating the render pass specified in the `renderPass` member of [`p_render_pass_begin`] is not `VK_IMAGE_LAYOUT_UNDEFINED`, then each such `initialLayout` **must**  be equal to the current layout of the corresponding attachment image subresource of the framebuffer specified in the `framebuffer` member of [`p_render_pass_begin`]
-    The `srcStageMask` members of any element of the `pDependencies` member of [`RenderPassCreateInfo`] used to create `renderPass` **must**  be supported by the capabilities of the queue family identified by the `queueFamilyIndex` member of the [`CommandPoolCreateInfo`] used to create the command pool which [`command_buffer`] was allocated from
-    The `dstStageMask` members of any element of the `pDependencies` member of [`RenderPassCreateInfo`] used to create `renderPass` **must**  be supported by the capabilities of the queue family identified by the `queueFamilyIndex` member of the [`CommandPoolCreateInfo`] used to create the command pool which [`command_buffer`] was allocated from
-    For any attachment in `framebuffer` that is used by `renderPass` and is bound to memory locations that are also bound to another attachment used by `renderPass`, and if at least one of those uses causes either attachment to be written to, both attachments  **must**  have had the `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT` set

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_render_pass_begin`] **must**  be a valid pointer to a valid [`RenderPassBeginInfo`] structure
-  [`contents`] **must**  be a valid [`SubpassContents`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called outside of a render pass instance
-  [`command_buffer`] **must**  be a primary [`CommandBuffer`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`RenderPassBeginInfo`]
- [`SubpassContents`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        