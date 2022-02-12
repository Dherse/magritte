//![VK_EXT_validation_cache](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_validation_cache.html) - device extension
//!# Description
//!This extension provides a mechanism for caching the results of potentially
//!expensive internal validation operations across multiple runs of a Vulkan
//!application.
//!At the core is the [`ValidationCacheEXT`] object type, which is managed
//!similarly to the existing [`PipelineCache`].The new struct
//! [`ShaderModuleValidationCacheCreateInfoEXT`] can be
//!included in the `pNext` chain at [`CreateShaderModule`] time.
//!It contains a [`ValidationCacheEXT`] to use when validating the
//![`ShaderModule`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Cort Stratton [cdwfs](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_validation_cache]
//!   @cdwfs%0A<<Here describe the issue or question you have about the VK_EXT_validation_cache
//!   extension>>)
//!# New handles
//! - [`ValidationCacheEXT`]
//!# New functions & commands
//! - [`CreateValidationCacheEXT`]
//! - [`DestroyValidationCacheEXT`]
//! - [`GetValidationCacheDataEXT`]
//! - [`MergeValidationCachesEXT`]
//!# New structures
//! - [`ValidationCacheCreateInfoEXT`]
//! - Extending [`ShaderModuleCreateInfo`]:
//! - [`ShaderModuleValidationCacheCreateInfoEXT`]
//!# New enums
//! - [`ValidationCacheHeaderVersionEXT`]
//!# New bitmasks
//! - [`ValidationCacheCreateFlagsEXT`]
//!# New constants
//! - [`EXT_VALIDATION_CACHE_EXTENSION_NAME`]
//! - [`EXT_VALIDATION_CACHE_SPEC_VERSION`]
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_VALIDATION_CACHE_EXT`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2017-08-29 (Cort Stratton)
//! - Initial draft
//!# Other info
//! * 2017-08-29
//! * No known IP claims.
//!*
//! - Cort Stratton, Google
//! - Chris Forbes, Google
//!# Related
//! - [`ShaderModuleValidationCacheCreateInfoEXT`]
//! - [`ValidationCacheCreateFlagsEXT`]
//! - [`ValidationCacheCreateInfoEXT`]
//! - [`ValidationCacheEXT`]
//! - [`ValidationCacheHeaderVersionEXT`]
//! - [`CreateValidationCacheEXT`]
//! - [`DestroyValidationCacheEXT`]
//! - [`GetValidationCacheDataEXT`]
//! - [`MergeValidationCachesEXT`]
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
#[doc(alias = "VK_EXT_VALIDATION_CACHE_SPEC_VERSION")]
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VALIDATION_CACHE_EXTENSION_NAME")]
pub const EXT_VALIDATION_CACHE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_validation_cache");
///[VkValidationCacheHeaderVersionEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html) - Encode validation cache version
///# C Specifications
///Possible values of the second group of four bytes in the header returned by
///[`GetValidationCacheDataEXT`], encoding the validation cache version,
///are:
///```c
///// Provided by VK_EXT_validation_cache
///typedef enum VkValidationCacheHeaderVersionEXT {
///    VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT = 1,
///} VkValidationCacheHeaderVersionEXT;
///```
///# Description
/// - [`VALIDATION_CACHE_HEADER_VERSION_ONE`] specifies version one
///of the validation cache.
///# Related
/// - [`VK_EXT_validation_cache`]
/// - [`CreateValidationCacheEXT`]
/// - [`GetValidationCacheDataEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkValidationCacheHeaderVersionEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ValidationCacheHeaderVersionEXT(i32);
impl const Default for ValidationCacheHeaderVersionEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ValidationCacheHeaderVersionEXT")
            .field(match *self {
                Self::VALIDATION_CACHE_HEADER_VERSION_ONE => &"VALIDATION_CACHE_HEADER_VERSION_ONE",
                other => unreachable!("invalid value for `ValidationCacheHeaderVersionEXT`: {:?}", other),
            })
            .finish()
    }
}
impl ValidationCacheHeaderVersionEXT {
    ///[`VALIDATION_CACHE_HEADER_VERSION_ONE`] specifies version one
    ///of the validation cache.
    pub const VALIDATION_CACHE_HEADER_VERSION_ONE: Self = Self(1);
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
