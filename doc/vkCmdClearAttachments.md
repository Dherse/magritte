[vkCmdClearAttachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdClearAttachments.html) - Clear regions within bound framebuffer attachments

# C Specifications
To clear one or more regions of color and depth/stencil attachments inside a
render pass instance, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdClearAttachments(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    attachmentCount,
    const VkClearAttachment*                    pAttachments,
    uint32_t                                    rectCount,
    const VkClearRect*                          pRects);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`attachment_count`] is the number of entries in the [`p_attachments`] array.
- [`p_attachments`] is a pointer to an array of [`ClearAttachment`] structures defining the attachments to clear and the clear values to use. If any attachment index to be cleared is not backed by an image view, then the clear has no effect.
- [`rect_count`] is the number of entries in the [`p_rects`] array.
- [`p_rects`] is a pointer to an array of [`ClearRect`] structures defining regions within each selected attachment to clear.

# Description
If the render pass has a [fragment
density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment), clears follow the
[operations of fragment density maps](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymapops) as if each
clear region was a primitive which generates fragments.
The clear color is applied to all pixels inside each fragment’s area
regardless if the pixels lie outside of the clear region.
Clears  **may**  have a different set of supported fragment areas than draws.Unlike other [clear commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears), [`cmd_clear_attachments`] executes
as a drawing command, rather than a transfer command, with writes performed
by it executing in [rasterization order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-order).
Clears to color attachments are executed as color attachment writes, by the
`VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` stage.
Clears to depth/stencil attachments are executed as [depth
writes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-depth) and [writes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-stencil) by the
`VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT` and
`VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT` stages.[`cmd_clear_attachments`] is not affected by the bound pipeline state.
## Valid Usage
-    If the `aspectMask` member of any element of [`p_attachments`] contains `VK_IMAGE_ASPECT_COLOR_BIT`, then the `colorAttachment` member of that element  **must**  either refer to a color attachment which is [`ATTACHMENT_UNUSED`], or  **must**  be a valid color attachment
-    If the `aspectMask` member of any element of [`p_attachments`] contains `VK_IMAGE_ASPECT_DEPTH_BIT`, then the current subpass' depth/stencil attachment  **must**  either be [`ATTACHMENT_UNUSED`], or  **must**  have a depth component
-    If the `aspectMask` member of any element of [`p_attachments`] contains `VK_IMAGE_ASPECT_STENCIL_BIT`, then the current subpass' depth/stencil attachment  **must**  either be [`ATTACHMENT_UNUSED`], or  **must**  have a stencil component
-    The `rect` member of each element of [`p_rects`] **must**  have an `extent.width` greater than `0`
-    The `rect` member of each element of [`p_rects`] **must**  have an `extent.height` greater than `0`
-    The rectangular region specified by each element of [`p_rects`] **must**  be contained within the render area of the current render pass instance
-    The layers specified by each element of [`p_rects`] **must**  be contained within every attachment that [`p_attachments`] refers to
-    The `layerCount` member of each element of [`p_rects`] **must**  not be `0`
-    If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, each attachment to be cleared  **must**  not be a protected image
-    If [`command_buffer`] is a protected command buffer and [`protectedNoFault`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-protectedNoFault) is not supported, each attachment to be cleared  **must**  not be an unprotected image
-    If the render pass instance this is recorded in uses multiview, then `baseArrayLayer` **must**  be zero and `layerCount` **must**  be one

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_attachments`] **must**  be a valid pointer to an array of [`attachment_count`] valid [`ClearAttachment`] structures
-  [`p_rects`] **must**  be a valid pointer to an array of [`rect_count`][`ClearRect`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called inside of a render pass instance
-  [`attachment_count`] **must**  be greater than `0`
-  [`rect_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`ClearAttachment`]
- [`ClearRect`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        