#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION")]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME")]
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_viewport_swizzle");
///[VkViewportCoordinateSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportCoordinateSwizzleNV.html) - Specify how a viewport coordinate is swizzled
///# C Specifications
///Possible values of the [`ViewportSwizzleNV::x`], `y`, `z`,
///and `w` members, specifying swizzling of the corresponding components of
///primitives, are:
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef enum VkViewportCoordinateSwizzleNV {
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV = 0,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV = 1,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV = 2,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV = 3,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV = 4,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV = 5,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV = 6,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV = 7,
///} VkViewportCoordinateSwizzleNV;
///```
///# Description
///These values are described in detail in [Viewport Swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-viewport-swizzle).
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`ViewportSwizzleNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ViewportCoordinateSwizzleNV(i32);
impl const Default for ViewportCoordinateSwizzleNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ViewportCoordinateSwizzleNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ViewportCoordinateSwizzleNV")
            .field(match *self {
                Self::VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X => &"VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X",
                Self::VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X => &"VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X",
                Self::VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y => &"VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y",
                Self::VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y => &"VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y",
                Self::VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z => &"VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z",
                Self::VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z => &"VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z",
                Self::VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W => &"VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W",
                Self::VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W => &"VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W",
                other => unreachable!("invalid value for `ViewportCoordinateSwizzleNV`: {:?}", other),
            })
            .finish()
    }
}
impl ViewportCoordinateSwizzleNV {
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X: Self = Self(0);
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X: Self = Self(1);
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y: Self = Self(2);
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y: Self = Self(3);
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z: Self = Self(4);
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z: Self = Self(5);
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W: Self = Self(6);
    ///No documentation found
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W: Self = Self(7);
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
