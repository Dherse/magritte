[vkCmdBindShadingRateImageNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadingRateImageNV.html) - Bind a shading rate image on a command buffer

# C Specifications
When shading rate image usage is enabled in the bound pipeline, the pipeline
uses a shading rate image specified by the command:
```c
// Provided by VK_NV_shading_rate_image
void vkCmdBindShadingRateImageNV(
    VkCommandBuffer                             commandBuffer,
    VkImageView                                 imageView,
    VkImageLayout                               imageLayout);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`image_view`] is an image view handle specifying the shading rate image. [`image_view`] **may**  be set to [`crate::Handle::null`], which is equivalent to specifying a view of an image filled with zero values.
- [`image_layout`] is the layout that the image subresources accessible from [`image_view`] will be in when the shading rate image is accessed.

# Description
## Valid Usage
-    The [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shadingRateImage) feature  **must**  be enabled
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  be a valid [`ImageView`] handle of type `VK_IMAGE_VIEW_TYPE_2D` or `VK_IMAGE_VIEW_TYPE_2D_ARRAY`
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  have a format of `VK_FORMAT_R8_UINT`
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with a `usage` value including `VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV`
-    If [`image_view`] is not [`crate::Handle::null`], [`image_layout`] **must**  match the actual [`ImageLayout`] of each subresource accessible from [`image_view`] at the time the subresource is accessed
-    If [`image_view`] is not [`crate::Handle::null`], [`image_layout`] **must**  be `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV` or `VK_IMAGE_LAYOUT_GENERAL`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-    If [`image_view`] is not [`crate::Handle::null`], [`image_view`] **must**  be a valid [`ImageView`] handle
-  [`image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    Both of [`command_buffer`], and [`image_view`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`nv_shading_rate_image`]
- [`CommandBuffer`]
- [`ImageLayout`]
- [`ImageView`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        