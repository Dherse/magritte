[vkCompileDeferredNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html) - Deferred compilation of shaders

# C Specifications
To compile a deferred shader in a pipeline call:
```c
// Provided by VK_NV_ray_tracing
VkResult vkCompileDeferredNV(
    VkDevice                                    device,
    VkPipeline                                  pipeline,
    uint32_t                                    shader);
```

# Parameters
- [`device`] is the logical device containing the ray tracing pipeline.
- [`pipeline`] is the ray tracing pipeline object containing the shaders.
- [`shader`] is the index of the shader to compile.

# Description
## Valid Usage
-  [`pipeline`] **must**  be a ray tracing pipeline
-  [`pipeline`] **must**  have been created with `VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV`
-  [`shader`] **must**  not have been called as a deferred compile before

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`pipeline`] **must**  be a valid [`Pipeline`] handle
-  [`pipeline`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`VK_NV_ray_tracing`]
- [`Device`]
- [`Pipeline`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        