[VkAccessFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccessFlagBits.html) - Bitmask specifying memory access types that will participate in a memory dependency

# C Specifications
Bits which  **can**  be set in the `srcAccessMask` and `dstAccessMask`
members of [`SubpassDependency`],
[`SubpassDependency2`],
[`MemoryBarrier`], [`BufferMemoryBarrier`], and
[`ImageMemoryBarrier`], specifying access behavior, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkAccessFlagBits {
    VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,
    VK_ACCESS_INDEX_READ_BIT = 0x00000002,
    VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
    VK_ACCESS_UNIFORM_READ_BIT = 0x00000008,
    VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
    VK_ACCESS_SHADER_READ_BIT = 0x00000020,
    VK_ACCESS_SHADER_WRITE_BIT = 0x00000040,
    VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
    VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
    VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
    VK_ACCESS_TRANSFER_READ_BIT = 0x00000800,
    VK_ACCESS_TRANSFER_WRITE_BIT = 0x00001000,
    VK_ACCESS_HOST_READ_BIT = 0x00002000,
    VK_ACCESS_HOST_WRITE_BIT = 0x00004000,
    VK_ACCESS_MEMORY_READ_BIT = 0x00008000,
    VK_ACCESS_MEMORY_WRITE_BIT = 0x00010000,
  // Provided by VK_VERSION_1_3
    VK_ACCESS_NONE = 0,
  // Provided by VK_EXT_transform_feedback
    VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000,
  // Provided by VK_EXT_transform_feedback
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000,
  // Provided by VK_EXT_transform_feedback
    VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000,
  // Provided by VK_EXT_conditional_rendering
    VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000,
  // Provided by VK_EXT_blend_operation_advanced
    VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000,
  // Provided by VK_KHR_acceleration_structure
    VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR = 0x00200000,
  // Provided by VK_KHR_acceleration_structure
    VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 0x00400000,
  // Provided by VK_EXT_fragment_density_map
    VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000,
  // Provided by VK_KHR_fragment_shading_rate
    VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 0x00800000,
  // Provided by VK_NV_device_generated_commands
    VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV = 0x00020000,
  // Provided by VK_NV_device_generated_commands
    VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV = 0x00040000,
  // Provided by VK_NV_shading_rate_image
    VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV = VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV = VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR,
  // Provided by VK_NV_ray_tracing
    VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV = VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR,
  // Provided by VK_KHR_synchronization2
    VK_ACCESS_NONE_KHR = VK_ACCESS_NONE,
} VkAccessFlagBits;
```

# Description
These values all have the same meaning as the equivalently named values for
[`AccessFlags2`].
- [`NONE`] specifies no accesses.
- [`MEMORY_READ`] specifies all read accesses. It is always valid in any access mask, and is treated as equivalent to setting all `READ` access flags that are valid where it is used.
- [`MEMORY_WRITE`] specifies all write accesses. It is always valid in any access mask, and is treated as equivalent to setting all `WRITE` access flags that are valid where it is used.
- [`INDIRECT_COMMAND_READ`] specifies read access to     indirect command data read as part of an indirect build, trace,     drawing or dispatching command.     Such access occurs in the `VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT`     pipeline stage.
- [`INDEX_READ`] specifies read access to an index buffer as part of an indexed drawing command, bound by [`cmd_bind_index_buffer`]. Such access occurs in the `VK_PIPELINE_STAGE_VERTEX_INPUT_BIT` pipeline stage.
- [`VERTEX_ATTRIBUTE_READ`] specifies read access to a vertex buffer as part of a drawing command, bound by [`cmd_bind_vertex_buffers`]. Such access occurs in the `VK_PIPELINE_STAGE_VERTEX_INPUT_BIT` pipeline stage.
- [`UNIFORM_READ`] specifies read access to a [uniform buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformbuffer) in any shader pipeline stage.
- [`INPUT_ATTACHMENT_READ`] specifies read access to an [input attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass) within a render pass during subpass shading or fragment shading. Such access occurs in the `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI` or `VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT` pipeline stage.
- [`SHADER_READ`] specifies read access to a [uniform buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformbuffer), [uniform texel buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer), [sampled image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage), [storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebuffer), [physical storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-physical-storage-buffer), [shader binding table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-binding-table), [storage texel buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer), or [storage image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage) in any shader pipeline stage.
- [`SHADER_WRITE`] specifies write access to a [storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebuffer), [physical storage buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-physical-storage-buffer), [storage texel buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagetexelbuffer), or [storage image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storageimage) in any shader pipeline stage.
- [`COLOR_ATTACHMENT_READ`] specifies read access to a [color attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), such as via [blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blending), [logic operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-logicop), or via certain [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). It does not include [advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced). Such access occurs in the `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
- [`COLOR_ATTACHMENT_WRITE`] specifies write access to a [color, resolve, or depth/stencil resolve attachment](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#renderpass) during a [render pass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass) or via certain [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). Such access occurs in the `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
- [`DEPTH_STENCIL_ATTACHMENT_READ`] specifies read access to a [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), via [depth or stencil operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-ds-state) or via certain [subpass load operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). Such access occurs in the `VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT` or `VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT` pipeline stages.
- [`DEPTH_STENCIL_ATTACHMENT_WRITE`] specifies write access to a [depth/stencil attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), via [depth or stencil operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-ds-state) or via certain [subpass load and store operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-load-store-ops). Such access occurs in the `VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT` or `VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT` pipeline stages.
- [`TRANSFER_READ`] specifies read access to an image or buffer in a [copy](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies) operation. Such access occurs in the `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT` pipeline stage.
- [`TRANSFER_WRITE`] specifies write access to an image or buffer in a [clear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#clears) or [copy](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies) operation. Such access occurs in the `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT` pipeline stage.
- [`HOST_READ`] specifies read access by a host operation. Accesses of this type are not performed through a resource, but directly on memory. Such access occurs in the `VK_PIPELINE_STAGE_HOST_BIT` pipeline stage.
- [`HOST_WRITE`] specifies write access by a host operation. Accesses of this type are not performed through a resource, but directly on memory. Such access occurs in the `VK_PIPELINE_STAGE_HOST_BIT` pipeline stage.
- [`CONDITIONAL_RENDERING_READ_EXT`] specifies read access to a predicate as part of conditional rendering. Such access occurs in the `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT` pipeline stage.
- [`TRANSFORM_FEEDBACK_WRITE_EXT`] specifies write access to a transform feedback buffer made when transform feedback is active. Such access occurs in the `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT` pipeline stage.
- [`TRANSFORM_FEEDBACK_COUNTER_READ_EXT`] specifies read access to a transform feedback counter buffer which is read when [`cmd_begin_transform_feedback_ext`] executes. Such access occurs in the `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT` pipeline stage.
- [`TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT`] specifies write access to a transform feedback counter buffer which is written when [`cmd_end_transform_feedback_ext`] executes. Such access occurs in the `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT` pipeline stage.
- [`COMMAND_PREPROCESS_READ_NV`] specifies reads from buffer inputs to [`cmd_preprocess_generated_commands_nv`]. Such access occurs in the `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV` pipeline stage.
- [`COMMAND_PREPROCESS_WRITE_NV`] specifies writes to the target command buffer:VkBuffer preprocess outputs in [`cmd_preprocess_generated_commands_nv`]. Such access occurs in the `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV` pipeline stage.
- [`COLOR_ATTACHMENT_READ_NONCOHERENT_EXT`] specifies read access to [color attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass), including [advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced). Such access occurs in the `VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT` pipeline stage.
- `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI` specifies read access to a invocation mask image in the `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI` pipeline stage.
- [`ACCELERATION_STRUCTURE_READ_KHR`] specifies read access to an acceleration structure as part of a trace, build, or copy command, or to an [acceleration structure scratch buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-scratch) as part of a build command. Such access occurs in the `VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR` pipeline stage or `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` pipeline stage.
- [`ACCELERATION_STRUCTURE_WRITE_KHR`] specifies write access to an acceleration structure or [acceleration structure scratch buffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-scratch) as part of a build or copy command. Such access occurs in the `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` pipeline stage.
- [`FRAGMENT_DENSITY_MAP_READ_EXT`] specifies read access to a [fragment density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment) during dynamic [fragment density map operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymapops) Such access occurs in the `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT` pipeline stage.
- [`FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR`] specifies read access to a fragment shading rate attachment during rasterization. Such access occurs in the `VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` pipeline stage.
- [`SHADING_RATE_IMAGE_READ_NV`] specifies read access to a shading rate image during rasterization. Such access occurs in the `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV` pipeline stage. It is equivalent to `VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`.
Certain access types are only performed by a subset of pipeline stages.
Any synchronization command that takes both stage masks and access masks
uses both to define the [access
scopes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) - only the specified access types performed by the specified stages
are included in the access scope.
An application  **must**  not specify an access flag in a synchronization command
if it does not include a pipeline stage in the corresponding stage mask that
is able to perform accesses of that type.
The following table lists, for each access flag, which pipeline stages  **can** 
perform that type of access.

# Related
- [`crate::vulkan1_0`]
- [`AccessFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        