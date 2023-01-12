[`max_fragment_shading_rate_attachment_texel_size`] indicates maximum
supported width and height of the portion of the framebuffer
corresponding to each texel in a fragment shading rate attachment.
Each value  **must**  be greater than or equal to the values in
[`min_fragment_shading_rate_attachment_texel_size`].
Each value  **must**  be a power-of-two.
It  **must**  be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.