[`primitive_fragment_shading_rate_with_multiple_viewports`] specifies
    whether the [primitive
    fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive) **can**  be used when multiple viewports are used.
    If this value is `VK_FALSE`, only a single viewport  **must**  be used,
    and applications  **must**  not write to the
    `ViewportMaskNV` or
    `ViewportIndex` built-in when setting `PrimitiveShadingRateKHR`.
    It  **must**  be `VK_FALSE` if
    the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,
    the `[`ext_shader_viewport_index_layer`]` extension,
or
    the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not
    supported, or if the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) feature is not supported.