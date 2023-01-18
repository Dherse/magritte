[VkInputAttachmentAspectReference](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInputAttachmentAspectReference.html) - Structure specifying a subpass/input attachment pair and an aspect mask that can: be read.

# C Specifications
The [`InputAttachmentAspectReference`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkInputAttachmentAspectReference {
    uint32_t              subpass;
    uint32_t              inputAttachmentIndex;
    VkImageAspectFlags    aspectMask;
} VkInputAttachmentAspectReference;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance2
typedef VkInputAttachmentAspectReference VkInputAttachmentAspectReferenceKHR;
```

# Members
- [`subpass`] is an index into the `pSubpasses` array of the parent [`RenderPassCreateInfo`] structure.
- [`input_attachment_index`] is an index into the `pInputAttachments` of the specified subpass.
- [`aspect_mask`] is a mask of which aspect(s)  **can**  be accessed within the specified subpass.

# Description
This structure specifies an aspect mask for a specific input attachment of a
specific subpass in the render pass.[`subpass`] and [`input_attachment_index`] index into the render pass as:
```c
pCreateInfo->pSubpasses[subpass].pInputAttachments[inputAttachmentIndex]
```

## Valid Usage
-  [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_METADATA_BIT`
-  [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index *i*

## Valid Usage (Implicit)
-  [`aspect_mask`] **must**  be a valid combination of [`ImageAspectFlagBits`] values
-  [`aspect_mask`] **must**  not be `0`

# Related
- [`crate::vulkan1_1`]
- [`ImageAspectFlags`]
- [`RenderPassInputAttachmentAspectCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        