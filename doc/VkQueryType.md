[VkQueryType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryType.html) - Specify the type of queries managed by a query pool

# C Specifications
Possible values of [`QueryPoolCreateInfo::query_type`], specifying
the type of queries managed by the pool, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkQueryType {
    VK_QUERY_TYPE_OCCLUSION = 0,
    VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
    VK_QUERY_TYPE_TIMESTAMP = 2,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_queue
    VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR = 1000023000,
#endif
  // Provided by VK_EXT_transform_feedback
    VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT = 1000028004,
  // Provided by VK_KHR_performance_query
    VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR = 1000116000,
  // Provided by VK_KHR_acceleration_structure
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR = 1000150000,
  // Provided by VK_KHR_acceleration_structure
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR = 1000150001,
  // Provided by VK_NV_ray_tracing
    VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV = 1000165000,
  // Provided by VK_INTEL_performance_query
    VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL = 1000210000,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_QUERY_TYPE_VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR = 1000299000,
#endif
} VkQueryType;
```

# Description
- [`OCCLUSION`] specifies an [occlusion query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-occlusion).
- [`PIPELINE_STATISTICS`] specifies a [pipeline statistics query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-pipestats).
- [`TIMESTAMP`] specifies a [timestamp query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-timestamps).
- [`PERFORMANCE_QUERY_KHR`] specifies a [performance query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-performance).
- [`TRANSFORM_FEEDBACK_STREAM_EXT`] specifies a [transform feedback query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-transform-feedback).
- [`ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR`] specifies a [acceleration structure size query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-copying) for use with [`cmd_write_acceleration_structures_properties_khr`] or [`write_acceleration_structures_properties_khr`].
- [`ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`] specifies a [serialization acceleration structure size query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-copying)
- [`ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`] specifies a [acceleration structure size query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-copying) for use with [`cmd_write_acceleration_structures_properties_nv`].
- [`PERFORMANCE_QUERY_INTEL`] specifies a [Intel performance query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-performance-intel).
- [`RESULT_STATUS_ONLY_KHR`] specifies a [result status query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-result-status-only).
- [`VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR`] specifies a [video encode bitstream range query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-video-encode-bitstream-buffer-range).

# Related
- [`crate::vulkan1_0`]
- [`QueryPoolCreateInfo`]
- [`cmd_write_acceleration_structures_properties_khr`]
- [`cmd_write_acceleration_structures_properties_nv`]
- [`write_acceleration_structures_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        