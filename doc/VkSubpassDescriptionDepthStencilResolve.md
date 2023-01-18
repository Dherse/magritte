[VkSubpassDescriptionDepthStencilResolve](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolve.html) - Structure specifying depth/stencil resolve operations for a subpass

# C Specifications
If the [`p_next`] chain of [`SubpassDescription2`] includes a
[`SubpassDescriptionDepthStencilResolve`] structure, then that structure
describes multisample resolve operations for the depth/stencil attachment in
a subpass.The [`SubpassDescriptionDepthStencilResolve`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSubpassDescriptionDepthStencilResolve {
    VkStructureType                  sType;
    const void*                      pNext;
    VkResolveModeFlagBits            depthResolveMode;
    VkResolveModeFlagBits            stencilResolveMode;
    const VkAttachmentReference2*    pDepthStencilResolveAttachment;
} VkSubpassDescriptionDepthStencilResolve;
```
or the equivalent
```c
// Provided by VK_KHR_depth_stencil_resolve
typedef VkSubpassDescriptionDepthStencilResolve VkSubpassDescriptionDepthStencilResolveKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`depth_resolve_mode`] is a [`ResolveModeFlagBits`] value describing the depth resolve mode.
- [`stencil_resolve_mode`] is a [`ResolveModeFlagBits`] value describing the stencil resolve mode.
- [`depth_stencil_resolve_attachment`] is `NULL` or a pointer to a [`AttachmentReference2`] structure defining the depth/stencil resolve attachment for this subpass and its layout.

# Description
If [`depth_stencil_resolve_attachment`] is `NULL`, or if its attachment
index is [`ATTACHMENT_UNUSED`], it indicates that no depth/stencil
resolve attachment will be used in the subpass.
## Valid Usage
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], `pDepthStencilAttachment` **must**  not be `NULL` or have the value [`ATTACHMENT_UNUSED`]
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], [`depth_resolve_mode`] and [`stencil_resolve_mode`] **must**  not both be `VK_RESOLVE_MODE_NONE`
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], `pDepthStencilAttachment` **must**  not have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], [`depth_stencil_resolve_attachment`] **must**  have a sample count of `VK_SAMPLE_COUNT_1_BIT`
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`] then it  **must**  have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`] and [`Format`] of [`depth_stencil_resolve_attachment`] has a depth component, then the [`Format`] of `pDepthStencilAttachment` **must**  have a depth component with the same number of bits and numerical type
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], and [`Format`] of [`depth_stencil_resolve_attachment`] has a stencil component, then the [`Format`] of `pDepthStencilAttachment` **must**  have a stencil component with the same number of bits and numerical type
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`] and the [`Format`] of [`depth_stencil_resolve_attachment`] has a depth component, then the value of [`depth_resolve_mode`] **must**  be one of the bits set in [`PhysicalDeviceDepthStencilResolveProperties::supported_depth_resolve_modes`] or `VK_RESOLVE_MODE_NONE`
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`] and the [`Format`] of [`depth_stencil_resolve_attachment`] has a stencil component, then the value of [`stencil_resolve_mode`] **must**  be one of the bits set in [`PhysicalDeviceDepthStencilResolveProperties::supported_stencil_resolve_modes`] or `VK_RESOLVE_MODE_NONE`
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], the [`Format`] of [`depth_stencil_resolve_attachment`] has both depth and stencil components, [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve`] is [`FALSE`], and [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is [`FALSE`], then the values of [`depth_resolve_mode`] and [`stencil_resolve_mode`] **must**  be identical
-    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`], the [`Format`] of [`depth_stencil_resolve_attachment`] has both depth and stencil components, [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve`] is [`FALSE`] and [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is [`TRUE`], then the values of [`depth_resolve_mode`] and [`stencil_resolve_mode`] **must**  be identical or one of them  **must**  be `VK_RESOLVE_MODE_NONE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`
-    If [`depth_stencil_resolve_attachment`] is not `NULL`, [`depth_stencil_resolve_attachment`] **must**  be a valid pointer to a valid [`AttachmentReference2`] structure

# Related
- [`VK_KHR_depth_stencil_resolve`]
- [`crate::vulkan1_2`]
- [`AttachmentReference2`]
- [`ResolveModeFlagBits`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        