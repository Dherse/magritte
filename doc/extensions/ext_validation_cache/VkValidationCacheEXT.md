[VkValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheEXT.html) - Opaque handle to a validation cache object

# C Specifications
Validation cache objects allow the result of internal validation to be
reused, both within a single application run and between multiple runs.
Reuse within a single run is achieved by passing the same validation cache
object when creating supported Vulkan objects.
Reuse across runs of an application is achieved by retrieving validation
cache contents in one run of an application, saving the contents, and using
them to preinitialize a validation cache on a subsequent run.
The contents of the validation cache objects are managed by the validation
layers.
Applications  **can**  manage the host memory consumed by a validation cache
object and control the amount of data retrieved from a validation cache
object.Validation cache objects are represented by [`ValidationCacheEXT`]
handles:
```c
// Provided by VK_EXT_validation_cache
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkValidationCacheEXT)
```

# Related
- [`VK_EXT_validation_cache`]
- [`ShaderModuleValidationCacheCreateInfoEXT`]
- [`create_validation_cache_ext`]
- [`destroy_validation_cache_ext`]
- [`get_validation_cache_data_ext`]
- [`merge_validation_caches_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        