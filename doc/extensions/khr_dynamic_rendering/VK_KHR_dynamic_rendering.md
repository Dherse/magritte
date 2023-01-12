[VK_KHR_dynamic_rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering.html) - device extension

# Description
This extension allows applications to create single-pass render pass
instances without needing to create render pass objects or framebuffers.
Dynamic render passes can also span across multiple primary command buffers,
rather than relying on secondary command buffers.This extension also incorporates `VK_ATTACHMENT_STORE_OP_NONE_KHR` from
[`qcom_render_pass_store_ops`], enabling
applications to avoid unnecessary synchronization when an attachment is not
written during a render pass.

# Registered extension number
45

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Deprecation state
- *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)

# Contacts
- Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_dynamic_rendering] @tobski%0A<<Here describe the issue or question you have about the VK_KHR_dynamic_rendering extension>>)

# New commands
- [`cmd_begin_rendering_khr`]
- [`cmd_end_rendering_khr`]

# New structures
- [`RenderingAttachmentInfoKHR`]
- [`RenderingInfoKHR`]
- Extending [`CommandBufferInheritanceInfo`]:  - [`CommandBufferInheritanceRenderingInfoKHR`] 
- Extending [`GraphicsPipelineCreateInfo`]:  - [`PipelineRenderingCreateInfoKHR`] 
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceDynamicRenderingFeaturesKHR`] 
If [`amd_mixed_attachment_samples`] is supported:
- Extending [`CommandBufferInheritanceInfo`], [`GraphicsPipelineCreateInfo`]:  - [`AttachmentSampleCountInfoAMD`] 
If [`ext_fragment_density_map`] is supported:
- Extending [`RenderingInfo`]:  - [`RenderingFragmentDensityMapAttachmentInfoEXT`] 
If [`khr_fragment_shading_rate`] is supported:
- Extending [`RenderingInfo`]:  - [`RenderingFragmentShadingRateAttachmentInfoKHR`] 
If [`nv_framebuffer_mixed_samples`] is supported:
- Extending [`CommandBufferInheritanceInfo`], [`GraphicsPipelineCreateInfo`]:  - [`AttachmentSampleCountInfoNV`] 
If [`nvx_multiview_per_view_attributes`] is supported:
- Extending [`CommandBufferInheritanceInfo`], [`GraphicsPipelineCreateInfo`], [`RenderingInfo`]:  - [`MultiviewPerViewAttributesInfoNVX`]

# New enums
- [`RenderingFlagBitsKHR`]

# New bitmasks
- [`RenderingFlagsKHR`]

# New constants
- `VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME`
- `VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION`
- Extending [`AttachmentStoreOp`]:  - `VK_ATTACHMENT_STORE_OP_NONE_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR`  - `VK_STRUCTURE_TYPE_RENDERING_INFO_KHR` 
If [`amd_mixed_attachment_samples`] is supported:
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD` 
If [`ext_fragment_density_map`] is supported:
- Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`  - `VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT` 
If [`khr_fragment_shading_rate`] is supported:
- Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`  - `VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR` 
If [`nv_framebuffer_mixed_samples`] is supported:
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_NV` 
If [`nvx_multiview_per_view_attributes`] is supported:
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX`

# Version history
- Revision 1, 2021-10-06 (Tobias Hector)  - Initial revision

# Other information
* 2021-10-06
*   - Promoted to Vulkan 1.3 Core 
*   - Tobias Hector, AMD  - Arseny Kapoulkine, Roblox  - François Duranleau, Gameloft  - Stuart Smith, AMD  - Hai Nguyen, Google  - Jean-François Roy, Google  - Jeff Leger, Qualcomm  - Jan-Harald Fredriksen, Arm  - Piers Daniell, Nvidia  - James Fitzpatrick, Imagination  - Piotr Byszewski, Mobica  - Jesse Hall, Google  - Mike Blumenkrantz, Valve

# Related
- [`CommandBufferInheritanceRenderingInfoKHR`]
- [`PhysicalDeviceDynamicRenderingFeaturesKHR`]
- [`PipelineRenderingCreateInfoKHR`]
- [`RenderingAttachmentInfoKHR`]
- [`RenderingFlagBitsKHR`]
- [`RenderingFlagsKHR`]
- [`RenderingInfoKHR`]
- [`cmd_begin_rendering_khr`]
- [`cmd_end_rendering_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        