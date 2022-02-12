//![VK_EXT_validation_features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_features.html) - instance extension
//!# Description
//!This extension provides the [`ValidationFeaturesEXT`] struct that can be
//!included in the `pNext` chain of the [`InstanceCreateInfo`]
//!structure passed as the `pCreateInfo` parameter of
//![`CreateInstance`].
//!The structure contains an array of [`ValidationFeatureEnableEXT`] enum
//!values that enable specific validation features that are disabled by
//!default.
//!The structure also contains an array of [`ValidationFeatureDisableEXT`]
//!enum values that disable specific validation layer features that are enabled
//!by default.
//!# Revision
//!5
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Karl Schultz [karl-lunarg](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_features]
//!   @karl-lunarg%0A<<Here describe the issue or question you have about the
//!   VK_EXT_validation_features extension>>)
//!# New structures
//! - Extending [`InstanceCreateInfo`]:
//! - [`ValidationFeaturesEXT`]
//!# New enums
//! - [`ValidationFeatureDisableEXT`]
//! - [`ValidationFeatureEnableEXT`]
//!# New constants
//! - [`EXT_VALIDATION_FEATURES_EXTENSION_NAME`]
//! - [`EXT_VALIDATION_FEATURES_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2018-11-14 (Karl Schultz)
//! - Initial revision
//! - Revision 2, 2019-08-06 (Mark Lobodzinski)
//! - Add Best Practices enable
//! - Revision 3, 2020-03-04 (Tony Barbour)
//! - Add Debug Printf enable
//! - Revision 4, 2020-07-29 (John Zulauf)
//! - Add Synchronization Validation enable
//! - Revision 5, 2021-05-18 (Tony Barbour)
//! - Add Shader Validation Cache disable
//!# Other info
//! * 2018-11-14
//! * No known IP claims.
//!*
//! - Karl Schultz, LunarG
//! - Dave Houlton, LunarG
//! - Mark Lobodzinski, LunarG
//! - Camden Stocker, LunarG
//! - Tony Barbour, LunarG
//! - John Zulauf, LunarG
//!# Related
//! - [`ValidationFeatureDisableEXT`]
//! - [`ValidationFeatureEnableEXT`]
//! - [`ValidationFeaturesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
/// - [`VALIDATION_FEATURE_ENABLE_GPU_ASSISTED`] specifies that
///GPU-assisted validation is enabled.
///Activating this feature instruments shader programs to generate
///additional diagnostic data.
///This feature is disabled by default.
/// - [`VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT`]
///specifies that the validation layers reserve a descriptor set binding
///slot for their own use.
///The layer reports a value for
///[`PhysicalDeviceLimits::max_bound_descriptor_sets`] that is one
///less than the value reported by the device.
///If the device supports the binding of only one descriptor set, the
///validation layer does not perform GPU-assisted validation.
///This feature is disabled by default.
/// - [`VALIDATION_FEATURE_ENABLE_BEST_PRACTICES`] specifies that
///Vulkan best-practices validation is enabled.
///Activating this feature enables the output of warnings related to common
///misuse of the API, but which are not explicitly prohibited by the
///specification.
///This feature is disabled by default.
/// - [`VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF`] specifies that the
///layers will process `debugPrintfEXT` operations in shaders and send
///the resulting output to the debug callback.
///This feature is disabled by default.
/// - [`VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION`]
///specifies that Vulkan synchronization validation is enabled.
///This feature reports resource access conflicts due to missing or
///incorrect synchronization operations between actions (Draw, Copy,
///Dispatch, Blit) reading or writing the same regions of memory.
///This feature is disabled by default.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ValidationFeatureEnableEXT(i32);
impl const Default for ValidationFeatureEnableEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ValidationFeatureEnableEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ValidationFeatureEnableEXT")
            .field(match *self {
                Self::VALIDATION_FEATURE_ENABLE_GPU_ASSISTED => &"VALIDATION_FEATURE_ENABLE_GPU_ASSISTED",
                Self::VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT => {
                    &"VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT"
                },
                Self::VALIDATION_FEATURE_ENABLE_BEST_PRACTICES => &"VALIDATION_FEATURE_ENABLE_BEST_PRACTICES",
                Self::VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF => &"VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF",
                Self::VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION => {
                    &"VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION"
                },
                other => unreachable!("invalid value for `ValidationFeatureEnableEXT`: {:?}", other),
            })
            .finish()
    }
}
impl ValidationFeatureEnableEXT {
    ///[`VALIDATION_FEATURE_ENABLE_GPU_ASSISTED`] specifies that
    ///GPU-assisted validation is enabled.
    ///Activating this feature instruments shader programs to generate
    ///additional diagnostic data.
    ///This feature is disabled by default.
    pub const VALIDATION_FEATURE_ENABLE_GPU_ASSISTED: Self = Self(0);
    ///[`VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT`]
    ///specifies that the validation layers reserve a descriptor set binding
    ///slot for their own use.
    ///The layer reports a value for
    ///[`PhysicalDeviceLimits`]::`maxBoundDescriptorSets` that is one
    ///less than the value reported by the device.
    ///If the device supports the binding of only one descriptor set, the
    ///validation layer does not perform GPU-assisted validation.
    ///This feature is disabled by default.
    pub const VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = Self(1);
    ///[`VALIDATION_FEATURE_ENABLE_BEST_PRACTICES`] specifies that
    ///Vulkan best-practices validation is enabled.
    ///Activating this feature enables the output of warnings related to common
    ///misuse of the API, but which are not explicitly prohibited by the
    ///specification.
    ///This feature is disabled by default.
    pub const VALIDATION_FEATURE_ENABLE_BEST_PRACTICES: Self = Self(2);
    ///[`VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF`] specifies that the
    ///layers will process `debugPrintfEXT` operations in shaders and send
    ///the resulting output to the debug callback.
    ///This feature is disabled by default.
    pub const VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF: Self = Self(3);
    ///[`VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION`]
    ///specifies that Vulkan synchronization validation is enabled.
    ///This feature reports resource access conflicts due to missing or
    ///incorrect synchronization operations between actions (Draw, Copy,
    ///Dispatch, Blit) reading or writing the same regions of memory.
    ///This feature is disabled by default.
    pub const VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`VALIDATION_FEATURE_DISABLE_ALL`] specifies that all
///validation checks are disabled.
/// - [`VALIDATION_FEATURE_DISABLE_SHADERS`] specifies that shader
///validation is disabled.
///This feature is enabled by default.
/// - [`VALIDATION_FEATURE_DISABLE_THREAD_SAFETY`] specifies that
///thread safety validation is disabled.
///This feature is enabled by default.
/// - [`VALIDATION_FEATURE_DISABLE_API_PARAMETERS`] specifies that
///stateless parameter validation is disabled.
///This feature is enabled by default.
/// - [`VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES`] specifies that
///object lifetime validation is disabled.
///This feature is enabled by default.
/// - [`VALIDATION_FEATURE_DISABLE_CORE_CHECKS`] specifies that core
///validation checks are disabled.
///This feature is enabled by default.
///If this feature is disabled, the shader validation and GPU-assisted
///validation features are also disabled.
/// - [`VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES`] specifies that
///protection against duplicate non-dispatchable object handles is
///disabled.
///This feature is enabled by default.
/// - [`VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE`]
///specifies that there will be no caching of shader validation results and
///every shader will be validated on every application execution.
///Shader validation caching is enabled by default.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ValidationFeatureDisableEXT(i32);
impl const Default for ValidationFeatureDisableEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ValidationFeatureDisableEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ValidationFeatureDisableEXT")
            .field(match *self {
                Self::VALIDATION_FEATURE_DISABLE_ALL => &"VALIDATION_FEATURE_DISABLE_ALL",
                Self::VALIDATION_FEATURE_DISABLE_SHADERS => &"VALIDATION_FEATURE_DISABLE_SHADERS",
                Self::VALIDATION_FEATURE_DISABLE_THREAD_SAFETY => &"VALIDATION_FEATURE_DISABLE_THREAD_SAFETY",
                Self::VALIDATION_FEATURE_DISABLE_API_PARAMETERS => &"VALIDATION_FEATURE_DISABLE_API_PARAMETERS",
                Self::VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES => &"VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES",
                Self::VALIDATION_FEATURE_DISABLE_CORE_CHECKS => &"VALIDATION_FEATURE_DISABLE_CORE_CHECKS",
                Self::VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES => &"VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES",
                Self::VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE => {
                    &"VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE"
                },
                other => unreachable!("invalid value for `ValidationFeatureDisableEXT`: {:?}", other),
            })
            .finish()
    }
}
impl ValidationFeatureDisableEXT {
    ///[`VALIDATION_FEATURE_DISABLE_ALL`] specifies that all
    ///validation checks are disabled.
    pub const VALIDATION_FEATURE_DISABLE_ALL: Self = Self(0);
    ///[`VALIDATION_FEATURE_DISABLE_SHADERS`] specifies that shader
    ///validation is disabled.
    ///This feature is enabled by default.
    pub const VALIDATION_FEATURE_DISABLE_SHADERS: Self = Self(1);
    ///[`VALIDATION_FEATURE_DISABLE_THREAD_SAFETY`] specifies that
    ///thread safety validation is disabled.
    ///This feature is enabled by default.
    pub const VALIDATION_FEATURE_DISABLE_THREAD_SAFETY: Self = Self(2);
    ///[`VALIDATION_FEATURE_DISABLE_API_PARAMETERS`] specifies that
    ///stateless parameter validation is disabled.
    ///This feature is enabled by default.
    pub const VALIDATION_FEATURE_DISABLE_API_PARAMETERS: Self = Self(3);
    ///[`VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES`] specifies that
    ///object lifetime validation is disabled.
    ///This feature is enabled by default.
    pub const VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES: Self = Self(4);
    ///[`VALIDATION_FEATURE_DISABLE_CORE_CHECKS`] specifies that core
    ///validation checks are disabled.
    ///This feature is enabled by default.
    ///If this feature is disabled, the shader validation and GPU-assisted
    ///validation features are also disabled.
    pub const VALIDATION_FEATURE_DISABLE_CORE_CHECKS: Self = Self(5);
    ///[`VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES`] specifies that
    ///protection against duplicate non-dispatchable object handles is
    ///disabled.
    ///This feature is enabled by default.
    pub const VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES: Self = Self(6);
    ///[`VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE`]
    ///specifies that there will be no caching of shader validation results and
    ///every shader will be validated on every application execution.
    ///Shader validation caching is enabled by default.
    pub const VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE: Self = Self(7);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
