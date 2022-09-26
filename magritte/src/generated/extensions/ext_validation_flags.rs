//![VK_EXT_validation_flags](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_flags.html) - instance extension
//!# Description
//!This extension provides the [`ValidationFlagsEXT`] struct that can be
//!included in the `pNext` chain of the [`InstanceCreateInfo`]
//!structure passed as the `pCreateInfo` parameter of
//![`create_instance`].
//!The structure contains an array of [`ValidationCheckEXT`] values that
//!will be disabled by the validation layers.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Deprecation State
//! - *Deprecated* by `[`ext_validation_features`]` extension
//!# Contacts
//! - Tobin Ehlis [tobine](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_flags]
//!   @tobine%0A<<Here describe the issue or question you have about the VK_EXT_validation_flags
//!   extension>>)
//!# New structures
//! - Extending [`InstanceCreateInfo`]:  - [`ValidationFlagsEXT`]
//!# New enums
//! - [`ValidationCheckEXT`]
//!# New constants
//! - [`EXT_VALIDATION_FLAGS_EXTENSION_NAME`]
//! - [`EXT_VALIDATION_FLAGS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`
//!# Version history
//! - Revision 2, 2019-08-19 (Mark Lobodzinski)  - Marked as deprecated
//! - Revision 1, 2016-08-26 (Courtney Goeltzenleuchter)  - Initial draft
//!# Other information
//! * 2019-08-19
//! * No known IP claims.
//! * - Tobin Ehlis, Google  - Courtney Goeltzenleuchter, Google
//!# Related
//! - [`ValidationCheckEXT`]
//! - [`ValidationFlagsEXT`]
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
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_SPEC_VERSION")]
pub const EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME")]
pub const EXT_VALIDATION_FLAGS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_validation_flags");
///[VkValidationCheckEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCheckEXT.html) - Specify validation checks to disable
///# C Specifications
///Possible values of elements of the
///[`ValidationFlagsEXT::disabled_validation_checks`] array,
///specifying validation checks to be disabled, are:
///```c
///// Provided by VK_EXT_validation_flags
///typedef enum VkValidationCheckEXT {
///    VK_VALIDATION_CHECK_ALL_EXT = 0,
///    VK_VALIDATION_CHECK_SHADERS_EXT = 1,
///} VkValidationCheckEXT;
///```
///# Description
/// - [`ALL`] specifies that all validation checks are disabled.
/// - [`SHADERS`] specifies that shader validation is disabled.
///# Related
/// - [`ext_validation_flags`]
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ValidationCheckEXT(i32);
impl const Default for ValidationCheckEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ValidationCheckEXT {
    ///[`ALL`] specifies that all validation checks
    ///are disabled.
    pub const ALL: Self = Self(0);
    ///[`SHADERS`] specifies that shader validation
    ///is disabled.
    pub const SHADERS: Self = Self(1);
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
impl std::fmt::Debug for ValidationCheckEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ValidationCheckEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ValidationCheckEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        ValidationCheckEXT::ALL => f.write_str("ALL")?,
                        ValidationCheckEXT::SHADERS => f.write_str("SHADERS")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ValidationCheckEXT))
            .field(&Flags(*self))
            .finish()
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
/// - [`disabled_validation_checks`] is a pointer to an array of [`ValidationCheckEXT`] values
///   specifying the validation checks to be disabled.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`
/// - [`disabled_validation_checks`] **must**  be a valid pointer to an array of
///   [`disabled_validation_check_count`] valid [`ValidationCheckEXT`] values
/// - [`disabled_validation_check_count`] **must**  be greater than `0`
///# Related
/// - [`ext_validation_flags`]
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
#[doc(alias = "VkValidationFlagsEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ValidationFlagsEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`disabled_validation_check_count`] is the number of checks to disable.
    pub disabled_validation_check_count: u32,
    ///[`disabled_validation_checks`] is a pointer to an array of
    ///[`ValidationCheckEXT`] values specifying the validation checks to be
    ///disabled.
    pub disabled_validation_checks: *const ValidationCheckEXT,
}
impl<'lt> Default for ValidationFlagsEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VALIDATION_FLAGS_EXT,
            p_next: std::ptr::null(),
            disabled_validation_check_count: 0,
            disabled_validation_checks: std::ptr::null(),
        }
    }
}
impl<'lt> ValidationFlagsEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::disabled_validation_checks`]
    pub fn disabled_validation_checks_raw(&self) -> *const ValidationCheckEXT {
        self.disabled_validation_checks
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::disabled_validation_checks`]
    pub fn set_disabled_validation_checks_raw(&mut self, value: *const ValidationCheckEXT) -> &mut Self {
        self.disabled_validation_checks = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::disabled_validation_checks`]
    pub fn with_disabled_validation_checks_raw(mut self, value: *const ValidationCheckEXT) -> Self {
        self.disabled_validation_checks = value;
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
    ///Gets the value of [`Self::disabled_validation_check_count`]
    pub fn disabled_validation_check_count(&self) -> u32 {
        self.disabled_validation_check_count
    }
    ///Gets the value of [`Self::disabled_validation_checks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn disabled_validation_checks(&self) -> &[ValidationCheckEXT] {
        std::slice::from_raw_parts(
            self.disabled_validation_checks,
            self.disabled_validation_check_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::disabled_validation_check_count`]
    pub fn disabled_validation_check_count_mut(&mut self) -> &mut u32 {
        &mut self.disabled_validation_check_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::disabled_validation_check_count`]
    pub fn set_disabled_validation_check_count(&mut self, value: u32) -> &mut Self {
        self.disabled_validation_check_count = value;
        self
    }
    ///Sets the value of [`Self::disabled_validation_checks`]
    pub fn set_disabled_validation_checks(
        &mut self,
        value: &'lt [crate::extensions::ext_validation_flags::ValidationCheckEXT],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.disabled_validation_checks = value.as_ptr();
        self.disabled_validation_check_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::disabled_validation_check_count`]
    pub fn with_disabled_validation_check_count(mut self, value: u32) -> Self {
        self.disabled_validation_check_count = value;
        self
    }
    ///Sets the value of [`Self::disabled_validation_checks`]
    pub fn with_disabled_validation_checks(
        mut self,
        value: &'lt [crate::extensions::ext_validation_flags::ValidationCheckEXT],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.disabled_validation_checks = value.as_ptr();
        self.disabled_validation_check_count = len_;
        self
    }
}
