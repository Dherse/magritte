[VkValidationFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeaturesEXT.html) - Specify validation features to enable or disable for a Vulkan instance

# C Specifications
When creating a Vulkan instance for which you wish to enable or disable
specific validation features, add a [`ValidationFeaturesEXT`] structure
to the [`p_next`] chain of the [`InstanceCreateInfo`] structure,
specifying the features to be enabled or disabled.
```c
// Provided by VK_EXT_validation_features
typedef struct VkValidationFeaturesEXT {
    VkStructureType                         sType;
    const void*                             pNext;
    uint32_t                                enabledValidationFeatureCount;
    const VkValidationFeatureEnableEXT*     pEnabledValidationFeatures;
    uint32_t                                disabledValidationFeatureCount;
    const VkValidationFeatureDisableEXT*    pDisabledValidationFeatures;
} VkValidationFeaturesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`enabled_validation_feature_count`] is the number of features to enable.
- [`enabled_validation_features`] is a pointer to an array of [`ValidationFeatureEnableEXT`] values specifying the validation features to be enabled.
- [`disabled_validation_feature_count`] is the number of features to disable.
- [`disabled_validation_features`] is a pointer to an array of [`ValidationFeatureDisableEXT`] values specifying the validation features to be disabled.

# Description
## Valid Usage
-    If the [`enabled_validation_features`] array contains `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT`, then it  **must**  also contain `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT`
-    If the [`enabled_validation_features`] array contains `VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT`, then it  **must**  not contain `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`
-    If [`enabled_validation_feature_count`] is not `0`, [`enabled_validation_features`] **must**  be a valid pointer to an array of [`enabled_validation_feature_count`] valid [`ValidationFeatureEnableEXT`] values
-    If [`disabled_validation_feature_count`] is not `0`, [`disabled_validation_features`] **must**  be a valid pointer to an array of [`disabled_validation_feature_count`] valid [`ValidationFeatureDisableEXT`] values

# Related
- [`VK_EXT_validation_features`]
- [`StructureType`]
- [`ValidationFeatureDisableEXT`]
- [`ValidationFeatureEnableEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        