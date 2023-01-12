[`CAPTURE_STATISTICS_KHR`] specifies that the
shader compiler should capture statistics for the pipeline executables
produced by the compile process which  **can**  later be retrieved by calling
[`get_pipeline_executable_statistics_khr`].
Enabling this flag  **must**  not affect the final compiled pipeline but  **may** 
disable pipeline caching or otherwise affect pipeline creation time.