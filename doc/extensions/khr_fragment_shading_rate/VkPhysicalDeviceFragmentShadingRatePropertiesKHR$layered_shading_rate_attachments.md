[`layered_shading_rate_attachments`] specifies whether a shading rate
    attachment image view  **can**  be created with multiple layers.
    If this value is `VK_FALSE`, when creating an image view with a
    `usage` that includes
    `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`,
    `layerCount` **must**  be `1`.
    It  **must**  be `VK_FALSE` if
    the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature,
    the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,
    the `[`ext_shader_viewport_index_layer`]` extension,
or
    the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not
    supported, or if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.