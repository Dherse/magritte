[VkPipelineRasterizationStateStreamCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html) - Structure defining the geometry stream used for rasterization

# C Specifications
The vertex stream used for rasterization is specified by adding a
[`PipelineRasterizationStateStreamCreateInfoEXT`] structure to the
[`p_next`] chain of a [`PipelineRasterizationStateCreateInfo`]
structure.The [`PipelineRasterizationStateStreamCreateInfoEXT`] structure is
defined as:
```c
// Provided by VK_EXT_transform_feedback
typedef struct VkPipelineRasterizationStateStreamCreateInfoEXT {
    VkStructureType                                     sType;
    const void*                                         pNext;
    VkPipelineRasterizationStateStreamCreateFlagsEXT    flags;
    uint32_t                                            rasterizationStream;
} VkPipelineRasterizationStateStreamCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`rasterization_stream`] is the vertex stream selected for rasterization.

# Description
If this structure is not present, [`rasterization_stream`] is assumed to be
zero.
## Valid Usage
-  [`PhysicalDeviceTransformFeedbackFeaturesEXT::geometry_streams`] **must**  be enabled
-  [`rasterization_stream`] **must**  be less than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
-  [`rasterization_stream`] **must**  be zero if [`PhysicalDeviceTransformFeedbackPropertiesEXT::transform_feedback_rasterization_stream_select`] is `VK_FALSE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`
-  [`flags`] **must**  be `0`

# Related
- [`ext_transform_feedback`]
- [`PipelineRasterizationStateStreamCreateFlagsEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        