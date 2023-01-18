[`fragment_shading_rate_with_conservative_rasterization`]
specifies whether [conservative
rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) is supported for multi-pixel fragments.
It  **must**  be [`FALSE`] if `[`VK_EXT_conservative_rasterization`]`
is not supported.
If this value is [`FALSE`], using [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) will clamp the fragment shading rate to
(1,1).