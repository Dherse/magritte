[`dual_src_blend`] specifies whether blend
operations which take two sources are supported.
If this feature is not enabled, the `VK_BLEND_FACTOR_SRC1_COLOR`,
`VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`,
`VK_BLEND_FACTOR_SRC1_ALPHA`, and
`VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA` enum values  **must**  not be used
as source or destination blending factors.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-dsb](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-dsb).