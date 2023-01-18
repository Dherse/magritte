[`APPLICATION_PIPELINE_CACHE_HIT`]
indicates that a readily usable pipeline or pipeline stage was found in
the `pipelineCache` specified by the application in the pipeline
creation command.An implementation  **should**  set the
[`APPLICATION_PIPELINE_CACHE_HIT`] bit
if it was able to avoid the large majority of pipeline or pipeline stage
creation work by using the `pipelineCache` parameter of
[`create_graphics_pipelines`],
[`create_ray_tracing_pipelines_khr`],
[`create_ray_tracing_pipelines_nv`],
or [`create_compute_pipelines`].
When an implementation sets this bit for the entire pipeline, it  **may**  leave
it unset for any stage.