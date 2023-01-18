[VK_KHR_create_renderpass2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_create_renderpass2.html) - device extension

# Description
This extension provides a new entry point to create render passes in a way
that can be easily extended by other extensions through the substructures of
render pass creation.
The Vulkan 1.0 render pass creation sub-structures do not include
`sType`/`pNext` members.
Additionally, the render pass begin/next/end commands have been augmented
with new extensible structures for passing additional subpass information.The [`RenderPassMultiviewCreateInfo`] and
[`InputAttachmentAspectReference`] structures that extended the original
[`RenderPassCreateInfo`] are not accepted into the new creation
functions, and instead their parameters are folded into this extension as
follows:
- Elements of [`RenderPassMultiviewCreateInfo::view_masks`] are now specified in [`SubpassDescription2KHR::view_mask`].
- Elements of [`RenderPassMultiviewCreateInfo::view_offsets`] are now specified in [`SubpassDependency2KHR::view_offset`].
- [`RenderPassMultiviewCreateInfo::correlation_mask_count`] and [`RenderPassMultiviewCreateInfo::correlation_masks`] are directly specified in [`RenderPassCreateInfo2KHR`].
- [`InputAttachmentAspectReference::aspect_mask`] is now specified in the relevant input attachment description in [`AttachmentDescription2KHR`]`::aspectMask`
The details of these mappings are explained fully in the new structures.

# Registered extension number
110

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_multiview`]`
- Requires `[`VK_KHR_maintenance2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)

# Contacts
- Tobias Hector [tobias](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_create_renderpass2] @tobias%0A<<Here describe the issue or question you have about the VK_KHR_create_renderpass2 extension>>)

# New commands
- [`cmd_begin_render_pass2_khr`]
- [`cmd_end_render_pass2_khr`]
- [`cmd_next_subpass2_khr`]
- [`create_render_pass2_khr`]

# New structures
- [`AttachmentDescription2KHR`]
- [`AttachmentReference2KHR`]
- [`RenderPassCreateInfo2KHR`]
- [`SubpassBeginInfoKHR`]
- [`SubpassDependency2KHR`]
- [`SubpassDescription2KHR`]
- [`SubpassEndInfoKHR`]

# New constants
- [`KHR_CREATE_RENDERPASS_2_EXTENSION_NAME`]
- [`KHR_CREATE_RENDERPASS_2_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR`  - `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR`  - `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR`  - `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR`  - `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR`  - `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR`  - `VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR`

# Version history
- Revision 1, 2018-02-07 (Tobias Hector)  - Internal revisions

# Other information
* 2018-02-07
*   - Promoted to Vulkan 1.2 Core 
*   - Tobias Hector  - Jeff Bolz

# Related
- [`AttachmentDescription2KHR`]
- [`AttachmentReference2KHR`]
- [`RenderPassCreateInfo2KHR`]
- [`SubpassBeginInfoKHR`]
- [`SubpassDependency2KHR`]
- [`SubpassDescription2KHR`]
- [`SubpassEndInfoKHR`]
- [`cmd_begin_render_pass2_khr`]
- [`cmd_end_render_pass2_khr`]
- [`cmd_next_subpass2_khr`]
- [`create_render_pass2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        