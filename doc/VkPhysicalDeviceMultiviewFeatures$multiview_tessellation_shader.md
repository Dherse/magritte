[`multiview_tessellation_shader`] specifies whether the implementation
supports multiview rendering within a render pass, with
[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation).
If this feature is not enabled, then a pipeline compiled against a
subpass with a non-zero view mask  **must**  not include any tessellation
shaders.