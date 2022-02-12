//![VK_EXT_validation_flags](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_flags.html) - instance extension
//!# Description
//!This extension provides the [`ValidationFlagsEXT`] struct that can be
//!included in the `pNext` chain of the [`InstanceCreateInfo`]
//!structure passed as the `pCreateInfo` parameter of
//![`CreateInstance`].
//!The structure contains an array of [`ValidationCheckEXT`] values that
//!will be disabled by the validation layers.
//!# Revision
//!2
//!# Dependencies
//! - *Deprecated* by
//!`[`VK_EXT_validation_features`]`
//!extension
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Tobin Ehlis [tobine](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_flags]
//!   @tobine%0A<<Here describe the issue or question you have about the VK_EXT_validation_flags
//!   extension>>)
//!# New structures
//! - Extending [`InstanceCreateInfo`]:
//! - [`ValidationFlagsEXT`]
//!# New enums
//! - [`ValidationCheckEXT`]
//!# New constants
//! - [`EXT_VALIDATION_FLAGS_EXTENSION_NAME`]
//! - [`EXT_VALIDATION_FLAGS_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT`
//!# Version History
//! - Revision 2, 2019-08-19 (Mark Lobodzinski)
//! - Marked as deprecated
//! - Revision 1, 2016-08-26 (Courtney Goeltzenleuchter)
//! - Initial draft
//!# Other info
//! * 2019-08-19
//! * No known IP claims.
//!*
//! - Tobin Ehlis, Google
//! - Courtney Goeltzenleuchter, Google
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
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
/// - [`VALIDATION_CHECK_ALL`] specifies that all validation checks
///are disabled.
/// - [`VALIDATION_CHECK_SHADERS`] specifies that shader validation
///is disabled.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ValidationCheckEXT(i32);
impl const Default for ValidationCheckEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ValidationCheckEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ValidationCheckEXT")
            .field(match *self {
                Self::VALIDATION_CHECK_ALL => &"VALIDATION_CHECK_ALL",
                Self::VALIDATION_CHECK_SHADERS => &"VALIDATION_CHECK_SHADERS",
                other => unreachable!("invalid value for `ValidationCheckEXT`: {:?}", other),
            })
            .finish()
    }
}
impl ValidationCheckEXT {
    ///[`VALIDATION_CHECK_ALL`] specifies that all validation checks
    ///are disabled.
    pub const VALIDATION_CHECK_ALL: Self = Self(0);
    ///[`VALIDATION_CHECK_SHADERS`] specifies that shader validation
    ///is disabled.
    pub const VALIDATION_CHECK_SHADERS: Self = Self(1);
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
