//![VK_EXT_validation_features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_features.html) - instance extension
//!# Description
//!This extension provides the [`ValidationFeaturesEXT`] struct that can be
//!included in the `pNext` chain of the [`InstanceCreateInfo`]
//!structure passed as the `pCreateInfo` parameter of
//![`create_instance`].
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
//! - Extending [`InstanceCreateInfo`]:  - [`ValidationFeaturesEXT`]
//!# New enums
//! - [`ValidationFeatureDisableEXT`]
//! - [`ValidationFeatureEnableEXT`]
//!# New constants
//! - [`EXT_VALIDATION_FEATURES_EXTENSION_NAME`]
//! - [`EXT_VALIDATION_FEATURES_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2018-11-14 (Karl Schultz)  - Initial revision
//! - Revision 2, 2019-08-06 (Mark Lobodzinski)  - Add Best Practices enable
//! - Revision 3, 2020-03-04 (Tony Barbour)  - Add Debug Printf enable
//! - Revision 4, 2020-07-29 (John Zulauf)  - Add Synchronization Validation enable
//! - Revision 5, 2021-05-18 (Tony Barbour)  - Add Shader Validation Cache disable
//!# Other info
//! * 2018-11-14
//! * No known IP claims.
//! * - Karl Schultz, LunarG  - Dave Houlton, LunarG  - Mark Lobodzinski, LunarG  - Camden Stocker,
//!   LunarG  - Tony Barbour, LunarG  - John Zulauf, LunarG
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
use crate::vulkan1_0::{BaseInStructure, StructureType};
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
///[`ValidationFeaturesEXT::enabled_validation_features`] array,
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
/// - [`GPU_ASSISTED`] specifies that GPU-assisted validation is enabled. Activating this feature
///   instruments shader programs to generate additional diagnostic data. This feature is disabled
///   by default.
/// - [`GPU_ASSISTED_RESERVE_BINDING_SLOT`] specifies that the validation layers reserve a
///   descriptor set binding slot for their own use. The layer reports a value for
///   [`PhysicalDeviceLimits::max_bound_descriptor_sets`] that is one less than the value reported
///   by the device. If the device supports the binding of only one descriptor set, the validation
///   layer does not perform GPU-assisted validation. This feature is disabled by default.
/// - [`BEST_PRACTICES`] specifies that Vulkan best-practices validation is enabled. Activating this
///   feature enables the output of warnings related to common misuse of the API, but which are not
///   explicitly prohibited by the specification. This feature is disabled by default.
/// - [`DEBUG_PRINTF`] specifies that the layers will process `debugPrintfEXT` operations in shaders
///   and send the resulting output to the debug callback. This feature is disabled by default.
/// - [`SYNCHRONIZATION_VALIDATION`] specifies that Vulkan synchronization validation is enabled.
///   This feature reports resource access conflicts due to missing or incorrect synchronization
///   operations between actions (Draw, Copy, Dispatch, Blit) reading or writing the same regions of
///   memory. This feature is disabled by default.
///# Related
/// - [`ext_validation_features`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ValidationFeatureEnableEXT(i32);
impl const Default for ValidationFeatureEnableEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ValidationFeatureEnableEXT {
    ///[`GPU_ASSISTED`] specifies that
    ///GPU-assisted validation is enabled.
    ///Activating this feature instruments shader programs to generate
    ///additional diagnostic data.
    ///This feature is disabled by default.
    pub const GPU_ASSISTED: Self = Self(0);
    ///[`GPU_ASSISTED_RESERVE_BINDING_SLOT`]
    ///specifies that the validation layers reserve a descriptor set binding
    ///slot for their own use.
    ///The layer reports a value for
    ///[`PhysicalDeviceLimits`]::`maxBoundDescriptorSets` that is one
    ///less than the value reported by the device.
    ///If the device supports the binding of only one descriptor set, the
    ///validation layer does not perform GPU-assisted validation.
    ///This feature is disabled by default.
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = Self(1);
    ///[`BEST_PRACTICES`] specifies that
    ///Vulkan best-practices validation is enabled.
    ///Activating this feature enables the output of warnings related to common
    ///misuse of the API, but which are not explicitly prohibited by the
    ///specification.
    ///This feature is disabled by default.
    pub const BEST_PRACTICES: Self = Self(2);
    ///[`DEBUG_PRINTF`] specifies that the
    ///layers will process `debugPrintfEXT` operations in shaders and send
    ///the resulting output to the debug callback.
    ///This feature is disabled by default.
    pub const DEBUG_PRINTF: Self = Self(3);
    ///[`SYNCHRONIZATION_VALIDATION`]
    ///specifies that Vulkan synchronization validation is enabled.
    ///This feature reports resource access conflicts due to missing or
    ///incorrect synchronization operations between actions (Draw, Copy,
    ///Dispatch, Blit) reading or writing the same regions of memory.
    ///This feature is disabled by default.
    pub const SYNCHRONIZATION_VALIDATION: Self = Self(4);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for ValidationFeatureEnableEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ValidationFeatureEnableEXT))
            .field(match *self {
                Self::GPU_ASSISTED => &"GPU_ASSISTED",
                Self::GPU_ASSISTED_RESERVE_BINDING_SLOT => &"GPU_ASSISTED_RESERVE_BINDING_SLOT",
                Self::BEST_PRACTICES => &"BEST_PRACTICES",
                Self::DEBUG_PRINTF => &"DEBUG_PRINTF",
                Self::SYNCHRONIZATION_VALIDATION => &"SYNCHRONIZATION_VALIDATION",
                other => unreachable!(
                    concat!("invalid value for", stringify!(ValidationFeatureEnableEXT), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for ValidationFeatureEnableEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::GPU_ASSISTED => &"GPU_ASSISTED",
            Self::GPU_ASSISTED_RESERVE_BINDING_SLOT => &"GPU_ASSISTED_RESERVE_BINDING_SLOT",
            Self::BEST_PRACTICES => &"BEST_PRACTICES",
            Self::DEBUG_PRINTF => &"DEBUG_PRINTF",
            Self::SYNCHRONIZATION_VALIDATION => &"SYNCHRONIZATION_VALIDATION",
            other => unreachable!(
                concat!("invalid value for", stringify!(ValidationFeatureEnableEXT), ": {:?}"),
                other
            ),
        })
    }
}
///[VkValidationFeatureDisableEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFeatureDisableEXT.html) - Specify validation features to disable
///# C Specifications
///Possible values of elements of the
///[`ValidationFeaturesEXT::disabled_validation_features`] array,
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
/// - [`ALL`] specifies that all validation checks are disabled.
/// - [`SHADERS`] specifies that shader validation is disabled. This feature is enabled by default.
/// - [`THREAD_SAFETY`] specifies that thread safety validation is disabled. This feature is enabled
///   by default.
/// - [`API_PARAMETERS`] specifies that stateless parameter validation is disabled. This feature is
///   enabled by default.
/// - [`OBJECT_LIFETIMES`] specifies that object lifetime validation is disabled. This feature is
///   enabled by default.
/// - [`CORE_CHECKS`] specifies that core validation checks are disabled. This feature is enabled by
///   default. If this feature is disabled, the shader validation and GPU-assisted validation
///   features are also disabled.
/// - [`UNIQUE_HANDLES`] specifies that protection against duplicate non-dispatchable object handles
///   is disabled. This feature is enabled by default.
/// - [`SHADER_VALIDATION_CACHE`] specifies that there will be no caching of shader validation
///   results and every shader will be validated on every application execution. Shader validation
///   caching is enabled by default.
///# Related
/// - [`ext_validation_features`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ValidationFeatureDisableEXT(i32);
impl const Default for ValidationFeatureDisableEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ValidationFeatureDisableEXT {
    ///[`ALL`] specifies that all
    ///validation checks are disabled.
    pub const ALL: Self = Self(0);
    ///[`SHADERS`] specifies that shader
    ///validation is disabled.
    ///This feature is enabled by default.
    pub const SHADERS: Self = Self(1);
    ///[`THREAD_SAFETY`] specifies that
    ///thread safety validation is disabled.
    ///This feature is enabled by default.
    pub const THREAD_SAFETY: Self = Self(2);
    ///[`API_PARAMETERS`] specifies that
    ///stateless parameter validation is disabled.
    ///This feature is enabled by default.
    pub const API_PARAMETERS: Self = Self(3);
    ///[`OBJECT_LIFETIMES`] specifies that
    ///object lifetime validation is disabled.
    ///This feature is enabled by default.
    pub const OBJECT_LIFETIMES: Self = Self(4);
    ///[`CORE_CHECKS`] specifies that core
    ///validation checks are disabled.
    ///This feature is enabled by default.
    ///If this feature is disabled, the shader validation and GPU-assisted
    ///validation features are also disabled.
    pub const CORE_CHECKS: Self = Self(5);
    ///[`UNIQUE_HANDLES`] specifies that
    ///protection against duplicate non-dispatchable object handles is
    ///disabled.
    ///This feature is enabled by default.
    pub const UNIQUE_HANDLES: Self = Self(6);
    ///[`SHADER_VALIDATION_CACHE`]
    ///specifies that there will be no caching of shader validation results and
    ///every shader will be validated on every application execution.
    ///Shader validation caching is enabled by default.
    pub const SHADER_VALIDATION_CACHE: Self = Self(7);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for ValidationFeatureDisableEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(ValidationFeatureDisableEXT))
            .field(match *self {
                Self::ALL => &"ALL",
                Self::SHADERS => &"SHADERS",
                Self::THREAD_SAFETY => &"THREAD_SAFETY",
                Self::API_PARAMETERS => &"API_PARAMETERS",
                Self::OBJECT_LIFETIMES => &"OBJECT_LIFETIMES",
                Self::CORE_CHECKS => &"CORE_CHECKS",
                Self::UNIQUE_HANDLES => &"UNIQUE_HANDLES",
                Self::SHADER_VALIDATION_CACHE => &"SHADER_VALIDATION_CACHE",
                other => unreachable!(
                    concat!("invalid value for", stringify!(ValidationFeatureDisableEXT), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for ValidationFeatureDisableEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::ALL => &"ALL",
            Self::SHADERS => &"SHADERS",
            Self::THREAD_SAFETY => &"THREAD_SAFETY",
            Self::API_PARAMETERS => &"API_PARAMETERS",
            Self::OBJECT_LIFETIMES => &"OBJECT_LIFETIMES",
            Self::CORE_CHECKS => &"CORE_CHECKS",
            Self::UNIQUE_HANDLES => &"UNIQUE_HANDLES",
            Self::SHADER_VALIDATION_CACHE => &"SHADER_VALIDATION_CACHE",
            other => unreachable!(
                concat!("invalid value for", stringify!(ValidationFeatureDisableEXT), ": {:?}"),
                other
            ),
        })
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
/// - [`enabled_validation_features`] is a pointer to an array of [`ValidationFeatureEnableEXT`]
///   values specifying the validation features to be enabled.
/// - [`disabled_validation_feature_count`] is the number of features to disable.
/// - [`disabled_validation_features`] is a pointer to an array of [`ValidationFeatureDisableEXT`]
///   values specifying the validation features to be disabled.
///# Description
///## Valid Usage
/// - If the [`enabled_validation_features`] array contains
///   `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT`, then it  **must**  also
///   contain `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT`
/// - If the [`enabled_validation_features`] array contains
///   `VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT`, then it  **must**  not contain
///   `VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT`
/// - If [`enabled_validation_feature_count`] is not `0`, [`enabled_validation_features`] **must**
///   be a valid pointer to an array of [`enabled_validation_feature_count`] valid
///   [`ValidationFeatureEnableEXT`] values
/// - If [`disabled_validation_feature_count`] is not `0`, [`disabled_validation_features`] **must**
///   be a valid pointer to an array of [`disabled_validation_feature_count`] valid
///   [`ValidationFeatureDisableEXT`] values
///# Related
/// - [`ext_validation_features`]
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
#[doc(alias = "VkValidationFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ValidationFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`enabled_validation_feature_count`] is the number of features to enable.
    pub enabled_validation_feature_count: u32,
    ///[`enabled_validation_features`] is a pointer to an array of
    ///[`ValidationFeatureEnableEXT`] values specifying the validation
    ///features to be enabled.
    pub enabled_validation_features: *const ValidationFeatureEnableEXT,
    ///[`disabled_validation_feature_count`] is the number of features to
    ///disable.
    pub disabled_validation_feature_count: u32,
    ///[`disabled_validation_features`] is a pointer to an array of
    ///[`ValidationFeatureDisableEXT`] values specifying the validation
    ///features to be disabled.
    pub disabled_validation_features: *const ValidationFeatureDisableEXT,
}
impl<'lt> Default for ValidationFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VALIDATION_FEATURES_EXT,
            p_next: std::ptr::null(),
            enabled_validation_feature_count: 0,
            enabled_validation_features: std::ptr::null(),
            disabled_validation_feature_count: 0,
            disabled_validation_features: std::ptr::null(),
        }
    }
}
impl<'lt> ValidationFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::enabled_validation_features`]
    pub fn enabled_validation_features_raw(&self) -> *const ValidationFeatureEnableEXT {
        self.enabled_validation_features
    }
    ///Gets the raw value of [`Self::disabled_validation_features`]
    pub fn disabled_validation_features_raw(&self) -> *const ValidationFeatureDisableEXT {
        self.disabled_validation_features
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::enabled_validation_features`]
    pub fn set_enabled_validation_features_raw(mut self, value: *const ValidationFeatureEnableEXT) -> Self {
        self.enabled_validation_features = value;
        self
    }
    ///Sets the raw value of [`Self::disabled_validation_features`]
    pub fn set_disabled_validation_features_raw(mut self, value: *const ValidationFeatureDisableEXT) -> Self {
        self.disabled_validation_features = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::enabled_validation_feature_count`]
    pub fn enabled_validation_feature_count(&self) -> u32 {
        self.enabled_validation_feature_count
    }
    ///Gets the value of [`Self::enabled_validation_features`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn enabled_validation_features(&self) -> &[ValidationFeatureEnableEXT] {
        std::slice::from_raw_parts(
            self.enabled_validation_features,
            self.enabled_validation_feature_count as usize,
        )
    }
    ///Gets the value of [`Self::disabled_validation_feature_count`]
    pub fn disabled_validation_feature_count(&self) -> u32 {
        self.disabled_validation_feature_count
    }
    ///Gets the value of [`Self::disabled_validation_features`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn disabled_validation_features(&self) -> &[ValidationFeatureDisableEXT] {
        std::slice::from_raw_parts(
            self.disabled_validation_features,
            self.disabled_validation_feature_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::enabled_validation_feature_count`]
    pub fn enabled_validation_feature_count_mut(&mut self) -> &mut u32 {
        &mut self.enabled_validation_feature_count
    }
    ///Gets a mutable reference to the value of [`Self::disabled_validation_feature_count`]
    pub fn disabled_validation_feature_count_mut(&mut self) -> &mut u32 {
        &mut self.disabled_validation_feature_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::enabled_validation_feature_count`]
    pub fn set_enabled_validation_feature_count(mut self, value: u32) -> Self {
        self.enabled_validation_feature_count = value;
        self
    }
    ///Sets the value of [`Self::enabled_validation_features`]
    pub fn set_enabled_validation_features(
        mut self,
        value: &'lt [crate::extensions::ext_validation_features::ValidationFeatureEnableEXT],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.enabled_validation_features = value.as_ptr();
        self.enabled_validation_feature_count = len_;
        self
    }
    ///Sets the value of [`Self::disabled_validation_feature_count`]
    pub fn set_disabled_validation_feature_count(mut self, value: u32) -> Self {
        self.disabled_validation_feature_count = value;
        self
    }
    ///Sets the value of [`Self::disabled_validation_features`]
    pub fn set_disabled_validation_features(
        mut self,
        value: &'lt [crate::extensions::ext_validation_features::ValidationFeatureDisableEXT],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.disabled_validation_features = value.as_ptr();
        self.disabled_validation_feature_count = len_;
        self
    }
}
