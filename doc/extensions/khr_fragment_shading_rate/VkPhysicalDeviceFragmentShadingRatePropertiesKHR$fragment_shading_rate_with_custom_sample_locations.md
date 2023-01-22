[`fragment_shading_rate_with_custom_sample_locations`]
specifies whether [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations)
are supported for multi-pixel fragments.
It  **must**  be [`FALSE`] if `[`VK_EXT_sample_locations`]` is not
supported.
If this value is [`FALSE`], using [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) will clamp the fragment shading rate to
(1,1).