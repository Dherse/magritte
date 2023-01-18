[VkPipelineCreationFeedbackCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackCreateInfo.html) - Request for feedback about the creation of a pipeline

# C Specifications
Feedback about the creation of a particular pipeline object  **can**  be obtained
by adding a [`PipelineCreationFeedbackCreateInfo`] structure to the
[`p_next`] chain of [`GraphicsPipelineCreateInfo`],
[`RayTracingPipelineCreateInfoKHR`],
[`RayTracingPipelineCreateInfoNV`],
or [`ComputePipelineCreateInfo`].
The [`PipelineCreationFeedbackCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_3
typedef struct VkPipelineCreationFeedbackCreateInfo {
    VkStructureType                sType;
    const void*                    pNext;
    VkPipelineCreationFeedback*    pPipelineCreationFeedback;
    uint32_t                       pipelineStageCreationFeedbackCount;
    VkPipelineCreationFeedback*    pPipelineStageCreationFeedbacks;
} VkPipelineCreationFeedbackCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_pipeline_creation_feedback
typedef VkPipelineCreationFeedbackCreateInfo VkPipelineCreationFeedbackCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`pipeline_creation_feedback`] is a pointer to a [`PipelineCreationFeedback`] structure.
- [`pipeline_stage_creation_feedback_count`] is the number of elements in [`pipeline_stage_creation_feedbacks`].
- [`pipeline_stage_creation_feedbacks`] is a pointer to an array of [`pipeline_stage_creation_feedback_count`][`PipelineCreationFeedback`] structures.

# Description
An implementation  **should**  write pipeline creation feedback to
[`pipeline_creation_feedback`] and  **may**  write pipeline stage creation
feedback to [`pipeline_stage_creation_feedbacks`].
An implementation  **must**  set or clear the
`VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT` in
[`PipelineCreationFeedback::flags`] for
[`pipeline_creation_feedback`] and every element of
[`pipeline_stage_creation_feedbacks`].When chained to
[`RayTracingPipelineCreateInfoKHR`],
[`RayTracingPipelineCreateInfoNV`],
or
[`GraphicsPipelineCreateInfo`], the `i` element of
[`pipeline_stage_creation_feedbacks`] corresponds to the `i` element of
[`RayTracingPipelineCreateInfoKHR::stages`],
[`RayTracingPipelineCreateInfoNV::stages`],
or
[`GraphicsPipelineCreateInfo::stages`].
When chained to [`ComputePipelineCreateInfo`], the first element of
[`pipeline_stage_creation_feedbacks`] corresponds to
[`ComputePipelineCreateInfo::stage`].
## Valid Usage
-    When chained to [`GraphicsPipelineCreateInfo`], [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`] **must**  equal [`GraphicsPipelineCreateInfo::stage_count`]
-    When chained to [`ComputePipelineCreateInfo`], [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`] **must**  equal 1
-    When chained to [`RayTracingPipelineCreateInfoKHR`], [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`] **must**  equal [`RayTracingPipelineCreateInfoKHR::stage_count`]
-    When chained to [`RayTracingPipelineCreateInfoNV`], [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`] **must**  equal [`RayTracingPipelineCreateInfoNV::stage_count`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO`
-  [`pipeline_creation_feedback`] **must**  be a valid pointer to a [`PipelineCreationFeedback`] structure
-  [`pipeline_stage_creation_feedbacks`] **must**  be a valid pointer to an array of [`pipeline_stage_creation_feedback_count`][`PipelineCreationFeedback`] structures
-  [`pipeline_stage_creation_feedback_count`] **must**  be greater than `0`

# Related
- [`VK_EXT_pipeline_creation_feedback`]
- [`crate::vulkan1_3`]
- [`ComputePipelineCreateInfo`]
- [`GraphicsPipelineCreateInfo`]
- [`PipelineCreationFeedback`]
- [`RayTracingPipelineCreateInfoKHR`]
- [`RayTracingPipelineCreateInfoNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        