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
