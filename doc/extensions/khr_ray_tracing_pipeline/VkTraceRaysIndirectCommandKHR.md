[VkTraceRaysIndirectCommandKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTraceRaysIndirectCommandKHR.html) - Structure specifying the parameters of an indirect ray tracing command

# C Specifications
The [`TraceRaysIndirectCommandKHR`] structure is defined as:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef struct VkTraceRaysIndirectCommandKHR {
    uint32_t    width;
    uint32_t    height;
    uint32_t    depth;
} VkTraceRaysIndirectCommandKHR;
```

# Members
- [`width`] is the width of the ray trace query dimensions.
- [`height`] is height of the ray trace query dimensions.
- [`depth`] is depth of the ray trace query dimensions.

# Description
The members of [`TraceRaysIndirectCommandKHR`] have the same meaning as
the similarly named parameters of [`cmd_trace_rays_khr`].
## Valid Usage
-  [`width`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][0] × [`PhysicalDeviceLimits::max_compute_work_group_size`][0]
-  [`height`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][1] × [`PhysicalDeviceLimits::max_compute_work_group_size`][1]
-  [`depth`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_compute_work_group_count`][2] × [`PhysicalDeviceLimits::max_compute_work_group_size`][2]
-  [`width`] × [`height`] × [`depth`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_dispatch_invocation_count`]

# Related
- [`khr_ray_tracing_pipeline`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        