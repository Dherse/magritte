[VkSubpassDescriptionFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionFlagBits.html) - Bitmask specifying usage of a subpass

# C Specifications
Bits which  **can**  be set in [`SubpassDescription::flags`],
specifying usage of the subpass, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSubpassDescriptionFlagBits {
  // Provided by VK_NVX_multiview_per_view_attributes
    VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX = 0x00000001,
  // Provided by VK_NVX_multiview_per_view_attributes
    VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX = 0x00000002,
  // Provided by VK_QCOM_render_pass_shader_resolve
    VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM = 0x00000004,
  // Provided by VK_QCOM_render_pass_shader_resolve
    VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM = 0x00000008,
  // Provided by VK_ARM_rasterization_order_attachment_access
    VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM = 0x00000010,
  // Provided by VK_ARM_rasterization_order_attachment_access
    VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM = 0x00000020,
  // Provided by VK_ARM_rasterization_order_attachment_access
    VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM = 0x00000040,
} VkSubpassDescriptionFlagBits;
```

# Description
- [`PER_VIEW_ATTRIBUTES_NVX`] specifies that shaders compiled for this subpass write the attributes for all views in a single invocation of each [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization). All pipelines compiled against a subpass that includes this bit  **must**  write per-view attributes to the `*PerViewNV[]` shader outputs, in addition to the non-per-view (e.g. `Position`) outputs.
- [`PER_VIEW_POSITION_X_ONLY_NVX`] specifies that shaders compiled for this subpass use per-view positions which only differ in value in the x component. Per-view viewport mask  **can**  also be used.
- [`FRAGMENT_REGION_QCOM`] specifies that the framebuffer region is the fragment region, that is, the minimum region dependencies are by pixel rather than by sample, such that any fragment shader invocation  **can**  access any sample associated with that fragment shader invocation.
- [`SHADER_RESOLVE_QCOM`] specifies that the subpass performs shader resolve operations.
- [`RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM`] specifies that this subpass supports pipelines created with `VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM`.
- [`RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM`] specifies that this subpass supports pipelines created with `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`.
- [`RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM`] specifies that this subpass supports pipelines created with `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`.

# Related
- [`crate::vulkan1_0`]
- [`SubpassDescriptionFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        