[vkCmdBindInvocationMaskHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html) - Bind an invocation mask image on a command buffer

# C Specifications
When invocation mask image usage is enabled in the bound ray tracing
pipeline, the pipeline uses an invocation mask image specified by the
command:
```c
// Provided by VK_HUAWEI_invocation_mask
void vkCmdBindInvocationMaskHUAWEI(
    VkCommandBuffer                             commandBuffer,
    VkImageView                                 imageView,
    VkImageLayout                               imageLayout);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded
- [`image_view`] is an image view handle specifying the invocation mask image [`image_view`] **may**  be set to [`crate::Handle::null`], which is equivalent to specifying a view of an image filled with ones value.
- [`image_layout`] is the layout that the image subresources accessible from [`image_view`] will be in when the invocation mask image is accessed

# Description
## Valid Usage
-    The [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-invocationMask) feature  **must**  be enabled
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  be a valid [`ImageView`] handle of type `VK_IMAGE_VIEW_TYPE_2D`
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  have a format of `VK_FORMAT_R8_UINT`
-    If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with `VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI` set
-    If [`image_view`] is not [`crate::Handle::null`], [`image_layout`] **must**  be `VK_IMAGE_LAYOUT_GENERAL`
-    Thread mask image resolution must match the `width` and `height` in [`cmd_trace_rays_khr`]
-    Each element in the invocation mask image  **must**  have the value `0` or `1`. The value 1 means the invocation is active
-  `width` in [`cmd_trace_rays_khr`] should be 1

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-    If [`image_view`] is not [`crate::Handle::null`], [`image_view`] **must**  be a valid [`ImageView`] handle
-  [`image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Both of [`command_buffer`], and [`image_view`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_HUAWEI_invocation_mask`]
- [`CommandBuffer`]
- [`ImageLayout`]
- [`ImageView`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        