[VkAttachmentLoadOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentLoadOp.html) - Specify how contents of an attachment are treated at the beginning of a subpass

# C Specifications
Possible values of [`AttachmentDescription::load_op`] and
`stencilLoadOp`, specifying how the contents of the attachment are
treated, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkAttachmentLoadOp {
    VK_ATTACHMENT_LOAD_OP_LOAD = 0,
    VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
    VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,
  // Provided by VK_EXT_load_store_op_none
    VK_ATTACHMENT_LOAD_OP_NONE_EXT = 1000400000,
} VkAttachmentLoadOp;
```

# Description
- [`LOAD`] specifies that the previous contents of the image within the render area will be preserved. For attachments with a depth/stencil format, this uses the access type `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT`. For attachments with a color format, this uses the access type `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`.
- [`CLEAR`] specifies that the contents within the render area will be cleared to a uniform value, which is specified when a render pass instance is begun. For attachments with a depth/stencil format, this uses the access type `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`. For attachments with a color format, this uses the access type `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.
- [`DONT_CARE`] specifies that the previous contents within the area need not be preserved; the contents of the attachment will be undefined inside the render area. For attachments with a depth/stencil format, this uses the access type `VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`. For attachments with a color format, this uses the access type `VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT`.
- [`NONE_EXT`] specifies that the previous contents of the image within the render area will be preserved, but the contents of the attachment will be undefined inside the render pass. No access type is used as the image is not accessed.

# Related
- [`crate::vulkan1_0`]
- [`AttachmentDescription`]
- [`AttachmentDescription2`]
- [`RenderingAttachmentInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        