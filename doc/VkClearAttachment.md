[VkClearAttachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearAttachment.html) - Structure specifying a clear attachment

# C Specifications
The [`ClearAttachment`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkClearAttachment {
    VkImageAspectFlags    aspectMask;
    uint32_t              colorAttachment;
    VkClearValue          clearValue;
} VkClearAttachment;
```

# Members
- [`aspect_mask`] is a mask selecting the color, depth and/or stencil aspects of the attachment to be cleared.
- [`color_attachment`] is only meaningful if `VK_IMAGE_ASPECT_COLOR_BIT` is set in [`aspect_mask`], in which case it is an index into the currently bound color attachments.
- [`clear_value`] is the color or depth/stencil value to clear the attachment to, as described in [Clear Values](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears-values) below.

# Description
## Valid Usage
-    If [`aspect_mask`] includes `VK_IMAGE_ASPECT_COLOR_BIT`, it  **must**  not include `VK_IMAGE_ASPECT_DEPTH_BIT` or `VK_IMAGE_ASPECT_STENCIL_BIT`
-  [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_METADATA_BIT`
-  [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index *i*
-  [`clear_value`] **must**  be a valid [`ClearValue`] union

## Valid Usage (Implicit)
-  [`aspect_mask`] **must**  be a valid combination of [`ImageAspectFlagBits`] values
-  [`aspect_mask`] **must**  not be `0`

# Related
- [`crate::vulkan1_0`]
- [`ClearValue`]
- [`ImageAspectFlags`]
- [`cmd_clear_attachments`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        