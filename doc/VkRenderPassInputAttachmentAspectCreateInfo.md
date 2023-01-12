[VkRenderPassInputAttachmentAspectCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html) - Structure specifying, for a given subpass/input attachment pair, which aspect can: be read.

# C Specifications
The [`RenderPassInputAttachmentAspectCreateInfo`] structure is defined
as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkRenderPassInputAttachmentAspectCreateInfo {
    VkStructureType                            sType;
    const void*                                pNext;
    uint32_t                                   aspectReferenceCount;
    const VkInputAttachmentAspectReference*    pAspectReferences;
} VkRenderPassInputAttachmentAspectCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_maintenance2
typedef VkRenderPassInputAttachmentAspectCreateInfo VkRenderPassInputAttachmentAspectCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`aspect_reference_count`] is the number of elements in the [`aspect_references`] array.
- [`aspect_references`] is a pointer to an array of [`aspect_reference_count`][`InputAttachmentAspectReference`] structures containing a mask describing which aspect(s)  **can**  be accessed for a given input attachment within a given subpass.

# Description
To specify which aspects of an input attachment  **can**  be read, add a
[`RenderPassInputAttachmentAspectCreateInfo`] structure to the
[`p_next`] chain of the [`RenderPassCreateInfo`] structure:An application  **can**  access any aspect of an input attachment that does not
have a specified aspect mask in the [`aspect_references`] array.
Otherwise, an application  **must**  not access aspect(s) of an input attachment
other than those in its specified aspect mask.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`
-  [`aspect_references`] **must**  be a valid pointer to an array of [`aspect_reference_count`] valid [`InputAttachmentAspectReference`] structures
-  [`aspect_reference_count`] **must**  be greater than `0`

# Related
- [`crate::vulkan1_1`]
- [`InputAttachmentAspectReference`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        