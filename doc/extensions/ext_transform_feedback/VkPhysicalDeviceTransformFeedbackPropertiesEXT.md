[VkPhysicalDeviceTransformFeedbackPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html) - Structure describing transform feedback properties that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceTransformFeedbackPropertiesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_transform_feedback
typedef struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           maxTransformFeedbackStreams;
    uint32_t           maxTransformFeedbackBuffers;
    VkDeviceSize       maxTransformFeedbackBufferSize;
    uint32_t           maxTransformFeedbackStreamDataSize;
    uint32_t           maxTransformFeedbackBufferDataSize;
    uint32_t           maxTransformFeedbackBufferDataStride;
    VkBool32           transformFeedbackQueries;
    VkBool32           transformFeedbackStreamsLinesTriangles;
    VkBool32           transformFeedbackRasterizationStreamSelect;
    VkBool32           transformFeedbackDraw;
} VkPhysicalDeviceTransformFeedbackPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_transform_feedback_streams`] is the maximum number of vertex streams that can be output from geometry shaders declared with the `GeometryStreams` capability. If the implementation does not support [`PhysicalDeviceTransformFeedbackFeaturesEXT::geometry_streams`] then [`max_transform_feedback_streams`] **must**  be set to `1`.
- [`max_transform_feedback_buffers`] is the maximum number of transform feedback buffers that can be bound for capturing shader outputs from the last [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
- [`max_transform_feedback_buffer_size`] is the maximum size that can be specified when binding a buffer for transform feedback in [`cmd_bind_transform_feedback_buffers_ext`].
- [`max_transform_feedback_stream_data_size`] is the maximum amount of data in bytes for each vertex that captured to one or more transform feedback buffers associated with a specific vertex stream.
- [`max_transform_feedback_buffer_data_size`] is the maximum amount of data in bytes for each vertex that can be captured to a specific transform feedback buffer.
- [`max_transform_feedback_buffer_data_stride`] is the maximum stride between each capture of vertex data to the buffer.
- [`transform_feedback_queries`] is [`TRUE`] if the implementation supports the `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` query type. [`transform_feedback_queries`] is [`FALSE`] if queries of this type  **cannot**  be created.
- [`transform_feedback_streams_lines_triangles`] is [`TRUE`] if the implementation supports the geometry shader `OpExecutionMode` of `OutputLineStrip` and `OutputTriangleStrip` in addition to `OutputPoints` when more than one vertex stream is output. If [`transform_feedback_streams_lines_triangles`] is [`FALSE`] the implementation only supports an `OpExecutionMode` of `OutputPoints` when more than one vertex stream is output from the geometry shader.
- [`transform_feedback_rasterization_stream_select`] is [`TRUE`] if the implementation supports the `GeometryStreams` SPIR-V capability and the application can use [`PipelineRasterizationStateStreamCreateInfoEXT`] to modify which vertex stream output is used for rasterization. Otherwise vertex stream `0` **must**  always be used for rasterization.
- [`transform_feedback_draw`] is [`TRUE`] if the implementation supports the [`cmd_draw_indirect_byte_count_ext`] function otherwise the function  **must**  not be called.

# Description
If the [`PhysicalDeviceTransformFeedbackPropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`

# Related
- [`VK_EXT_transform_feedback`]
- [`Bool32`]
- [`DeviceSize`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        