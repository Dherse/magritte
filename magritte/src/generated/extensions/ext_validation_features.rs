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
#[non_exhaustive]
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
        Self::ValidationFeatureEnableGpuAssistedExt
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
#[non_exhaustive]
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
        Self::ValidationFeatureDisableAllExt
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ValidationFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`enabled_validation_feature_count`] is the number of features to enable.
    enabled_validation_feature_count: u32,
    ///[`enabled_validation_features`] is a pointer to an array of
    ///[`ValidationFeatureEnableEXT`] values specifying the validation
    ///features to be enabled.
    enabled_validation_features: *const ValidationFeatureEnableEXT,
    ///[`disabled_validation_feature_count`] is the number of features to
    ///disable.
    disabled_validation_feature_count: u32,
    ///[`disabled_validation_features`] is a pointer to an array of
    ///[`ValidationFeatureDisableEXT`] values specifying the validation
    ///features to be disabled.
    disabled_validation_features: *const ValidationFeatureDisableEXT,
}
impl<'lt> Default for ValidationFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::enabled_validation_features`]
    pub fn set_enabled_validation_features_raw(&mut self, value: *const ValidationFeatureEnableEXT) -> &mut Self {
        self.enabled_validation_features = value;
        self
    }
    ///Sets the raw value of [`Self::disabled_validation_features`]
    pub fn set_disabled_validation_features_raw(&mut self, value: *const ValidationFeatureDisableEXT) -> &mut Self {
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
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::disabled_validation_feature_count`]
    pub fn disabled_validation_feature_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::enabled_validation_feature_count`]
    pub fn set_enabled_validation_feature_count(&mut self, value: u32) -> &mut Self {
        self.enabled_validation_feature_count = value;
        self
    }
    ///Sets the raw value of [`Self::enabled_validation_features`]
    pub fn set_enabled_validation_features(
        &mut self,
        value: &'lt [crate::extensions::ext_validation_features::ValidationFeatureEnableEXT],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.enabled_validation_features = value.as_ptr();
        self.enabled_validation_feature_count = len_;
        self
    }
    ///Sets the raw value of [`Self::disabled_validation_feature_count`]
    pub fn set_disabled_validation_feature_count(&mut self, value: u32) -> &mut Self {
        self.disabled_validation_feature_count = value;
        self
    }
    ///Sets the raw value of [`Self::disabled_validation_features`]
    pub fn set_disabled_validation_features(
        &mut self,
        value: &'lt [crate::extensions::ext_validation_features::ValidationFeatureDisableEXT],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.disabled_validation_features = value.as_ptr();
        self.disabled_validation_feature_count = len_;
        self
    }
}
