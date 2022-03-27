use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_SPEC_VERSION")]
pub const EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 5;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME")]
pub const EXT_VALIDATION_FEATURES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_validation_features");
///[VkValidationFeatureEnableEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureEnableEXT.html) - Specify validation features to enable
///# C Specifications
///Possible values of elements of the
///[`ValidationFeaturesEXT::p_enabled_validation_features`] array,
///specifying validation features to be enabled, are:
///```c
///// Provided by VK_EXT_validation_features
///typedef enum VkValidationFeatureEnableEXT {
///    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT = 0,
///    VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT = 1,
///    VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT = 2,
///    VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT = 3,
///    VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT = 4,
///} VkValidationFeatureEnableEXT;
///```
///# Description
/// - [`ValidationFeatureEnableGpuAssistedExt`] specifies that GPU-assisted validation is enabled.
///   Activating this feature instruments shader programs to generate additional diagnostic data.
///   This feature is disabled by default.
/// - [`ValidationFeatureEnableGpuAssistedReserveBindingSlotExt`] specifies that the validation
///   layers reserve a descriptor set binding slot for their own use. The layer reports a value for
///   [`PhysicalDeviceLimits::max_bound_descriptor_sets`] that is one less than the value reported
///   by the device. If the device supports the binding of only one descriptor set, the validation
///   layer does not perform GPU-assisted validation. This feature is disabled by default.
/// - [`ValidationFeatureEnableBestPracticesExt`] specifies that Vulkan best-practices validation is
///   enabled. Activating this feature enables the output of warnings related to common misuse of
///   the API, but which are not explicitly prohibited by the specification. This feature is
///   disabled by default.
/// - [`ValidationFeatureEnableDebugPrintfExt`] specifies that the layers will process
///   `debugPrintfEXT` operations in shaders and send the resulting output to the debug callback.
///   This feature is disabled by default.
/// - [`ValidationFeatureEnableSynchronizationValidationExt`] specifies that Vulkan synchronization
///   validation is enabled. This feature reports resource access conflicts due to missing or
///   incorrect synchronization operations between actions (Draw, Copy, Dispatch, Blit) reading or
///   writing the same regions of memory. This feature is disabled by default.
///# Related
/// - [`VK_EXT_validation_features`]
/// - [`ValidationFeaturesEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationFeatureEnableEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ValidationFeatureEnableEXT {
    ///[`ValidationFeatureEnableGpuAssistedExt`] specifies that
    ///GPU-assisted validation is enabled.
    ///Activating this feature instruments shader programs to generate
    ///additional diagnostic data.
    ///This feature is disabled by default.
    ValidationFeatureEnableGpuAssistedExt = 0,
    ///[`ValidationFeatureEnableGpuAssistedReserveBindingSlotExt`]
    ///specifies that the validation layers reserve a descriptor set binding
    ///slot for their own use.
    ///The layer reports a value for
    ///[`PhysicalDeviceLimits`]::`maxBoundDescriptorSets` that is one
    ///less than the value reported by the device.
    ///If the device supports the binding of only one descriptor set, the
    ///validation layer does not perform GPU-assisted validation.
    ///This feature is disabled by default.
    ValidationFeatureEnableGpuAssistedReserveBindingSlotExt = 1,
    ///[`ValidationFeatureEnableBestPracticesExt`] specifies that
    ///Vulkan best-practices validation is enabled.
    ///Activating this feature enables the output of warnings related to common
    ///misuse of the API, but which are not explicitly prohibited by the
    ///specification.
    ///This feature is disabled by default.
    ValidationFeatureEnableBestPracticesExt = 2,
    ///[`ValidationFeatureEnableDebugPrintfExt`] specifies that the
    ///layers will process `debugPrintfEXT` operations in shaders and send
    ///the resulting output to the debug callback.
    ///This feature is disabled by default.
    ValidationFeatureEnableDebugPrintfExt = 3,
    ///[`ValidationFeatureEnableSynchronizationValidationExt`]
    ///specifies that Vulkan synchronization validation is enabled.
    ///This feature reports resource access conflicts due to missing or
    ///incorrect synchronization operations between actions (Draw, Copy,
    ///Dispatch, Blit) reading or writing the same regions of memory.
    ///This feature is disabled by default.
    ValidationFeatureEnableSynchronizationValidationExt = 4,
}
impl const Default for ValidationFeatureEnableEXT {
    fn default() -> Self {
        ValidationFeatureEnableGpuAssistedExt
    }
}
impl ValidationFeatureEnableEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkValidationFeatureDisableEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureDisableEXT.html) - Specify validation features to disable
///# C Specifications
///Possible values of elements of the
///[`ValidationFeaturesEXT::p_disabled_validation_features`] array,
///specifying validation features to be disabled, are:
///```c
///// Provided by VK_EXT_validation_features
///typedef enum VkValidationFeatureDisableEXT {
///    VK_VALIDATION_FEATURE_DISABLE_ALL_EXT = 0,
///    VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT = 1,
///    VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT = 2,
///    VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT = 3,
///    VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT = 4,
///    VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT = 5,
///    VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT = 6,
///    VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT = 7,
///} VkValidationFeatureDisableEXT;
///```
///# Description
/// - [`ValidationFeatureDisableAllExt`] specifies that all validation checks are disabled.
/// - [`ValidationFeatureDisableShadersExt`] specifies that shader validation is disabled. This
///   feature is enabled by default.
/// - [`ValidationFeatureDisableThreadSafetyExt`] specifies that thread safety validation is
///   disabled. This feature is enabled by default.
/// - [`ValidationFeatureDisableApiParametersExt`] specifies that stateless parameter validation is
///   disabled. This feature is enabled by default.
/// - [`ValidationFeatureDisableObjectLifetimesExt`] specifies that object lifetime validation is
///   disabled. This feature is enabled by default.
/// - [`ValidationFeatureDisableCoreChecksExt`] specifies that core validation checks are disabled.
///   This feature is enabled by default. If this feature is disabled, the shader validation and
///   GPU-assisted validation features are also disabled.
/// - [`ValidationFeatureDisableUniqueHandlesExt`] specifies that protection against duplicate
///   non-dispatchable object handles is disabled. This feature is enabled by default.
/// - [`ValidationFeatureDisableShaderValidationCacheExt`] specifies that there will be no caching
///   of shader validation results and every shader will be validated on every application
///   execution. Shader validation caching is enabled by default.
///# Related
/// - [`VK_EXT_validation_features`]
/// - [`ValidationFeaturesEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationFeatureDisableEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ValidationFeatureDisableEXT {
    ///[`ValidationFeatureDisableAllExt`] specifies that all
    ///validation checks are disabled.
    ValidationFeatureDisableAllExt = 0,
    ///[`ValidationFeatureDisableShadersExt`] specifies that shader
    ///validation is disabled.
    ///This feature is enabled by default.
    ValidationFeatureDisableShadersExt = 1,
    ///[`ValidationFeatureDisableThreadSafetyExt`] specifies that
    ///thread safety validation is disabled.
    ///This feature is enabled by default.
    ValidationFeatureDisableThreadSafetyExt = 2,
    ///[`ValidationFeatureDisableApiParametersExt`] specifies that
    ///stateless parameter validation is disabled.
    ///This feature is enabled by default.
    ValidationFeatureDisableApiParametersExt = 3,
    ///[`ValidationFeatureDisableObjectLifetimesExt`] specifies that
    ///object lifetime validation is disabled.
    ///This feature is enabled by default.
    ValidationFeatureDisableObjectLifetimesExt = 4,
    ///[`ValidationFeatureDisableCoreChecksExt`] specifies that core
    ///validation checks are disabled.
    ///This feature is enabled by default.
    ///If this feature is disabled, the shader validation and GPU-assisted
    ///validation features are also disabled.
    ValidationFeatureDisableCoreChecksExt = 5,
    ///[`ValidationFeatureDisableUniqueHandlesExt`] specifies that
    ///protection against duplicate non-dispatchable object handles is
    ///disabled.
    ///This feature is enabled by default.
    ValidationFeatureDisableUniqueHandlesExt = 6,
    ///[`ValidationFeatureDisableShaderValidationCacheExt`]
    ///specifies that there will be no caching of shader validation results and
    ///every shader will be validated on every application execution.
    ///Shader validation caching is enabled by default.
    ValidationFeatureDisableShaderValidationCacheExt = 7,
}
impl const Default for ValidationFeatureDisableEXT {
    fn default() -> Self {
        ValidationFeatureDisableAllExt
    }
}
impl ValidationFeatureDisableEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkValidationFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeaturesEXT.html) - Specify validation features to enable or disable for a Vulkan instance
///# C Specifications
///When creating a Vulkan instance for which you wish to enable or disable
///specific validation features, add a [`ValidationFeaturesEXT`] structure
///to the [`p_next`] chain of the [`InstanceCreateInfo`] structure,
///specifying the features to be enabled or disabled.
///```c
///// Provided by VK_EXT_validation_features
///typedef struct VkValidationFeaturesEXT {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    uint32_t                                enabledValidationFeatureCount;
///    const VkValidationFeatureEnableEXT*     pEnabledValidationFeatures;
///    uint32_t                                disabledValidationFeatureCount;
///    const VkValidationFeatureDisableEXT*    pDisabledValidationFeatures;
///} VkValidationFeaturesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`enabled_validation_feature_count`] is the number of features to enable.
/// - [`p_enabled_validation_features`] is a pointer to an array of [`ValidationFeatureEnableEXT`]
///   values specifying the validation features to be enabled.
/// - [`disabled_validation_feature_count`] is the number of features to disable.
/// - [`p_disabled_validation_features`] is a pointer to an array of [`ValidationFeatureDisableEXT`]
///   values specifying the validation features to be disabled.
///# Description
///Valid Usage
/// - If the [`p_enabled_validation_features`] array contains
///   `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT`, then it **must** also
///   contain `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT`
/// - If the [`p_enabled_validation_features`] array contains
///   `VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT`, then it **must** not contain
///   `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`
/// - If [`enabled_validation_feature_count`] is not `0`, [`p_enabled_validation_features`]**must**
///   be a valid pointer to an array of [`enabled_validation_feature_count`] valid
///   [`ValidationFeatureEnableEXT`] values
/// - If [`disabled_validation_feature_count`] is not `0`,
///   [`p_disabled_validation_features`]**must** be a valid pointer to an array of
///   [`disabled_validation_feature_count`] valid [`ValidationFeatureDisableEXT`] values
///# Related
/// - [`VK_EXT_validation_features`]
/// - [`StructureType`]
/// - [`ValidationFeatureDisableEXT`]
/// - [`ValidationFeatureEnableEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ValidationFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`enabled_validation_feature_count`] is the number of features to enable.
    enabled_validation_feature_count: u32,
    ///[`p_enabled_validation_features`] is a pointer to an array of
    ///[`ValidationFeatureEnableEXT`] values specifying the validation
    ///features to be enabled.
    p_enabled_validation_features: *mut ValidationFeatureEnableEXT,
    ///[`disabled_validation_feature_count`] is the number of features to
    ///disable.
    disabled_validation_feature_count: u32,
    ///[`p_disabled_validation_features`] is a pointer to an array of
    ///[`ValidationFeatureDisableEXT`] values specifying the validation
    ///features to be disabled.
    p_disabled_validation_features: *mut ValidationFeatureDisableEXT,
}
