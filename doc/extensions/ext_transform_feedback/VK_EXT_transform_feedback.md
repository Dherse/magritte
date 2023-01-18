[VK_EXT_transform_feedback](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_transform_feedback.html) - device extension

# Description
This extension adds transform feedback to the Vulkan API by exposing the
SPIR-V `TransformFeedback` and `GeometryStreams` capabilities to
capture vertex, tessellation or geometry shader outputs to one or more
buffers.
It adds API functionality to bind transform feedback buffers to capture the
primitives emitted by the graphics pipeline from SPIR-V outputs decorated
for transform feedback.
The transform feedback capture can be paused and resumed by way of storing
and retrieving a byte counter.
The captured data can be drawn again where the vertex count is derived from
the byte counter without CPU intervention.
If the implementation is capable, a vertex stream other than zero can be
rasterized.All these features are designed to match the full capabilities of OpenGL
core transform feedback functionality and beyond.
Many of the features are optional to allow base OpenGL ES GPUs to also
implement this extension.The primary purpose of the functionality exposed by this extension is to
support translation layers from other 3D APIs.
This functionality is not considered forward looking, and is not expected to
be promoted to a KHR extension or to core Vulkan.
Unless this is needed for translation, it is recommended that developers use
alternative techniques of using the GPU to process and capture vertex data.

# Registered extension number
29

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_transform_feedback] @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_EXT_transform_feedback extension>>)

# New commands
- [`cmd_begin_query_indexed_ext`]
- [`cmd_begin_transform_feedback_ext`]
- [`cmd_bind_transform_feedback_buffers_ext`]
- [`cmd_draw_indirect_byte_count_ext`]
- [`cmd_end_query_indexed_ext`]
- [`cmd_end_transform_feedback_ext`]

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceTransformFeedbackFeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceTransformFeedbackPropertiesEXT`] 
- Extending [`PipelineRasterizationStateCreateInfo`]:  - [`PipelineRasterizationStateStreamCreateInfoEXT`]

# New bitmasks
- [`PipelineRasterizationStateStreamCreateFlagsEXT`]

# New constants
- [`EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME`]
- [`EXT_TRANSFORM_FEEDBACK_SPEC_VERSION`]
- Extending [`AccessFlagBits`]:  - `VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`  - `VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`  - `VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT` 
- Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT`  - `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT` 
- Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT` 
- Extending [`QueryType`]:  - `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`

# Known issues & F.A.Q.
1) Should we include pause/resume functionality? **RESOLVED** : Yes, this is needed to ease layering other APIs which have this
functionality.
To pause use [`cmd_end_transform_feedback_ext`] and provide valid buffer
handles in the `pCounterBuffers` array and offsets in the
`pCounterBufferOffsets` array for the implementation to save the resume
points.
Then to resume use [`cmd_begin_transform_feedback_ext`] with the previous
`pCounterBuffers` and `pCounterBufferOffsets` values.
Between the pause and resume there needs to be a memory barrier for the
counter buffers with a source access of
`VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT` at pipeline stage
`VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT` to a destination access
of `VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT` at pipeline stage
`VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`.2) How does this interact with multiview? **RESOLVED** : Transform feedback cannot be made active in a render pass with
multiview enabled.3) How should queries be done? **RESOLVED** : There is a new query type
`VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT`.
A query pool created with this type will capture 2 integers -
numPrimitivesWritten and numPrimitivesNeeded - for the specified vertex
stream output from the last
[pre-rasterization shader
stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
The vertex stream output queried is zero by default, but can be specified
with the new [`cmd_begin_query_indexed_ext`] and
[`cmd_end_query_indexed_ext`] commands.

# Version history
- Revision 1, 2018-10-09 (Piers Daniell)  - Internal revisions

# Other information
* 2018-10-09
*   - Baldur Karlsson, Valve  - Boris Zanin, Mobica  - Daniel Rakos, AMD  - Donald Scorgie, Imagination  - Henri Verbeet, CodeWeavers  - Jan-Harald Fredriksen, Arm  - Jason Ekstrand, Intel  - Jeff Bolz, NVIDIA  - Jesse Barker, Unity  - Jesse Hall, Google  - Pierre-Loup Griffais, Valve  - Philip Rebohle, DXVK  - Ruihao Zhang, Qualcomm  - Samuel Pitoiset, Valve  - Slawomir Grajewski, Intel  - Stu Smith, Imagination Technologies

# Related
- [`PhysicalDeviceTransformFeedbackFeaturesEXT`]
- [`PhysicalDeviceTransformFeedbackPropertiesEXT`]
- [`PipelineRasterizationStateStreamCreateFlagsEXT`]
- [`PipelineRasterizationStateStreamCreateInfoEXT`]
- [`cmd_begin_query_indexed_ext`]
- [`cmd_begin_transform_feedback_ext`]
- [`cmd_bind_transform_feedback_buffers_ext`]
- [`cmd_draw_indirect_byte_count_ext`]
- [`cmd_end_query_indexed_ext`]
- [`cmd_end_transform_feedback_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        