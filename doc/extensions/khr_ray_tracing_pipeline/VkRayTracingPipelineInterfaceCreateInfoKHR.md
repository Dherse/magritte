[VkRayTracingPipelineInterfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRayTracingPipelineInterfaceCreateInfoKHR.html) - Structure specifying additional interface information when using libraries

# C Specifications
The [`RayTracingPipelineInterfaceCreateInfoKHR`] structure is defined
as:
```c
// Provided by VK_KHR_ray_tracing_pipeline
typedef struct VkRayTracingPipelineInterfaceCreateInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           maxPipelineRayPayloadSize;
    uint32_t           maxPipelineRayHitAttributeSize;
} VkRayTracingPipelineInterfaceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_pipeline_ray_payload_size`] is the maximum payload size in bytes used by any shader in the pipeline.
- [`max_pipeline_ray_hit_attribute_size`] is the maximum attribute structure size in bytes used by any shader in the pipeline.

# Description
[`max_pipeline_ray_payload_size`] is calculated as the maximum number of bytes
used by any block declared in the `RayPayloadKHR` or
`IncomingRayPayloadKHR` storage classes.
[`max_pipeline_ray_hit_attribute_size`] is calculated as the maximum number of
bytes used by any block declared in the `HitAttributeKHR` storage class.
As variables in these storage classes do not have explicit offsets, the size
should be calculated as if each variable has a
[scalar alignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-alignment-requirements) equal to the largest
scalar alignment of any of the block’s members.
## Valid Usage
-  [`max_pipeline_ray_hit_attribute_size`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPipelinePropertiesKHR::max_ray_hit_attribute_size`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`khr_ray_tracing_pipeline`]
- [`RayTracingPipelineCreateInfoKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        