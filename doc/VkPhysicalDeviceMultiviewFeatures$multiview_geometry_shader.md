[`multiview_geometry_shader`]
specifies whether the implementation supports multiview rendering within
a render pass, with [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry).
If this feature is not enabled, then a pipeline compiled against a
subpass with a non-zero view mask  **must**  not include a geometry shader.