[`multiview_tessellation_shader`] specifies whether the implementation
supports multiview rendering within a render pass, with
[tessellation shaders]().
If this feature is not enabled, then a pipeline compiled against a
subpass with a non-zero view mask  **must**  not include any tessellation
shaders.