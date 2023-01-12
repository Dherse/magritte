[VkPipelineStageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineStageFlagBits.html) - Bitmask specifying pipeline stages

# C Specifications
Bits which  **can**  be set in a [VkPipelineStageFlags]() mask, specifying
stages of execution, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkPipelineStageFlagBits {
    VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,
    VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,
    VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,
    VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,
    VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
    VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
    VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,
    VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,
    VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
    VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
    VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
    VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,
    VK_PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,
    VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,
    VK_PIPELINE_STAGE_HOST_BIT = 0x00004000,
    VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,
    VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000,
  // Provided by VK_VERSION_1_3
    VK_PIPELINE_STAGE_NONE = 0,
  // Provided by VK_EXT_transform_feedback
    VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000,
  // Provided by VK_EXT_conditional_rendering
    VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00040000,
  // Provided by VK_KHR_acceleration_structure
    VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 0x02000000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR = 0x00200000,
  // Provided by VK_NV_mesh_shader
    VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV = 0x00080000,
  // Provided by VK_NV_mesh_shader
    VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV = 0x00100000,
  // Provided by VK_EXT_fragment_density_map
    VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT = 0x00800000,
  // Provided by VK_KHR_fragment_shading_rate
    VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00400000,
  // Provided by VK_NV_device_generated_commands
    VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV = 0x00020000,
  // Provided by VK_NV_shading_rate_image
    VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV = VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV = VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV = VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR,
  // Provided by VK_KHR_synchronization2
    VK_PIPELINE_STAGE_NONE_KHR = VK_PIPELINE_STAGE_NONE,
} VkPipelineStageFlagBits;
```

# Description
These values all have the same meaning as the equivalently named values for
[`PipelineStageFlags2`].
- [`NONE`] specifies no stages of execution.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the stage of the pipeline where `VkDrawIndirect*` / `VkDispatchIndirect*` / `VkTraceRaysIndirect*` data structures are consumed. This stage also includes reading commands written by [`cmd_execute_generated_commands_nv`].
- [`TASK_SHADER_NV`] specifies the task shader stage.
- [`MESH_SHADER_NV`] specifies the mesh shader stage.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the stage of the pipeline where vertex and index buffers are consumed.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the vertex shader stage.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the tessellation control shader stage.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the tessellation evaluation shader stage.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the geometry shader stage.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the fragment shader stage.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the stage of the pipeline where early fragment tests (depth and stencil tests before fragment shading) are performed. This stage also includes [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a depth/stencil format.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the stage of the pipeline where late fragment tests (depth and stencil tests after fragment shading) are performed. This stage also includes [subpass store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) for framebuffer attachments with a depth/stencil format.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the stage of the pipeline after blending where the final color values are output from the pipeline. This stage also includes [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops) and multisample resolve operations for framebuffer attachments with a color or depth/stencil format.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the execution of a compute shader.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the following commands:  - All [copy commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies), including [`cmd_copy_query_pool_results`]  - [`cmd_blit_image2`] and [`cmd_blit_image`]  - [`cmd_resolve_image2`] and [`cmd_resolve_image`]  - All [clear commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears), with the exception of [`cmd_clear_attachments`] 
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies a pseudo-stage indicating execution on the host of reads/writes of device memory. This stage is not invoked by any commands recorded in a command buffer.
- [`ACCELERATION_STRUCTURE_BUILD_KHR`] specifies     the execution of     [`cmd_build_acceleration_structure_nv`],     [`cmd_copy_acceleration_structure_nv`],     [`cmd_write_acceleration_structures_properties_nv`] ,     [`cmd_build_acceleration_structures_khr`],     [`cmd_build_acceleration_structures_indirect_khr`],     [`cmd_copy_acceleration_structure_khr`],     [`cmd_copy_acceleration_structure_to_memory_khr`],     [`cmd_copy_memory_to_acceleration_structure_khr`], and     [`cmd_write_acceleration_structures_properties_khr`].
- [`RAY_TRACING_SHADER_KHR`] specifies the     execution of the ray tracing shader stages, via [`cmd_trace_rays_nv`] , [`cmd_trace_rays_khr`], or [`cmd_trace_rays_indirect_khr`]
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies the execution of all graphics pipeline stages, and is equivalent to the logical OR of:  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`TASK_SHADER_NV`]  - [`MESH_SHADER_NV`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`VK_PIPELINE_STAGE_FLAG_BITS`]  - [`CONDITIONAL_RENDERING_EXT`]  - [`TRANSFORM_FEEDBACK_EXT`]  - [`FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`]  - [`FRAGMENT_DENSITY_PROCESS_EXT`] 
- [`VK_PIPELINE_STAGE_FLAG_BITS`] specifies all operations performed by all commands supported on the queue it is used with.
- [`CONDITIONAL_RENDERING_EXT`] specifies the stage of the pipeline where the predicate of conditional rendering is consumed.
- [`TRANSFORM_FEEDBACK_EXT`] specifies the stage of the pipeline where vertex attribute output values are written to the transform feedback buffers.
- [`COMMAND_PREPROCESS_NV`] specifies the stage of the pipeline where device-side preprocessing for generated commands via [`cmd_preprocess_generated_commands_nv`] is handled.
- [`FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`]     specifies the stage of the pipeline where the     [fragment shading rate     attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment) or     [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-shading-rate-image)     is read to determine the fragment shading rate for portions of a     rasterized primitive.
- [`FRAGMENT_DENSITY_PROCESS_EXT`] specifies the stage of the pipeline where the fragment density map is read to [generate the fragment areas](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymapops).
- [`VK_PIPELINE_STAGE_FLAG_BITS`] is equivalent to [`VK_PIPELINE_STAGE_FLAG_BITS`] with [VkAccessFlags]() set to `0` when specified in the second synchronization scope, but specifies no stage of execution when specified in the first scope.
- [`VK_PIPELINE_STAGE_FLAG_BITS`] is equivalent to [`VK_PIPELINE_STAGE_FLAG_BITS`] with [VkAccessFlags]() set to `0` when specified in the first synchronization scope, but specifies no stage of execution when specified in the second scope.

# Related
- [`crate::vulkan1_0`]
- [`CheckpointDataNV`]
- [VkPipelineStageFlags]()
- [`cmd_write_buffer_marker_amd`]
- [`cmd_write_timestamp`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        