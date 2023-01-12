[VkPipelineCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreateFlagBits.html) - Bitmask controlling how a pipeline is created

# C Specifications
Bits which  **can**  be set in
  * [`GraphicsPipelineCreateInfo::flags`]
  * [`ComputePipelineCreateInfo::flags`]
  * [`RayTracingPipelineCreateInfoKHR::flags`]
  * [`RayTracingPipelineCreateInfoNV::flags`]
specify how a pipeline is created, and are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkPipelineCreateFlagBits {
    VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x00000001,
    VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x00000002,
    VK_PIPELINE_CREATE_DERIVATIVE_BIT = 0x00000004,
  // Provided by VK_VERSION_1_1
    VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 0x00000008,
  // Provided by VK_VERSION_1_1
    VK_PIPELINE_CREATE_DISPATCH_BASE_BIT = 0x00000010,
  // Provided by VK_VERSION_1_3
    VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT = 0x00000100,
  // Provided by VK_VERSION_1_3
    VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT = 0x00000200,
  // Provided by VK_KHR_dynamic_rendering with VK_KHR_fragment_shading_rate
    VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00200000,
  // Provided by VK_KHR_dynamic_rendering with VK_EXT_fragment_density_map
    VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 0x00400000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR = 0x00004000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR = 0x00008000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR = 0x00010000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR = 0x00020000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR = 0x00001000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR = 0x00002000,
  // Provided by VK_KHR_ray_tracing_pipeline
    VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR = 0x00080000,
  // Provided by VK_NV_ray_tracing
    VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV = 0x00000020,
  // Provided by VK_KHR_pipeline_executable_properties
    VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR = 0x00000040,
  // Provided by VK_KHR_pipeline_executable_properties
    VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR = 0x00000080,
  // Provided by VK_NV_device_generated_commands
    VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV = 0x00040000,
  // Provided by VK_KHR_pipeline_library
    VK_PIPELINE_CREATE_LIBRARY_BIT_KHR = 0x00000800,
  // Provided by VK_NV_ray_tracing_motion_blur
    VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV = 0x00100000,
  // Provided by VK_VERSION_1_1
    VK_PIPELINE_CREATE_DISPATCH_BASE = VK_PIPELINE_CREATE_DISPATCH_BASE_BIT,
  // Provided by VK_KHR_dynamic_rendering with VK_KHR_fragment_shading_rate
    VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR,
  // Provided by VK_KHR_dynamic_rendering with VK_EXT_fragment_density_map
    VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT,
  // Provided by VK_KHR_device_group
    VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR = VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT,
  // Provided by VK_KHR_device_group
    VK_PIPELINE_CREATE_DISPATCH_BASE_KHR = VK_PIPELINE_CREATE_DISPATCH_BASE,
  // Provided by VK_EXT_pipeline_creation_cache_control
    VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT = VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT,
  // Provided by VK_EXT_pipeline_creation_cache_control
    VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT = VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT,
} VkPipelineCreateFlagBits;
```

# Description
- [`VK_PIPELINE_CREATE_FLAG_BITS`] specifies that the created pipeline will not be optimized. Using this flag  **may**  reduce the time taken to create the pipeline.
- [`VK_PIPELINE_CREATE_FLAG_BITS`] specifies that the pipeline to be created is allowed to be the parent of a pipeline that will be created in a subsequent pipeline creation call.
- [`VK_PIPELINE_CREATE_FLAG_BITS`] specifies that the pipeline to be created will be a child of a previously created parent pipeline.
- [`VIEW_INDEX_FROM_DEVICE_INDEX`] specifies that any shader input variables decorated as `ViewIndex` will be assigned values as if they were decorated as `DeviceIndex`.
- [`DISPATCH_BASE`] specifies that a compute pipeline  **can**  be used with [`cmd_dispatch_base`] with a non-zero base workgroup.
- [`DEFER_COMPILE_NV`] specifies that a pipeline is created with all shaders in the deferred state. Before using the pipeline the application  **must**  call [`compile_deferred_nv`] exactly once on each shader in the pipeline before using the pipeline.
- [`CAPTURE_STATISTICS_KHR`] specifies that the shader compiler should capture statistics for the pipeline executables produced by the compile process which  **can**  later be retrieved by calling [`get_pipeline_executable_statistics_khr`]. Enabling this flag  **must**  not affect the final compiled pipeline but  **may**  disable pipeline caching or otherwise affect pipeline creation time.
- [`CAPTURE_INTERNAL_REPRESENTATIONS_KHR`] specifies that the shader compiler should capture the internal representations of pipeline executables produced by the compile process which  **can**  later be retrieved by calling [`get_pipeline_executable_internal_representations_khr`]. Enabling this flag  **must**  not affect the final compiled pipeline but  **may**  disable pipeline caching or otherwise affect pipeline creation time.
- [`LIBRARY_KHR`] specifies that the pipeline  **cannot**  be used directly, and instead defines a *pipeline library* that  **can**  be combined with other pipelines using the [`PipelineLibraryCreateInfoKHR`] structure. This is available in ray tracing pipelines.
- [`RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR`] specifies that an any-hit shader will always be present when an any-hit shader would be executed. A NULL any-hit shader is an any-hit shader which is effectively `VK_SHADER_UNUSED_KHR`, such as from a shader group consisting entirely of zeros.
- [`RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR`] specifies that a closest hit shader will always be present when a closest hit shader would be executed. A NULL closest hit shader is a closest hit shader which is effectively `VK_SHADER_UNUSED_KHR`, such as from a shader group consisting entirely of zeros.
- [`RAY_TRACING_NO_NULL_MISS_SHADERS_KHR`] specifies that a miss shader will always be present when a miss shader would be executed. A NULL miss shader is a miss shader which is effectively `VK_SHADER_UNUSED_KHR`, such as from a shader group consisting entirely of zeros.
- [`RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR`] specifies that an intersection shader will always be present when an intersection shader would be executed. A NULL intersection shader is an intersection shader which is effectively `VK_SHADER_UNUSED_KHR`, such as from a shader group consisting entirely of zeros.
- [`RAY_TRACING_SKIP_TRIANGLES_KHR`] specifies that triangle primitives will be skipped during traversal using `OpTraceRayKHR`.
- [`RAY_TRACING_SKIP_AABBS_KHR`] specifies that AABB primitives will be skipped during traversal using `OpTraceRayKHR`.
- [`RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR`] specifies that the shader group handles  **can**  be saved and reused on a subsequent run (e.g. for trace capture and replay).
- [`INDIRECT_BINDABLE_NV`] specifies that the pipeline can be used in combination with [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-generated-commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-generated-commands).
- [`FAIL_ON_PIPELINE_COMPILE_REQUIRED`] specifies that pipeline creation will fail if a compile is required for creation of a valid [`Pipeline`] object; `VK_PIPELINE_COMPILE_REQUIRED` will be returned by pipeline creation, and the [`Pipeline`] will be set to [`crate::Handle::null`].
- When creating multiple pipelines, [`EARLY_RETURN_ON_FAILURE`] specifies that control will be returned to the application on failure of the corresponding pipeline rather than continuing to create additional pipelines.
- [`RAY_TRACING_ALLOW_MOTION_NV`] specifies that the pipeline is allowed to use `OpTraceRayMotionNV`.
- [`RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR`] specifies that the pipeline will be used with a fragment shading rate attachment.
- [`RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT`] specifies that the pipeline will be used with a fragment density map attachment.
It is valid to set both [`VK_PIPELINE_CREATE_FLAG_BITS`] and
[`VK_PIPELINE_CREATE_FLAG_BITS`].
This allows a pipeline to be both a parent and possibly a child in a
pipeline hierarchy.
See [Pipeline Derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-pipeline-derivatives) for more
information.

# Related
- [`crate::vulkan1_0`]
- [VkPipelineCreateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        