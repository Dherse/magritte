[VkPipelineStageFlagBits2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits2.html) - Pipeline stage flags for VkPipelineStageFlags2

# C Specifications
Bits which  **can**  be set in a [`PipelineStageFlags2`] mask, specifying
stages of execution, are:
```c
// Provided by VK_VERSION_1_3
// Flag bits for VkPipelineStageFlagBits2
typedef VkFlags64 VkPipelineStageFlagBits2;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_NONE = 0ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_NONE_KHR = 0ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT = 0x00000001ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR = 0x00000001ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT = 0x00000002ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR = 0x00000002ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT = 0x00000004ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR = 0x00000004ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT = 0x00000008ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR = 0x00000008ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR = 0x00000010ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR = 0x00000020ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT = 0x00000040ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR = 0x00000040ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT = 0x00000080ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR = 0x00000080ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT = 0x00000100ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR = 0x00000100ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT = 0x00000200ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR = 0x00000200ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR = 0x00000400ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT = 0x00000800ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR = 0x00000800ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT = 0x00001000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR = 0x00001000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TRANSFER_BIT = 0x00001000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR = 0x00001000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT = 0x00002000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR = 0x00002000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_HOST_BIT = 0x00004000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_HOST_BIT_KHR = 0x00004000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT = 0x00008000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR = 0x00008000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT = 0x00010000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR = 0x00010000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_COPY_BIT = 0x100000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_COPY_BIT_KHR = 0x100000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_RESOLVE_BIT = 0x200000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR = 0x200000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_BLIT_BIT = 0x400000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_BLIT_BIT_KHR = 0x400000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_CLEAR_BIT = 0x800000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR = 0x800000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT = 0x1000000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR = 0x1000000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT = 0x2000000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR = 0x2000000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT = 0x4000000000ULL;
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR = 0x4000000000ULL;
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_decode_queue
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR = 0x04000000ULL;
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_KHR_video_encode_queue
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR = 0x08000000ULL;
#endif
// Provided by VK_KHR_synchronization2 with VK_EXT_transform_feedback
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000ULL;
// Provided by VK_KHR_synchronization2 with VK_EXT_conditional_rendering
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT = 0x00040000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_device_generated_commands
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV = 0x00020000ULL;
// Provided by VK_KHR_fragment_shading_rate with VK_KHR_synchronization2
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00400000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_shading_rate_image
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV = 0x00400000ULL;
// Provided by VK_KHR_acceleration_structure with VK_KHR_synchronization2
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 0x02000000ULL;
// Provided by VK_KHR_ray_tracing_pipeline with VK_KHR_synchronization2
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR = 0x00200000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_ray_tracing
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV = 0x00200000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_ray_tracing
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV = 0x02000000ULL;
// Provided by VK_KHR_synchronization2 with VK_EXT_fragment_density_map
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT = 0x00800000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_mesh_shader
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV = 0x00080000ULL;
// Provided by VK_KHR_synchronization2 with VK_NV_mesh_shader
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV = 0x00100000ULL;
// Provided by VK_HUAWEI_subpass_shading
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI = 0x8000000000ULL;
// Provided by VK_HUAWEI_invocation_mask
static const VkPipelineStageFlagBits2 VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI = 0x10000000000ULL;
```
or the equivalent
```c
// Provided by VK_KHR_synchronization2
typedef VkPipelineStageFlagBits2 VkPipelineStageFlagBits2KHR;
```

# Description
- [`PIPELINE_STAGE2_NONE`] specifies no stages of execution.
- [`PIPELINE_STAGE2_DRAW_INDIRECT`] specifies the stage of the pipeline where indirect command parameters are consumed. This stage also includes reading commands written by [`cmd_preprocess_generated_commands_nv`].
- [`PIPELINE_STAGE2_TASK_SHADER_NV`] specifies the task shader stage.
- [`PIPELINE_STAGE2_MESH_SHADER_NV`] specifies the mesh shader stage.
- [`PIPELINE_STAGE2_INDEX_INPUT`] specifies the stage of the pipeline where index buffers are consumed.
- [`PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT`] specifies the stage of the pipeline where vertex buffers are consumed.
- [`PIPELINE_STAGE2_VERTEX_INPUT`] is equivalent to the logical OR of:  - [`PIPELINE_STAGE2_INDEX_INPUT`]  - [`PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT`] 
- [`PIPELINE_STAGE2_VERTEX_SHADER`] specifies the vertex shader stage.
- [`PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER`] specifies the tessellation control shader stage.
- [`PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER`] specifies the tessellation evaluation shader stage.
- [`PIPELINE_STAGE2_GEOMETRY_SHADER`] specifies the geometry shader stage.
- [`PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS`] is equivalent to specifying all supported [pre-rasterization shader stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization):  - [`PIPELINE_STAGE2_VERTEX_SHADER`]  - [`PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER`]  - [`PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER`]  - [`PIPELINE_STAGE2_GEOMETRY_SHADER`]  - [`PIPELINE_STAGE2_TASK_SHADER_NV`]  - [`PIPELINE_STAGE2_MESH_SHADER_NV`] 
- [`PIPELINE_STAGE2_FRAGMENT_SHADER`] specifies the fragment shader stage.
- [`PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS`] specifies the stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed. This stage also includes [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a depth/stencil format.
- [`PIPELINE_STAGE2_LATE_FRAGMENT_TESTS`] specifies the stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed. This stage also includes [subpass store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a depth/stencil format.
- [`PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT`] specifies the stage of the pipeline after blending where the final color values are output from the pipeline. This stage also includes [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) and multisample resolve operations for framebuffer attachments with a color or depth/stencil format.
- [`PIPELINE_STAGE2_COMPUTE_SHADER`] specifies the compute shader stage.
- [`PIPELINE_STAGE2_HOST`] specifies a pseudo-stage indicating execution on the host of reads/writes of device memory. This stage is not invoked by any commands recorded in a command buffer.
- [`PIPELINE_STAGE2_COPY`] specifies the execution of all [copy commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies), including [`cmd_copy_query_pool_results`].
- [`PIPELINE_STAGE2_BLIT`] specifies the execution of [`cmd_blit_image`].
- [`PIPELINE_STAGE2_RESOLVE`] specifies the execution of [`cmd_resolve_image`].
- [`PIPELINE_STAGE2_CLEAR`] specifies the execution of [clear commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears), with the exception of [`cmd_clear_attachments`].
- [`PIPELINE_STAGE2_ALL_TRANSFER`] is equivalent to specifying all of:  - [`PIPELINE_STAGE2_COPY`]  - [`PIPELINE_STAGE2_BLIT`]  - [`PIPELINE_STAGE2_RESOLVE`]  - [`PIPELINE_STAGE2_CLEAR`] 
- [`PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR`] specifies the execution of the ray tracing shader stages.
- [`PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR`] specifies the execution of [acceleration structure commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure).
- [`PIPELINE_STAGE2_ALL_GRAPHICS`] specifies the execution of all graphics pipeline stages, and is equivalent to the logical OR of:  - [`PIPELINE_STAGE2_DRAW_INDIRECT`]  - [`PIPELINE_STAGE2_TASK_SHADER_NV`]  - [`PIPELINE_STAGE2_MESH_SHADER_NV`]  - [`PIPELINE_STAGE2_VERTEX_INPUT`]  - [`PIPELINE_STAGE2_VERTEX_SHADER`]  - [`PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER`]  - [`PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER`]  - [`PIPELINE_STAGE2_GEOMETRY_SHADER`]  - [`PIPELINE_STAGE2_FRAGMENT_SHADER`]  - [`PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS`]  - [`PIPELINE_STAGE2_LATE_FRAGMENT_TESTS`]  - [`PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT`]  - [`PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT`]  - [`PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT`]  - [`PIPELINE_STAGE2_SHADING_RATE_IMAGE_NV`]  - [`PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT`]  - [`PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI`] 
- [`PIPELINE_STAGE2_ALL_COMMANDS`] specifies all operations performed by all commands supported on the queue it is used with.
- [`PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT`] specifies the stage of the pipeline where the predicate of conditional rendering is consumed.
- [`PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT`] specifies the stage of the pipeline where vertex attribute output values are written to the transform feedback buffers.
- [`PIPELINE_STAGE2_COMMAND_PREPROCESS_NV`] specifies the stage of the pipeline where device-side generation of commands via [`cmd_preprocess_generated_commands_nv`] is handled.
- [`PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`]     specifies the stage of the pipeline where the     [fragment shading rate     attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment) or     [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-shading-rate-image)     is read to determine the fragment shading rate for portions of a     rasterized primitive.
- [`PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT`] specifies the stage of the pipeline where the fragment density map is read to [generate the fragment areas](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymapops).
- [`PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI`] specifies the stage of the pipeline where the invocation mask image is read by the implementation to optimize the ray dispatch.
- [`PIPELINE_STAGE2_VIDEO_DECODE_KHR`] specifies the stage of the pipeline where [video decode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-decode-operations) are performed.
- [`PIPELINE_STAGE2_VIDEO_ENCODE_KHR`] specifies the stage of the pipeline where [video encode operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-encode-operations) are performed.
- [`PIPELINE_STAGE2_SUBPASS_SHADING_HUAWEI`] specifies the subpass shading shader stage.
- [`PIPELINE_STAGE2_TOP_OF_PIPE`] is equivalent to [`PIPELINE_STAGE2_ALL_COMMANDS`] with [`AccessFlags2`] set to `0` when specified in the second synchronization scope, but equivalent to [`PIPELINE_STAGE2_NONE`] in the first scope.
- [`PIPELINE_STAGE2_BOTTOM_OF_PIPE`] is equivalent to [`PIPELINE_STAGE2_ALL_COMMANDS`] with [`AccessFlags2`] set to `0` when specified in the first synchronization scope, but equivalent to [`PIPELINE_STAGE2_NONE`] in the second scope.

# Related
- [`VK_KHR_synchronization2`]
- [`crate::vulkan1_3`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        