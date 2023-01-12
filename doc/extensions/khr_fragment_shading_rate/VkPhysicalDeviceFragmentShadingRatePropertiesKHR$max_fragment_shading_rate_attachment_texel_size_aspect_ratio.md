[`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] indicates the
maximum ratio between the width and height of the portion of the
framebuffer corresponding to each texel in a fragment shading rate
attachment.
[`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] **must**  be a
power-of-two value, and  **must**  be less than or equal to
max(`maxFragmentShadingRateAttachmentTexelSize.width` /
`minFragmentShadingRateAttachmentTexelSize.height`,
`maxFragmentShadingRateAttachmentTexelSize.height` /
`minFragmentShadingRateAttachmentTexelSize.width`).
It  **must**  be 0 if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.