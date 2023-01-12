[VkPipelineCache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCache.html) - Opaque handle to a pipeline cache object

# C Specifications
Pipeline cache objects allow the result of pipeline construction to be
reused between pipelines and between runs of an application.
Reuse between pipelines is achieved by passing the same pipeline cache
object when creating multiple related pipelines.
Reuse across runs of an application is achieved by retrieving pipeline cache
contents in one run of an application, saving the contents, and using them
to preinitialize a pipeline cache on a subsequent run.
The contents of the pipeline cache objects are managed by the
implementation.
Applications  **can**  manage the host memory consumed by a pipeline cache object
and control the amount of data retrieved from a pipeline cache object.Pipeline cache objects are represented by [`PipelineCache`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipelineCache)
```

# Related
- [`crate::vulkan1_0`]
- [`create_compute_pipelines`]
- [`create_graphics_pipelines`]
- [`create_pipeline_cache`]
- [`create_ray_tracing_pipelines_khr`]
- [`create_ray_tracing_pipelines_nv`]
- [`destroy_pipeline_cache`]
- [`get_pipeline_cache_data`]
- [`merge_pipeline_caches`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        