[vkGetRayTracingCaptureReplayShaderGroupHandlesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html) - Query ray tracing capture replay pipeline shader group handles

# C Specifications
To query the optional capture handle information of shaders in the ray
tracing pipeline, call:
```c
// Provided by VK_KHR_ray_tracing_pipeline
VkResult vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
    VkDevice                                    device,
    VkPipeline                                  pipeline,
    uint32_t                                    firstGroup,
    uint32_t                                    groupCount,
    size_t                                      dataSize,
    void*                                       pData);
```

# Parameters
- [`device`] is the logical device containing the ray tracing pipeline.
- [`pipeline`] is the ray tracing pipeline object containing the shaders.
- [`first_group`] is the index of the first group to retrieve a handle for from the [`RayTracingPipelineCreateInfoKHR::groups`] array.
- [`group_count`] is the number of shader handles to retrieve.
- [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
- [`p_data`] is a pointer to a user-allocated buffer where the results will be written.

# Description
## Valid Usage
-  [`pipeline`] **must**  be a ray tracing pipeline
-  [`first_group`] **must**  be less than the number of shader groups in [`pipeline`]
-    The sum of [`first_group`] and [`group_count`] **must**  be less than or equal to the number of shader groups in [`pipeline`]
-  [`data_size`] **must**  be at least [`PhysicalDeviceRayTracingPipelinePropertiesKHR::shader_group_handle_capture_replay_size`] × [`group_count`]
-  [`PhysicalDeviceRayTracingPipelineFeaturesKHR::ray_tracing_pipeline_shader_group_handle_capture_replay`] **must**  be enabled to call this function
-  [`pipeline`] **must**  have been created with a `flags` that included `VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
-  [`data_size`] **must**  be greater than `0`
-  [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_ray_tracing_pipeline`]
- [`Device`]
- [`Pipeline`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        