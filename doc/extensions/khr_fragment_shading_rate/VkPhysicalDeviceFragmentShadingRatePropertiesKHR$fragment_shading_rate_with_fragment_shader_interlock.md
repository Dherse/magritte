[`fragment_shading_rate_with_fragment_shader_interlock`]
specifies whether [fragment shader
interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) is supported for multi-pixel fragments.
It  **must**  be [`FALSE`] if `[`VK_EXT_fragment_shader_interlock`]`
is not supported.
If this value is [`FALSE`], using [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) will clamp the fragment shading rate to
(1,1).