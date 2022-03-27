use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_SPEC_VERSION")]
pub const EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME")]
pub const EXT_VALIDATION_FLAGS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_validation_flags");
///[VkValidationCheckEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCheckEXT.html) - Specify validation checks to disable
///# C Specifications
///Possible values of elements of the
///[`ValidationFlagsEXT::p_disabled_validation_checks`] array,
///specifying validation checks to be disabled, are:
///```c
///// Provided by VK_EXT_validation_flags
///typedef enum VkValidationCheckEXT {
///    VK_VALIDATION_CHECK_ALL_EXT = 0,
///    VK_VALIDATION_CHECK_SHADERS_EXT = 1,
///} VkValidationCheckEXT;
///```
///# Description
/// - [`ValidationCheckAllExt`] specifies that all validation checks are disabled.
/// - [`ValidationCheckShadersExt`] specifies that shader validation is disabled.
///# Related
/// - [`VK_EXT_validation_flags`]
/// - [`ValidationFlagsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationCheckEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ValidationCheckEXT {
    ///[`ValidationCheckAllExt`] specifies that all validation checks
    ///are disabled.
    ValidationCheckAllExt = 0,
    ///[`ValidationCheckShadersExt`] specifies that shader validation
    ///is disabled.
    ValidationCheckShadersExt = 1,
}
impl const Default for ValidationCheckEXT {
    fn default() -> Self {
        ValidationCheckAllExt
    }
}
impl ValidationCheckEXT {
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
///[VkValidationFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationFlagsEXT.html) - Specify validation checks to disable for a Vulkan instance
///# C Specifications
///When creating a Vulkan instance for which you wish to disable validation
///checks, add a [`ValidationFlagsEXT`] structure to the [`p_next`] chain
///of the [`InstanceCreateInfo`] structure, specifying the checks to be
///disabled.
///```c
///// Provided by VK_EXT_validation_flags
///typedef struct VkValidationFlagsEXT {
///    VkStructureType                sType;
///    const void*                    pNext;
///    uint32_t                       disabledValidationCheckCount;
///    const VkValidationCheckEXT*    pDisabledValidationChecks;
///} VkValidationFlagsEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`disabled_validation_check_count`] is the number of checks to disable.
/// - [`p_disabled_validation_checks`] is a pointer to an array of [`ValidationCheckEXT`] values
///   specifying the validation checks to be disabled.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`
/// - [`p_disabled_validation_checks`]**must** be a valid pointer to an array of
///   [`disabled_validation_check_count`] valid [`ValidationCheckEXT`] values
/// - [`disabled_validation_check_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_validation_flags`]
/// - [`StructureType`]
/// - [`ValidationCheckEXT`]
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
pub struct ValidationFlagsEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`disabled_validation_check_count`] is the number of checks to disable.
    disabled_validation_check_count: u32,
    ///[`p_disabled_validation_checks`] is a pointer to an array of
    ///[`ValidationCheckEXT`] values specifying the validation checks to be
    ///disabled.
    p_disabled_validation_checks: *mut ValidationCheckEXT,
}
