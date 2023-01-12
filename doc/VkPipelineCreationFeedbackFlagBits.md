[VkPipelineCreationFeedbackFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackFlagBits.html) - Bitmask specifying pipeline or pipeline stage creation feedback

# C Specifications
Possible values of the `flags` member of
[`PipelineCreationFeedback`] are:
```c
// Provided by VK_VERSION_1_3
typedef enum VkPipelineCreationFeedbackFlagBits {
    VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT = 0x00000001,
    VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT = 0x00000002,
    VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT = 0x00000004,
    VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT = VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT,
    VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT = VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT,
    VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT = VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT,
} VkPipelineCreationFeedbackFlagBits;
```
or the equivalent
```c
// Provided by VK_EXT_pipeline_creation_feedback
typedef VkPipelineCreationFeedbackFlagBits VkPipelineCreationFeedbackFlagBitsEXT;
```

# Description
- [`VK_PIPELINE_CREATION_FEEDBACK_FLAG_BITS`] indicates that the feedback information is valid.
- [`VK_PIPELINE_CREATION_FEEDBACK_FLAG_BITS`] indicates that a readily usable pipeline or pipeline stage was found in the `pipelineCache` specified by the application in the pipeline creation command.An implementation  **should**  set the [`VK_PIPELINE_CREATION_FEEDBACK_FLAG_BITS`] bit if it was able to avoid the large majority of pipeline or pipeline stage creation work by using the `pipelineCache` parameter of [`create_graphics_pipelines`], [`create_ray_tracing_pipelines_khr`], [`create_ray_tracing_pipelines_nv`], or [`create_compute_pipelines`]. When an implementation sets this bit for the entire pipeline, it  **may**  leave it unset for any stage.
- [`VK_PIPELINE_CREATION_FEEDBACK_FLAG_BITS`] indicates that the base pipeline specified by the `basePipelineHandle` or `basePipelineIndex` member of the `Vk*PipelineCreateInfo` structure was used to accelerate the creation of the pipeline.An implementation  **should**  set the [`VK_PIPELINE_CREATION_FEEDBACK_FLAG_BITS`] bit if it was able to avoid a significant amount of work by using the base pipeline.

# Related
- [`ext_pipeline_creation_feedback`]
- [`crate::vulkan1_3`]
- [`PipelineCreationFeedback`]
- [`PipelineCreationFeedbackCreateInfo`]
- [VkPipelineCreationFeedbackFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        