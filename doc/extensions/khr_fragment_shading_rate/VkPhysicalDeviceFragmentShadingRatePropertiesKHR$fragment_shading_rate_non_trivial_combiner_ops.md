[`fragment_shading_rate_non_trivial_combiner_ops`] specifies whether
[`FragmentShadingRateCombinerOpKHR`] enums other than
`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or
`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR` **can**  be used.
It  **must**  be [`FALSE`] unless either the
[`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) or
[`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is supported.