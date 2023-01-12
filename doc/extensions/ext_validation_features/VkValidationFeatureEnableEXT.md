[VkValidationFeatureEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureEnableEXT.html) - Specify validation features to enable

# C Specifications
Possible values of elements of the
[`ValidationFeaturesEXT::enabled_validation_features`] array,
specifying validation features to be enabled, are:
```c
// Provided by VK_EXT_validation_features
typedef enum VkValidationFeatureEnableEXT {
    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT = 0,
    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT = 1,
    VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT = 2,
    VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT = 3,
    VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT = 4,
} VkValidationFeatureEnableEXT;
```

# Description
- [`VK_VALIDATION_FEATURE_ENABLE_EXT`] specifies that GPU-assisted validation is enabled. Activating this feature instruments shader programs to generate additional diagnostic data. This feature is disabled by default.
- [`VK_VALIDATION_FEATURE_ENABLE_EXT`] specifies that the validation layers reserve a descriptor set binding slot for their own use. The layer reports a value for [`PhysicalDeviceLimits::max_bound_descriptor_sets`] that is one less than the value reported by the device. If the device supports the binding of only one descriptor set, the validation layer does not perform GPU-assisted validation. This feature is disabled by default.
- [`VK_VALIDATION_FEATURE_ENABLE_EXT`] specifies that Vulkan best-practices validation is enabled. Activating this feature enables the output of warnings related to common misuse of the API, but which are not explicitly prohibited by the specification. This feature is disabled by default.
- [`VK_VALIDATION_FEATURE_ENABLE_EXT`] specifies that the layers will process `debugPrintfEXT` operations in shaders and send the resulting output to the debug callback. This feature is disabled by default.
- [`VK_VALIDATION_FEATURE_ENABLE_EXT`] specifies that Vulkan synchronization validation is enabled. This feature reports resource access conflicts due to missing or incorrect synchronization operations between actions (Draw, Copy, Dispatch, Blit) reading or writing the same regions of memory. This feature is disabled by default.

# Related
- [`ext_validation_features`]
- [`ValidationFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        