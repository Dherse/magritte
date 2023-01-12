[VkRenderingAttachmentInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfo.html) - Structure specifying attachment information

# C Specifications
The [`RenderingAttachmentInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkRenderingAttachmentInfo {
    VkStructureType          sType;
    const void*              pNext;
    VkImageView              imageView;
    VkImageLayout            imageLayout;
    VkResolveModeFlagBits    resolveMode;
    VkImageView              resolveImageView;
    VkImageLayout            resolveImageLayout;
    VkAttachmentLoadOp       loadOp;
    VkAttachmentStoreOp      storeOp;
    VkClearValue             clearValue;
} VkRenderingAttachmentInfo;
```
or the equivalent
```c
// Provided by VK_KHR_dynamic_rendering
typedef VkRenderingAttachmentInfo VkRenderingAttachmentInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`image_view`] is the image view that will be used for rendering.
- [`image_layout`] is the layout that [`image_view`] will be in during rendering.
- [`resolve_mode`] is a [`ResolveModeFlagBits`] value defining how multisampled data written to [`image_view`] will be resolved.
- [`resolve_image_view`] is an image view used to write resolved multisample data at the end of rendering.
- [`resolve_image_layout`] is the layout that [`resolve_image_view`] will be in during rendering.
- [`load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of [`image_view`] are treated at the start of the render pass instance.
- [`store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of [`image_view`] are treated at the end of the render pass instance.
- [`clear_value`] is a [`ClearValue`] structure defining values used to clear [`image_view`] when [`load_op`] is `VK_ATTACHMENT_LOAD_OP_CLEAR`.

# Description
Values in [`image_view`] are loaded and stored according to the values of
[`load_op`] and [`store_op`], within the render area
for each device
specified in [`RenderingInfo`].
If [`image_view`] is [`crate::Handle::null`], other members of this structure
are ignored; writes to this attachment will be discarded, and no load,
store, or resolve operations will be performed.If [`resolve_mode`] is `VK_RESOLVE_MODE_NONE`, then
[`resolve_image_view`] is ignored.
If [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, values in
[`resolve_image_view`] within the render area become undefined once
rendering begins.
At the end of rendering, the color values written to each pixel location in
[`image_view`] will be resolved according to [`resolve_mode`] and stored
into the the same location in [`resolve_image_view`].Store and resolve operations are only performed at the end of a render pass
instance that does not specify the `VK_RENDERING_SUSPENDING_BIT_KHR`
flag.Load operations are only performed at the beginning of a render pass
instance that does not specify the `VK_RENDERING_RESUMING_BIT_KHR` flag.Image contents at the end of a suspended render pass instance remain defined
for access by a resuming render pass instance.
## Valid Usage
-    If [`image_view`] is not [`crate::Handle::null`] and has a non-integer color format, [`resolve_mode`] **must**  be `VK_RESOLVE_MODE_NONE` or `VK_RESOLVE_MODE_AVERAGE_BIT`
-    If [`image_view`] is not [`crate::Handle::null`] and has an integer color format, [`resolve_mode`] **must**  be `VK_RESOLVE_MODE_NONE` or `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`image_view`] **must**  not have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_view`] **must**  have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`image_view`] and [`resolve_image_view`] **must**  have the same [`Format`]
-    If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED`, `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`, or `VK_IMAGE_LAYOUT_PREINITIALIZED`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`, or `VK_IMAGE_LAYOUT_PREINITIALIZED`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
-    If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  not be `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`] **must**  not be `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV`
-    If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  not be `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`] **must**  not be `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`] **must**  not be `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR`
-    If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  not be `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`] **must**  not be `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
-    If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  not be `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
-    If [`image_view`] is not [`crate::Handle::null`] and [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`] **must**  not be `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO`
-  [`p_next`] **must**  be `NULL`
-    If [`image_view`] is not [`crate::Handle::null`], [`image_view`] **must**  be a valid [`ImageView`] handle
-  [`image_layout`] **must**  be a valid [`ImageLayout`] value
-    If [`resolve_mode`] is not `0`, [`resolve_mode`] **must**  be a valid [`ResolveModeFlagBits`] value
-    If [`resolve_image_view`] is not [`crate::Handle::null`], [`resolve_image_view`] **must**  be a valid [`ImageView`] handle
-  [`resolve_image_layout`] **must**  be a valid [`ImageLayout`] value
-  [`load_op`] **must**  be a valid [`AttachmentLoadOp`] value
-  [`store_op`] **must**  be a valid [`AttachmentStoreOp`] value
-  [`clear_value`] **must**  be a valid [`ClearValue`] union
-    Both of [`image_view`], and [`resolve_image_view`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`khr_dynamic_rendering`]
- [`crate::vulkan1_3`]
- [`AttachmentLoadOp`]
- [`AttachmentStoreOp`]
- [`ClearValue`]
- [`ImageLayout`]
- [`ImageView`]
- [`RenderingInfo`]
- [`ResolveModeFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        