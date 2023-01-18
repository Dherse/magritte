[VkValidationFeatureDisableEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureDisableEXT.html) - Specify validation features to disable

# C Specifications
Possible values of elements of the
[`ValidationFeaturesEXT::disabled_validation_features`] array,
specifying validation features to be disabled, are:
```c
// Provided by VK_EXT_validation_features
typedef enum VkValidationFeatureDisableEXT {
    VK_VALIDATION_FEATURE_DISABLE_ALL_EXT = 0,
    VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT = 1,
    VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT = 2,
    VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT = 3,
    VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT = 4,
    VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT = 5,
    VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT = 6,
    VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT = 7,
} VkValidationFeatureDisableEXT;
```

# Description
- [`ALL`] specifies that all validation checks are disabled.
- [`SHADERS`] specifies that shader validation is disabled. This feature is enabled by default.
- [`THREAD_SAFETY`] specifies that thread safety validation is disabled. This feature is enabled by default.
- [`API_PARAMETERS`] specifies that stateless parameter validation is disabled. This feature is enabled by default.
- [`OBJECT_LIFETIMES`] specifies that object lifetime validation is disabled. This feature is enabled by default.
- [`CORE_CHECKS`] specifies that core validation checks are disabled. This feature is enabled by default. If this feature is disabled, the shader validation and GPU-assisted validation features are also disabled.
- [`UNIQUE_HANDLES`] specifies that protection against duplicate non-dispatchable object handles is disabled. This feature is enabled by default.
- [`SHADER_VALIDATION_CACHE`] specifies that there will be no caching of shader validation results and every shader will be validated on every application execution. Shader validation caching is enabled by default.

# Related
- [`VK_EXT_validation_features`]
- [`ValidationFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        