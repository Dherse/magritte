[VkDynamicState](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDynamicState.html) - Indicate which dynamic state is taken from dynamic state commands

# C Specifications
The source of different pieces of dynamic state is specified by the
[`PipelineDynamicStateCreateInfo::dynamic_states`] property of the
currently active pipeline, each of whose elements  **must**  be one of the
values:
```c
// Provided by VK_VERSION_1_0
typedef enum VkDynamicState {
    VK_DYNAMIC_STATE_VIEWPORT = 0,
    VK_DYNAMIC_STATE_SCISSOR = 1,
    VK_DYNAMIC_STATE_LINE_WIDTH = 2,
    VK_DYNAMIC_STATE_DEPTH_BIAS = 3,
    VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4,
    VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5,
    VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,
    VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,
    VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_CULL_MODE = 1000267000,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_FRONT_FACE = 1000267001,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY = 1000267002,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT = 1000267003,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT = 1000267004,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE = 1000267005,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE = 1000267006,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE = 1000267007,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_DEPTH_COMPARE_OP = 1000267008,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE = 1000267009,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE = 1000267010,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_STENCIL_OP = 1000267011,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE = 1000377001,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE = 1000377002,
  // Provided by VK_VERSION_1_3
    VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE = 1000377004,
  // Provided by VK_NV_clip_space_w_scaling
    VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV = 1000087000,
  // Provided by VK_EXT_discard_rectangles
    VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT = 1000099000,
  // Provided by VK_EXT_sample_locations
    VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT = 1000143000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR = 1000347000,
  // Provided by VK_NV_shading_rate_image
    VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV = 1000164004,
  // Provided by VK_NV_shading_rate_image
    VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV = 1000164006,
  // Provided by VK_NV_scissor_exclusive
    VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV = 1000205001,
  // Provided by VK_KHR_fragment_shading_rate
    VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR = 1000226000,
  // Provided by VK_EXT_line_rasterization
    VK_DYNAMIC_STATE_LINE_STIPPLE_EXT = 1000259000,
  // Provided by VK_EXT_vertex_input_dynamic_state
    VK_DYNAMIC_STATE_VERTEX_INPUT_EXT = 1000352000,
  // Provided by VK_EXT_extended_dynamic_state2
    VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT = 1000377000,
  // Provided by VK_EXT_extended_dynamic_state2
    VK_DYNAMIC_STATE_LOGIC_OP_EXT = 1000377003,
  // Provided by VK_EXT_color_write_enable
    VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT = 1000381000,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_CULL_MODE_EXT = VK_DYNAMIC_STATE_CULL_MODE,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_FRONT_FACE_EXT = VK_DYNAMIC_STATE_FRONT_FACE,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT = VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT = VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT = VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT = VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT = VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT = VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT = VK_DYNAMIC_STATE_DEPTH_COMPARE_OP,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT = VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT = VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE,
  // Provided by VK_EXT_extended_dynamic_state
    VK_DYNAMIC_STATE_STENCIL_OP_EXT = VK_DYNAMIC_STATE_STENCIL_OP,
  // Provided by VK_EXT_extended_dynamic_state2
    VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE_EXT = VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE,
  // Provided by VK_EXT_extended_dynamic_state2
    VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE_EXT = VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE,
  // Provided by VK_EXT_extended_dynamic_state2
    VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT = VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE,
} VkDynamicState;
```

# Description
- [`VK_DYNAMIC_STATE`] specifies that the `pViewports` state in [`PipelineViewportStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_viewport`] before any drawing commands. The number of viewports used by a pipeline is still specified by the `viewportCount` member of [`PipelineViewportStateCreateInfo`].
- [`VK_DYNAMIC_STATE`] specifies that the `pScissors` state in [`PipelineViewportStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_scissor`] before any drawing commands. The number of scissor rectangles used by a pipeline is still specified by the `scissorCount` member of [`PipelineViewportStateCreateInfo`].
- [`VK_DYNAMIC_STATE`] specifies that the `lineWidth` state in [`PipelineRasterizationStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_line_width`] before any drawing commands that generate line primitives for the rasterizer.
- [`VK_DYNAMIC_STATE`] specifies that the `depthBiasConstantFactor`, `depthBiasClamp` and `depthBiasSlopeFactor` states in [`PipelineRasterizationStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_depth_bias`] before any draws are performed with `depthBiasEnable` in [`PipelineRasterizationStateCreateInfo`] set to `VK_TRUE`.
- [`VK_DYNAMIC_STATE`] specifies that the `blendConstants` state in [`PipelineColorBlendStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_blend_constants`] before any draws are performed with a pipeline state with [`PipelineColorBlendAttachmentState`] member `blendEnable` set to `VK_TRUE` and any of the blend functions using a constant blend color.
- [`VK_DYNAMIC_STATE`] specifies that the `minDepthBounds` and `maxDepthBounds` states of [`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_depth_bounds`] before any draws are performed with a pipeline state with [`PipelineDepthStencilStateCreateInfo`] member `depthBoundsTestEnable` set to `VK_TRUE`.
- [`VK_DYNAMIC_STATE`] specifies that the `compareMask` state in [`PipelineDepthStencilStateCreateInfo`] for both `front` and `back` will be ignored and  **must**  be set dynamically with [`cmd_set_stencil_compare_mask`] before any draws are performed with a pipeline state with [`PipelineDepthStencilStateCreateInfo`] member `stencilTestEnable` set to `VK_TRUE`
- [`VK_DYNAMIC_STATE`] specifies that the `writeMask` state in [`PipelineDepthStencilStateCreateInfo`] for both `front` and `back` will be ignored and  **must**  be set dynamically with [`cmd_set_stencil_write_mask`] before any draws are performed with a pipeline state with [`PipelineDepthStencilStateCreateInfo`] member `stencilTestEnable` set to `VK_TRUE`
- [`VK_DYNAMIC_STATE`] specifies that the `reference` state in [`PipelineDepthStencilStateCreateInfo`] for both `front` and `back` will be ignored and  **must**  be set dynamically with [`cmd_set_stencil_reference`] before any draws are performed with a pipeline state with [`PipelineDepthStencilStateCreateInfo`] member `stencilTestEnable` set to `VK_TRUE`
- [`VIEWPORT_W_SCALING_NV`] specifies that the `pViewportScalings` state in [`PipelineViewportWScalingStateCreateInfoNV`] will be ignored and  **must**  be set dynamically with [`cmd_set_viewport_w_scaling_nv`] before any draws are performed with a pipeline state with [`PipelineViewportWScalingStateCreateInfoNV`] member `viewportScalingEnable` set to `VK_TRUE`
- [`DISCARD_RECTANGLE_EXT`] specifies that the `pDiscardRectangles` state in [`PipelineDiscardRectangleStateCreateInfoEXT`] will be ignored and  **must**  be set dynamically with [`cmd_set_discard_rectangle_ext`] before any draw or clear commands. The [`DiscardRectangleModeEXT`] and the number of active discard rectangles is still specified by the `discardRectangleMode` and `discardRectangleCount` members of [`PipelineDiscardRectangleStateCreateInfoEXT`].
- [`SAMPLE_LOCATIONS_EXT`] specifies that the `sampleLocationsInfo` state in [`PipelineSampleLocationsStateCreateInfoEXT`] will be ignored and  **must**  be set dynamically with [`cmd_set_sample_locations_ext`] before any draw or clear commands. Enabling custom sample locations is still indicated by the `sampleLocationsEnable` member of [`PipelineSampleLocationsStateCreateInfoEXT`].
- [`EXCLUSIVE_SCISSOR_NV`] specifies that the `pExclusiveScissors` state in [`PipelineViewportExclusiveScissorStateCreateInfoNV`] will be ignored and  **must**  be set dynamically with [`cmd_set_exclusive_scissor_nv`] before any drawing commands. The number of exclusive scissor rectangles used by a pipeline is still specified by the `exclusiveScissorCount` member of [`PipelineViewportExclusiveScissorStateCreateInfoNV`].
- [`VIEWPORT_SHADING_RATE_PALETTE_NV`] specifies that the `pShadingRatePalettes` state in [`PipelineViewportShadingRateImageStateCreateInfoNV`] will be ignored and  **must**  be set dynamically with [`cmd_set_viewport_shading_rate_palette_nv`] before any drawing commands.
- [`VIEWPORT_COARSE_SAMPLE_ORDER_NV`] specifies that the coarse sample order state in [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] will be ignored and  **must**  be set dynamically with [`cmd_set_coarse_sample_order_nv`] before any drawing commands.
- [`LINE_STIPPLE_EXT`] specifies that the `lineStippleFactor` and `lineStipplePattern` state in [`PipelineRasterizationLineStateCreateInfoEXT`] will be ignored and  **must**  be set dynamically with [`cmd_set_line_stipple_ext`] before any draws are performed with a pipeline state with [`PipelineRasterizationLineStateCreateInfoEXT`] member `stippledLineEnable` set to `VK_TRUE`.
- [`CULL_MODE`] specifies that the `cullMode` state in [`PipelineRasterizationStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_cull_mode`] before any drawing commands.
- [`FRONT_FACE`] specifies that the `frontFace` state in [`PipelineRasterizationStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_front_face`] before any drawing commands.
- [`PRIMITIVE_TOPOLOGY`] specifies that the `topology` state in [`PipelineInputAssemblyStateCreateInfo`] only specifies the [topology class](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-topology-class), and the specific topology order and adjacency  **must**  be set dynamically with [`cmd_set_primitive_topology`] before any drawing commands.
- [`VIEWPORT_WITH_COUNT`] specifies that the `viewportCount` and `pViewports` state in [`PipelineViewportStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_viewport_with_count`] before any draw call.
- [`SCISSOR_WITH_COUNT`] specifies that the `scissorCount` and `pScissors` state in [`PipelineViewportStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_scissor_with_count`] before any draw call.
- [`VERTEX_INPUT_BINDING_STRIDE`] specifies that the `stride` state in [`VertexInputBindingDescription`] will be ignored and  **must**  be set dynamically with [`cmd_bind_vertex_buffers2`] before any draw call.
- [`DEPTH_TEST_ENABLE`] specifies that the `depthTestEnable` state in [`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_depth_test_enable`] before any draw call.
- [`DEPTH_WRITE_ENABLE`] specifies that the `depthWriteEnable` state in [`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_depth_write_enable`] before any draw call.
- [`DEPTH_COMPARE_OP`] specifies that the `depthCompareOp` state in [`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_depth_compare_op`] before any draw call.
- [`DEPTH_BOUNDS_TEST_ENABLE`] specifies that the `depthBoundsTestEnable` state in [`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_depth_bounds_test_enable`] before any draw call.
- [`STENCIL_TEST_ENABLE`] specifies that the `stencilTestEnable` state in [`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_stencil_test_enable`] before any draw call.
- [`STENCIL_OP`] specifies that the `failOp`, `passOp`, `depthFailOp`, and `compareOp` states in [`PipelineDepthStencilStateCreateInfo`] for both `front` and `back` will be ignored and  **must**  be set dynamically with [`cmd_set_stencil_op`] before any draws are performed with a pipeline state with [`PipelineDepthStencilStateCreateInfo`] member `stencilTestEnable` set to `VK_TRUE`
- [`PATCH_CONTROL_POINTS_EXT`] specifies that the `patchControlPoints` state in [`PipelineTessellationStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_patch_control_points_ext`] before any drawing commands.
- [`RASTERIZER_DISCARD_ENABLE`] specifies that the `rasterizerDiscardEnable` state in [`PipelineRasterizationStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_rasterizer_discard_enable`] before any drawing commands.
- [`DEPTH_BIAS_ENABLE`] specifies that the `depthBiasEnable` state in [`PipelineRasterizationStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_depth_bias_enable`] before any drawing commands.
- [`LOGIC_OP_EXT`] specifies that the `logicOp` state in [`PipelineColorBlendStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_logic_op_ext`] before any drawing commands.
- [`PRIMITIVE_RESTART_ENABLE`] specifies that the `primitiveRestartEnable` state in [`PipelineInputAssemblyStateCreateInfo`] will be ignored and  **must**  be set dynamically with [`cmd_set_primitive_restart_enable`] before any drawing commands.
- [`FRAGMENT_SHADING_RATE_KHR`] specifies that state in [`PipelineFragmentShadingRateStateCreateInfoKHR`] and [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] will be ignored and  **must**  be set dynamically with [`cmd_set_fragment_shading_rate_khr`] or [`cmd_set_fragment_shading_rate_enum_nv`] before any drawing commands.
- [`RAY_TRACING_PIPELINE_STACK_SIZE_KHR`] specifies that the default stack size computation for the pipeline will be ignored and  **must**  be set dynamically with [`cmd_set_ray_tracing_pipeline_stack_size_khr`] before any ray tracing calls are performed.
- [`VERTEX_INPUT_EXT`] specifies that the `pVertexInputState` state will be ignored and  **must**  be set dynamically with [`cmd_set_vertex_input_ext`] before any drawing commands
- [`COLOR_WRITE_ENABLE_EXT`] specifies that the `pColorWriteEnables` state in [`PipelineColorWriteCreateInfoEXT`] will be ignored and  **must**  be set dynamically with [`cmd_set_color_write_enable_ext`] before any draw call.

# Related
- [`crate::vulkan1_0`]
- [`PipelineDynamicStateCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        