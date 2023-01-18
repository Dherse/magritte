[VkPipelineCreationFeedback](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedback.html) - Feedback about the creation of a pipeline or pipeline stage

# C Specifications
The [`PipelineCreationFeedback`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPipelineCreationFeedback {
    VkPipelineCreationFeedbackFlags    flags;
    uint64_t                           duration;
} VkPipelineCreationFeedback;
```
or the equivalent
```c
// Provided by VK_EXT_pipeline_creation_feedback
typedef VkPipelineCreationFeedback VkPipelineCreationFeedbackEXT;
```

# Members
- [`flags`] is a bitmask of [`PipelineCreationFeedbackFlagBits`] providing feedback about the creation of a pipeline or of a pipeline stage.
- [`duration`] is the duration spent creating a pipeline or pipeline stage in nanoseconds.

# Description
If the `VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT` is not set in
[`flags`], an implementation  **must**  not set any other bits in [`flags`],
and the values of all other [`PipelineCreationFeedback`] data members
are undefined.

# Related
- [`VK_EXT_pipeline_creation_feedback`]
- [`crate::vulkan1_3`]
- [`PipelineCreationFeedbackCreateInfo`]
- [`PipelineCreationFeedbackFlagBits`]
- [`PipelineCreationFeedbackFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        