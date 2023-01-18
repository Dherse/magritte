[VkAttachmentDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionFlagBits.html) - Bitmask specifying additional properties of an attachment

# C Specifications
Bits which  **can**  be set in [`AttachmentDescription::flags`],
describing additional properties of the attachment, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkAttachmentDescriptionFlagBits {
    VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x00000001,
} VkAttachmentDescriptionFlagBits;
```

# Description
- [`MAY_ALIAS`] specifies that the attachment aliases the same device memory as other attachments.

# Related
- [`crate::vulkan1_0`]
- [`AttachmentDescriptionFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        