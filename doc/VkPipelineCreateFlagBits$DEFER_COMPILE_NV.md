[`DEFER_COMPILE_NV`] specifies that a pipeline
is created with all shaders in the deferred state.
Before using the pipeline the application  **must**  call
[`compile_deferred_nv`] exactly once on each shader in the pipeline
before using the pipeline.