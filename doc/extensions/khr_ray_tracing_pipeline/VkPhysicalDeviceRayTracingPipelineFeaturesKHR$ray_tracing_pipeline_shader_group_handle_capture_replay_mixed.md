[`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] indicates
whether the implementation supports reuse of shader group handles being
arbitrarily mixed with creation of non-reused shader group handles.
If this is `VK_FALSE`, all reused shader group handles  **must**  be
specified before any non-reused handles  **may**  be created.