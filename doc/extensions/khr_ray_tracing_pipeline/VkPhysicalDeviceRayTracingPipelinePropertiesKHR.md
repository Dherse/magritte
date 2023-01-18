[VkPhysicalDeviceRayTracingPipelinePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPipelinePropertiesKHR.html) - Properties of the physical device for ray tracing

# C Specifications
The [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] structure is
defined as:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           shaderGroupHandleSize;
    uint32_t           maxRayRecursionDepth;
    uint32_t           maxShaderGroupStride;
    uint32_t           shaderGroupBaseAlignment;
    uint32_t           shaderGroupHandleCaptureReplaySize;
    uint32_t           maxRayDispatchInvocationCount;
    uint32_t           shaderGroupHandleAlignment;
    uint32_t           maxRayHitAttributeSize;
} VkPhysicalDeviceRayTracingPipelinePropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`shader_group_handle_size`] is the size in bytes of the shader header.
- [`max_ray_recursion_depth`] is the maximum number of levels of ray recursion allowed in a trace command.
- [`max_shader_group_stride`] is the maximum stride in bytes allowed between shader groups in the shader binding table.
- [`shader_group_base_alignment`] is the  **required**  alignment in bytes for the base of the shader binding table.
- [`shader_group_handle_capture_replay_size`] is the number of bytes for the information required to do capture and replay for shader group handles.
- [`max_ray_dispatch_invocation_count`] is the maximum number of ray generation shader invocations which  **may**  be produced by a single [`cmd_trace_rays_indirect_khr`] or [`cmd_trace_rays_khr`] command.
- [`shader_group_handle_alignment`] is the  **required**  alignment in bytes for each shader binding table entry. The value  **must**  be a power of two.
- [`max_ray_hit_attribute_size`] is the maximum size in bytes for a ray attribute structure

# Description
If the [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.Limits specified by this structure  **must**  match those specified with the same
name in [`PhysicalDeviceRayTracingPropertiesNV`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR`

# Related
- [`VK_KHR_ray_tracing_pipeline`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        