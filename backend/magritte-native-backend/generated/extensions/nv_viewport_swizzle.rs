use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkViewportSwizzleNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ViewportSwizzleNV {
    x: ViewportCoordinateSwizzleNV,
    y: ViewportCoordinateSwizzleNV,
    z: ViewportCoordinateSwizzleNV,
    w: ViewportCoordinateSwizzleNV,
}
#[doc(alias = "VkPipelineViewportSwizzleStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineViewportSwizzleStateCreateFlagsNV,
    #[doc(alias = "viewportCount")]
    viewport_count: u32,
    #[doc(alias = "pViewportSwizzles")]
    viewport_swizzles: *const ViewportSwizzleNV,
}
#[doc(alias = "VkPipelineViewportSwizzleStateCreateFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportSwizzleStateCreateFlagsNV(u32);
impl PipelineViewportSwizzleStateCreateFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION")]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME")]
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_viewport_swizzle");
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ViewportCoordinateSwizzleNV(i32);
impl ViewportCoordinateSwizzleNV {
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV")]
    pub const POSITIVE_X: Self = Self(0);
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV")]
    pub const NEGATIVE_X: Self = Self(1);
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV")]
    pub const POSITIVE_Y: Self = Self(2);
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV")]
    pub const NEGATIVE_Y: Self = Self(3);
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV")]
    pub const POSITIVE_Z: Self = Self(4);
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV")]
    pub const NEGATIVE_Z: Self = Self(5);
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV")]
    pub const POSITIVE_W: Self = Self(6);
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV")]
    pub const NEGATIVE_W: Self = Self(7);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::POSITIVE_X.bits() => Some(Self(x)),
            x if x == Self::NEGATIVE_X.bits() => Some(Self(x)),
            x if x == Self::POSITIVE_Y.bits() => Some(Self(x)),
            x if x == Self::NEGATIVE_Y.bits() => Some(Self(x)),
            x if x == Self::POSITIVE_Z.bits() => Some(Self(x)),
            x if x == Self::NEGATIVE_Z.bits() => Some(Self(x)),
            x if x == Self::POSITIVE_W.bits() => Some(Self(x)),
            x if x == Self::NEGATIVE_W.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
