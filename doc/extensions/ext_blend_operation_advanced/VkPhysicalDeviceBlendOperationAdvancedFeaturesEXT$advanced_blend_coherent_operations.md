[`advanced_blend_coherent_operations`] specifies whether blending using
[advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced) is guaranteed
to execute atomically and in [primitive
order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
If this is [`TRUE`],
`VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is treated the
same as `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, and advanced blending
needs no additional synchronization over basic blending.
If this is [`FALSE`], then memory dependencies are required to
guarantee order between two advanced blending operations that occur on
the same sample.