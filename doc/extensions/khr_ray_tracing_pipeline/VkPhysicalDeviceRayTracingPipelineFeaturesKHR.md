[VkPhysicalDeviceRayTracingPipelineFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelineFeaturesKHR.html) - Structure describing the ray tracing features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure is defined
as:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           rayTracingPipeline;
    VkBool32           rayTracingPipelineShaderGroupHandleCaptureReplay;
    VkBool32           rayTracingPipelineShaderGroupHandleCaptureReplayMixed;
    VkBool32           rayTracingPipelineTraceRaysIndirect;
    VkBool32           rayTraversalPrimitiveCulling;
} VkPhysicalDeviceRayTracingPipelineFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`ray_tracing_pipeline`] indicates whether the implementation supports the ray tracing pipeline functionality. See [Ray Tracing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-tracing).
- [`ray_tracing_pipeline_shader_group_handle_capture_replay`] indicates whether the implementation supports saving and reusing shader group handles, e.g. for trace capture and replay.
- [`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] indicates whether the implementation supports reuse of shader group handles being arbitrarily mixed with creation of non-reused shader group handles. If this is `VK_FALSE`, all reused shader group handles  **must**  be specified before any non-reused handles  **may**  be created.
- [`ray_tracing_pipeline_trace_rays_indirect`] indicates whether the implementation supports indirect ray tracing commands, e.g. [`cmd_trace_rays_indirect_khr`].
- [`ray_traversal_primitive_culling`] indicates whether the implementation supports [primitive culling during ray traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-primitive).
If the [`PhysicalDeviceRayTracingPipelineFeaturesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceRayTracingPipelineFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage
-    If [`ray_tracing_pipeline_shader_group_handle_capture_replay_mixed`] is `VK_TRUE`, [`ray_tracing_pipeline_shader_group_handle_capture_replay`] **must**  also be `VK_TRUE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR`

# Related
- [`khr_ray_tracing_pipeline`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        