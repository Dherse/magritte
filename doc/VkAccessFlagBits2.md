[VkAccessFlagBits2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits2.html) - Access flags for VkAccessFlags2

# C Specifications
Bits which  **can**  be set in the `srcAccessMask` and `dstAccessMask`
members of [`MemoryBarrier2KHR`], [`ImageMemoryBarrier2KHR`], and
[`BufferMemoryBarrier2KHR`], specifying access behavior, are:
```c
// Provided by VK_VERSION_1_3
// Flag bits for VkAccessFlagBits2
typedef VkFlags64 VkAccessFlagBits2;
static const VkAccessFlagBits2 VK_ACCESS_2_NONE = 0ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_NONE_KHR = 0ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT = 0x00000001ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR = 0x00000001ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_INDEX_READ_BIT = 0x00000002ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_INDEX_READ_BIT_KHR = 0x00000002ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR = 0x00000004ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_UNIFORM_READ_BIT = 0x00000008ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_UNIFORM_READ_BIT_KHR = 0x00000008ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT = 0x00000010ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR = 0x00000010ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_READ_BIT = 0x00000020ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_READ_BIT_KHR = 0x00000020ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_WRITE_BIT = 0x00000040ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_WRITE_BIT_KHR = 0x00000040ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT = 0x00000080ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR = 0x00000080ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR = 0x00000100ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR = 0x00000200ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR = 0x00000400ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_TRANSFER_READ_BIT = 0x00000800ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_TRANSFER_READ_BIT_KHR = 0x00000800ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_TRANSFER_WRITE_BIT = 0x00001000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR = 0x00001000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_HOST_READ_BIT = 0x00002000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_HOST_READ_BIT_KHR = 0x00002000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_HOST_WRITE_BIT = 0x00004000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_HOST_WRITE_BIT_KHR = 0x00004000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_MEMORY_READ_BIT = 0x00008000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_MEMORY_READ_BIT_KHR = 0x00008000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_MEMORY_WRITE_BIT = 0x00010000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_MEMORY_WRITE_BIT_KHR = 0x00010000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_SAMPLED_READ_BIT = 0x100000000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR = 0x100000000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_STORAGE_READ_BIT = 0x200000000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR = 0x200000000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT = 0x400000000ULL;
static const VkAccessFlagBits2 VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR = 0x400000000ULL;
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_decode_queue
static const VkAccessFlagBits2 VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR = 0x800000000ULL;
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_decode_queue
static const VkAccessFlagBits2 VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR = 0x1000000000ULL;
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
static const VkAccessFlagBits2 VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR = 0x2000000000ULL;
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
static const VkAccessFlagBits2 VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR = 0x4000000000ULL;
#endif
// Provided by VK_KHR_synchronization2 with VK_EXT_transform_feedback
static const VkAccessFlagBits2 VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000ULL;
// Provided by VK_KHR_synchronization2 with VK_EXT_transform_feedback
static const VkAccessFlagBits2 VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000ULL;
// Provided by VK_KHR_synchronization2 with VK_EXT_transform_feedback
static const VkAccessFlagBits2 VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000ULL;
// Provided by VK_KHR_synchronization2 with VK_EXT_conditional_rendering
static const VkAccessFlagBits2 VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_device_generated_commands
static const VkAccessFlagBits2 VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV = 0x00020000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_device_generated_commands
static const VkAccessFlagBits2 VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV = 0x00040000ULL;
// Provided by VK_KHR_fragment_shading_rate with VK_KHR_synchronization2
static const VkAccessFlagBits2 VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 0x00800000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_shading_rate_image
static const VkAccessFlagBits2 VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV = 0x00800000ULL;
// Provided by VK_KHR_acceleration_structure with VK_KHR_synchronization2
static const VkAccessFlagBits2 VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR = 0x00200000ULL;
// Provided by VK_KHR_acceleration_structure with VK_KHR_synchronization2
static const VkAccessFlagBits2 VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 0x00400000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_ray_tracing
static const VkAccessFlagBits2 VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV = 0x00200000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_ray_tracing
static const VkAccessFlagBits2 VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV = 0x00400000ULL;
// Provided by VK_KHR_synchronization2 with VK_EXT_fragment_density_map
static const VkAccessFlagBits2 VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000ULL;
// Provided by VK_KHR_synchronization2 with VK_EXT_blend_operation_advanced
static const VkAccessFlagBits2 VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000ULL;
// Provided by VK_HUAWEI_invocation_mask
static const VkAccessFlagBits2 VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI = 0x8000000000ULL;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkAccessFlagBits2 VkAccessFlagBits2KHR;
```

# Description
- [`ACCESS2_NONE`] specifies no accesses.
- [`ACCESS2_MEMORY_READ`] specifies all read accesses. It is always valid in any access mask, and is treated as equivalent to setting all `READ` access flags that are valid where it is used.
- [`ACCESS2_MEMORY_WRITE`] specifies all write accesses. It is always valid in any access mask, and is treated as equivalent to setting all `WRITE` access flags that are valid where it is used.
- [`ACCESS2_INDIRECT_COMMAND_READ`] specifies read access to     command data read from indirect buffers as part of an indirect build, trace,     drawing or dispatch command.     Such access occurs in the `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`     pipeline stage.
- [`ACCESS2_INDEX_READ`] specifies read access to an index buffer as part of an indexed drawing command, bound by [`cmd_bind_index_buffer`]. Such access occurs in the `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT` pipeline stage.
- [`ACCESS2_VERTEX_ATTRIBUTE_READ`] specifies read access to a vertex buffer as part of a drawing command, bound by [`cmd_bind_vertex_buffers`]. Such access occurs in the `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT` pipeline stage.
- [`ACCESS2_UNIFORM_READ`] specifies read access to a [uniform buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformbuffer) in any shader pipeline stage.
- [`ACCESS2_INPUT_ATTACHMENT_READ`] specifies read access to an [input attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass) within a render pass during subpass shading or fragment shading. Such access occurs in the `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI` or `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT` pipeline stage.
- [`ACCESS2_SHADER_SAMPLED_READ`] specifies read access to a [uniform texel buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer) or [sampled image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage) in any shader pipeline stage.
- [`ACCESS2_SHADER_STORAGE_READ`] specifies read access to a [storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebuffer), [physical storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-physical-storage-buffer), [storage texel buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer), or [storage image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage) in any shader pipeline stage.
- [`ACCESS2_SHADER_READ`] specifies read access to a [shader binding table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-binding-table) in any shader pipeline. In addition, it is equivalent to the logical OR of:  - [`ACCESS2_UNIFORM_READ`]  - [`ACCESS2_SHADER_SAMPLED_READ`]  - [`ACCESS2_SHADER_STORAGE_READ`] 
- [`ACCESS2_SHADER_STORAGE_WRITE`] specifies write access to a [storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebuffer), [physical storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-physical-storage-buffer), [storage texel buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer), or [storage image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage) in any shader pipeline stage.
- [`ACCESS2_SHADER_WRITE`] is equivalent to [`ACCESS2_SHADER_STORAGE_WRITE`].
- [`ACCESS2_COLOR_ATTACHMENT_READ`] specifies read access to a [color attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), such as via [blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending), [logic operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-logicop), or via certain [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). It does not include [advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced). Such access occurs in the `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
- [`ACCESS2_COLOR_ATTACHMENT_WRITE`] specifies write access to a [color, resolve, or depth/stencil resolve attachment](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#renderpass) during a [render pass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass) or via certain [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). Such access occurs in the `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
- [`ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ`] specifies read access to a [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), via [depth or stencil operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-ds-state) or via certain [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). Such access occurs in the `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT` or `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT` pipeline stages.
- [`ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE`] specifies write access to a [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), via [depth or stencil operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-ds-state) or via certain [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). Such access occurs in the `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT` or `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT` pipeline stages.
- [`ACCESS2_TRANSFER_READ`] specifies read access to an image or buffer in a [copy](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies) operation. Such access occurs in the `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`, or `VK_PIPELINE_STAGE_2_RESOLVE_BIT` pipeline stages.
- [`ACCESS2_TRANSFER_WRITE`] specifies write access to an image or buffer in a [clear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears) or [copy](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies) operation. Such access occurs in the `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`, or `VK_PIPELINE_STAGE_2_RESOLVE_BIT` pipeline stages.
- [`ACCESS2_HOST_READ`] specifies read access by a host operation. Accesses of this type are not performed through a resource, but directly on memory. Such access occurs in the `VK_PIPELINE_STAGE_2_HOST_BIT` pipeline stage.
- [`ACCESS2_HOST_WRITE`] specifies write access by a host operation. Accesses of this type are not performed through a resource, but directly on memory. Such access occurs in the `VK_PIPELINE_STAGE_2_HOST_BIT` pipeline stage.
- [`ACCESS2_CONDITIONAL_RENDERING_READ_EXT`] specifies read access to a predicate as part of conditional rendering. Such access occurs in the `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT` pipeline stage.
- [`ACCESS2_TRANSFORM_FEEDBACK_WRITE_EXT`] specifies write access to a transform feedback buffer made when transform feedback is active. Such access occurs in the `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT` pipeline stage.
- [`ACCESS2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT`] specifies read access to a transform feedback counter buffer which is read when [`cmd_begin_transform_feedback_ext`] executes. Such access occurs in the `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT` pipeline stage.
- [`ACCESS2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT`] specifies write access to a transform feedback counter buffer which is written when [`cmd_end_transform_feedback_ext`] executes. Such access occurs in the `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT` pipeline stage.
- [`ACCESS2_COMMAND_PREPROCESS_READ_NV`] specifies reads from buffer inputs to [`cmd_preprocess_generated_commands_nv`]. Such access occurs in the `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` pipeline stage.
- [`ACCESS2_COMMAND_PREPROCESS_WRITE_NV`] specifies writes to the target command buffer preprocess outputs. Such access occurs in the `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` pipeline stage.
- [`ACCESS2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT`] specifies read access to [color attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), including [advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced). Such access occurs in the `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
- [`ACCESS2_INVOCATION_MASK_READ_HUAWEI`] specifies read access to a invocation mask image in the `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI` pipeline stage.
- [`ACCESS2_ACCELERATION_STRUCTURE_READ_KHR`] specifies read access to an acceleration structure as part of a trace, build, or copy command, or to an [acceleration structure scratch buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-scratch) as part of a build command. Such access occurs in the `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR` pipeline stage or `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` pipeline stage.
- [`ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR`] specifies write access to an acceleration structure or [acceleration structure scratch buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-scratch) as part of a build or copy command. Such access occurs in the `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` pipeline stage.
- [`ACCESS2_FRAGMENT_DENSITY_MAP_READ_EXT`] specifies read access to a [fragment density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment) during dynamic [fragment density map operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymapops). Such access occurs in the `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT` pipeline stage.
- [`ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR`] specifies read access to a fragment shading rate attachment during rasterization. Such access occurs in the `VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` pipeline stage.
- [`ACCESS2_SHADING_RATE_IMAGE_READ_NV`] specifies read access to a shading rate image during rasterization. Such access occurs in the `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV` pipeline stage. It is equivalent to [`ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR`].
- [`ACCESS2_VIDEO_DECODE_READ_KHR`] specifies read access to an image or buffer resource as part of a [video decode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations). Such access occurs in the `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR` pipeline stage.
- [`ACCESS2_VIDEO_DECODE_WRITE_KHR`] specifies write access to an image or buffer resource as part of a [video decode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations). Such access occurs in the `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR` pipeline stage.
- [`ACCESS2_VIDEO_ENCODE_READ_KHR`] specifies read access to an image or buffer resource as part of a [video encode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations). Such access occurs in the `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR` pipeline stage.
- [`ACCESS2_VIDEO_ENCODE_WRITE_KHR`] specifies write access to an image or buffer resource as part of a [video encode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations). Such access occurs in the `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR` pipeline stage.

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        