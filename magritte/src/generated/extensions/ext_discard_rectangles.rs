#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION")]
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME")]
pub const EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_discard_rectangles");
///[VkDiscardRectangleModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDiscardRectangleModeEXT.html) - Specify the discard rectangle mode
///# C Specifications
///[`DiscardRectangleModeEXT`] values are:
///```c
///// Provided by VK_EXT_discard_rectangles
///typedef enum VkDiscardRectangleModeEXT {
///    VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0,
///    VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1,
///} VkDiscardRectangleModeEXT;
///```
///# Description
/// - [`DISCARD_RECTANGLE_MODE_INCLUSIVE`] specifies that the discard
///rectangle test is inclusive.
/// - [`DISCARD_RECTANGLE_MODE_EXCLUSIVE`] specifies that the discard
///rectangle test is exclusive.
///# Related
/// - [`VK_EXT_discard_rectangles`]
/// - [`PipelineDiscardRectangleStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDiscardRectangleModeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DiscardRectangleModeEXT(i32);
impl const Default for DiscardRectangleModeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DiscardRectangleModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DiscardRectangleModeEXT")
            .field(match *self {
                Self::DISCARD_RECTANGLE_MODE_INCLUSIVE => &"DISCARD_RECTANGLE_MODE_INCLUSIVE",
                Self::DISCARD_RECTANGLE_MODE_EXCLUSIVE => &"DISCARD_RECTANGLE_MODE_EXCLUSIVE",
                other => unreachable!("invalid value for `DiscardRectangleModeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl DiscardRectangleModeEXT {
    ///[`DISCARD_RECTANGLE_MODE_INCLUSIVE`] specifies that the discard
    ///rectangle test is inclusive.
    pub const DISCARD_RECTANGLE_MODE_INCLUSIVE: Self = Self(0);
    ///[`DISCARD_RECTANGLE_MODE_EXCLUSIVE`] specifies that the discard
    ///rectangle test is exclusive.
    pub const DISCARD_RECTANGLE_MODE_EXCLUSIVE: Self = Self(1);
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
