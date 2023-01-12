[`CAPTURE_INTERNAL_REPRESENTATIONS_KHR`]
specifies that the shader compiler should capture the internal
representations of pipeline executables produced by the compile process
which  **can**  later be retrieved by calling
[`get_pipeline_executable_internal_representations_khr`].
Enabling this flag  **must**  not affect the final compiled pipeline but  **may** 
disable pipeline caching or otherwise affect pipeline creation time.