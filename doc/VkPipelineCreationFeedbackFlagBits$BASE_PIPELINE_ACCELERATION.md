[`BASE_PIPELINE_ACCELERATION`]
indicates that the base pipeline specified by the
`basePipelineHandle` or `basePipelineIndex` member of the
`Vk*PipelineCreateInfo` structure was used to accelerate the
creation of the pipeline.An implementation  **should**  set the
[`BASE_PIPELINE_ACCELERATION`] bit if it
was able to avoid a significant amount of work by using the base pipeline.