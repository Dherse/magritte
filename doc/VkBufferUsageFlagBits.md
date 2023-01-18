[VkBufferUsageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferUsageFlagBits.html) - Bitmask specifying allowed usage of a buffer

# C Specifications
Bits which  **can**  be set in [`BufferCreateInfo::usage`], specifying
usage behavior of a buffer, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkBufferUsageFlagBits {
    VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
    VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
    VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
    VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
    VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
    VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
    VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
    VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,
  // Provided by VK_VERSION_1_2
    VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT = 0x00020000,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_BUFFER_USAGE_VIDEO_DECODE_SRC_BIT_KHR = 0x00002000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_decode_queue
    VK_BUFFER_USAGE_VIDEO_DECODE_DST_BIT_KHR = 0x00004000,
#endif
  // Provided by VK_EXT_transform_feedback
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 0x00000800,
  // Provided by VK_EXT_transform_feedback
    VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 0x00001000,
  // Provided by VK_EXT_conditional_rendering
    VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00000200,
  // Provided by VK_KHR_acceleration_structure
    VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR = 0x00080000,
  // Provided by VK_KHR_acceleration_structure
    VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR = 0x00100000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR = 0x00000400,
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_BUFFER_USAGE_VIDEO_ENCODE_DST_BIT_KHR = 0x00008000,
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
  // Provided by VK_KHR_video_encode_queue
    VK_BUFFER_USAGE_VIDEO_ENCODE_SRC_BIT_KHR = 0x00010000,
#endif
  // Provided by VK_NV_ray_tracing
    VK_BUFFER_USAGE_RAY_TRACING_BIT_NV = VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR,
  // Provided by VK_EXT_buffer_device_address
    VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT = VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT,
  // Provided by VK_KHR_buffer_device_address
    VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR = VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT,
} VkBufferUsageFlagBits;
```

# Description
- [`TRANSFER_SRC`] specifies that the buffer  **can**  be used as the source of a *transfer command* (see the definition of [`VK_PIPELINE_STAGE_TRANSFER_BIT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-transfer)).
- [`TRANSFER_DST`] specifies that the buffer  **can**  be used as the destination of a transfer command.
- [`UNIFORM_TEXEL_BUFFER`] specifies that the buffer  **can**  be used to create a [`BufferView`] suitable for occupying a [`DescriptorSet`] slot of type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
- [`STORAGE_TEXEL_BUFFER`] specifies that the buffer  **can**  be used to create a [`BufferView`] suitable for occupying a [`DescriptorSet`] slot of type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
- [`UNIFORM_BUFFER`] specifies that the buffer  **can**  be used in a [`DescriptorBufferInfo`] suitable for occupying a [`DescriptorSet`] slot either of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`.
- [`STORAGE_BUFFER`] specifies that the buffer  **can**  be used in a [`DescriptorBufferInfo`] suitable for occupying a [`DescriptorSet`] slot either of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`.
- [`INDEX_BUFFER`] specifies that the buffer is suitable for passing as the `buffer` parameter to [`cmd_bind_index_buffer`].
- [`VERTEX_BUFFER`] specifies that the buffer is suitable for passing as an element of the `pBuffers` array to [`cmd_bind_vertex_buffers`].
- [`INDIRECT_BUFFER`] specifies that the buffer is suitable for passing as the `buffer` parameter to [`cmd_draw_indirect`], [`cmd_draw_indexed_indirect`], [`cmd_draw_mesh_tasks_indirect_nv`], [`cmd_draw_mesh_tasks_indirect_count_nv`], or [`cmd_dispatch_indirect`]. It is also suitable for passing as the `buffer` member of [`IndirectCommandsStreamNV`], or `sequencesCountBuffer` or `sequencesIndexBuffer` or `preprocessedBuffer` member of [`GeneratedCommandsInfoNV`]
- [`CONDITIONAL_RENDERING_EXT`] specifies that the buffer is suitable for passing as the `buffer` parameter to [`cmd_begin_conditional_rendering_ext`].
- [`TRANSFORM_FEEDBACK_BUFFER_EXT`] specifies that the buffer is suitable for using for binding as a transform feedback buffer with [`cmd_bind_transform_feedback_buffers_ext`].
- [`TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT`] specifies that the buffer is suitable for using as a counter buffer with [`cmd_begin_transform_feedback_ext`] and [`cmd_end_transform_feedback_ext`].
- [`RAY_TRACING_NV`] specifies that the buffer is suitable for use in [`cmd_trace_rays_nv`].
- [`SHADER_BINDING_TABLE_KHR`] specifies that the buffer is suitable for use as a [Shader Binding Table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-binding-table).
- [`ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR`] specifies that the buffer is suitable for use as a read-only input to an [acceleration structure build](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-building).
- [`ACCELERATION_STRUCTURE_STORAGE_KHR`] specifies that the buffer is suitable for storage space for a [`AccelerationStructureKHR`].
- [`SHADER_DEVICE_ADDRESS`] specifies that the buffer  **can**  be used to retrieve a buffer device address via [`get_buffer_device_address`] and use that address to access the buffer’s memory from a shader.
- [`VIDEO_DECODE_SRC_KHR`] specifies that the buffer  **can**  be used as the source bitstream buffer in a [video decode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations).
- [`VIDEO_DECODE_DST_KHR`] specifies that the buffer  **can**  be used as the destination status buffer in a [video decode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations).
- [`VIDEO_ENCODE_DST_KHR`] specifies that the buffer  **can**  be used as the destination bitstream buffer in a [video encode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations).
- [`VIDEO_ENCODE_DST_KHR`] specifies that the buffer  **can**  be used as the destination status buffer in a [video encode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations).

# Related
- [`crate::vulkan1_0`]
- [`BufferUsageFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        