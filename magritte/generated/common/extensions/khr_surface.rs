#[cfg(feature = "VK_KHR_display")]
use crate::common::extensions::khr_display::SurfaceTransformFlagsKHR;
use crate::{
    common::vulkan1_0::{Extent2D, Format, ImageUsageFlags},
    cstr,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkSurfaceCapabilitiesKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    #[doc(alias = "minImageCount")]
    pub min_image_count: u32,
    #[doc(alias = "maxImageCount")]
    pub max_image_count: u32,
    #[doc(alias = "currentExtent")]
    pub current_extent: Extent2D,
    #[doc(alias = "minImageExtent")]
    pub min_image_extent: Extent2D,
    #[doc(alias = "maxImageExtent")]
    pub max_image_extent: Extent2D,
    #[doc(alias = "maxImageArrayLayers")]
    pub max_image_array_layers: u32,
    #[doc(alias = "supportedTransforms")]
    pub supported_transforms: SurfaceTransformFlagsKHR,
    #[doc(alias = "currentTransform")]
    pub current_transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "supportedCompositeAlpha")]
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    #[doc(alias = "supportedUsageFlags")]
    pub supported_usage_flags: ImageUsageFlags,
}
#[doc(alias = "VkSurfaceFormatKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    #[doc(alias = "colorSpace")]
    pub color_space: ColorSpaceKHR,
}
#[doc(alias = "VkCompositeAlphaFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompositeAlphaFlagsKHR(u32);
impl CompositeAlphaFlagsKHR {
    #[doc(alias = "VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR")]
    pub const PRE_MULTIPLIED: Self = Self(2);
    #[doc(alias = "VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR")]
    pub const POST_MULTIPLIED: Self = Self(4);
    #[doc(alias = "VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR")]
    pub const INHERIT: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::OPAQUE;
        }
        {
            all |= Self::PRE_MULTIPLIED;
        }
        {
            all |= Self::POST_MULTIPLIED;
        }
        {
            all |= Self::INHERIT;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl std::ops::BitOr for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<CompositeAlphaFlagsKHR> for CompositeAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = CompositeAlphaFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<CompositeAlphaFlagsKHR> for CompositeAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = CompositeAlphaFlagsKHR>>(iterator: T) -> CompositeAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<CompositeAlphaFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for CompositeAlphaFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn from(bit: CompositeAlphaFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = CompositeAlphaFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = CompositeAlphaFlagBitsKHR>>(iterator: T) -> CompositeAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<CompositeAlphaFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for CompositeAlphaFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(CompositeAlphaFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == CompositeAlphaFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(CompositeAlphaFlagsKHR::OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE))?;
                    }
                    if self.0.contains(CompositeAlphaFlagsKHR::PRE_MULTIPLIED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PRE_MULTIPLIED))?;
                    }
                    if self.0.contains(CompositeAlphaFlagsKHR::POST_MULTIPLIED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(POST_MULTIPLIED))?;
                    }
                    if self.0.contains(CompositeAlphaFlagsKHR::INHERIT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INHERIT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(CompositeAlphaFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CompositeAlphaFlagsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CompositeAlphaFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_KHR_SURFACE_SPEC_VERSION")]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
#[doc(alias = "VK_KHR_SURFACE_EXTENSION_NAME")]
pub const KHR_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_surface");
#[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CompositeAlphaFlagBitsKHR(u32);
impl Default for CompositeAlphaFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl CompositeAlphaFlagBitsKHR {
    #[doc(alias = "VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR")]
    pub const PRE_MULTIPLIED: Self = Self(2);
    #[doc(alias = "VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR")]
    pub const POST_MULTIPLIED: Self = Self(4);
    #[doc(alias = "VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR")]
    pub const INHERIT: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::OPAQUE.bits() => Some(Self(x)),
            x if x == Self::PRE_MULTIPLIED.bits() => Some(Self(x)),
            x if x == Self::POST_MULTIPLIED.bits() => Some(Self(x)),
            x if x == Self::INHERIT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CompositeAlphaFlagBitsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CompositeAlphaFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SurfaceTransformFlagBitsKHR(u32);
impl Default for SurfaceTransformFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SurfaceTransformFlagBitsKHR {
    #[doc(alias = "VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR")]
    pub const IDENTITY: Self = Self(1);
    #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR")]
    pub const ROTATE90: Self = Self(2);
    #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR")]
    pub const ROTATE180: Self = Self(4);
    #[doc(alias = "VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR")]
    pub const ROTATE270: Self = Self(8);
    #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR")]
    pub const HORIZONTAL_MIRROR: Self = Self(16);
    #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR")]
    pub const HORIZONTAL_MIRROR_ROTATE90: Self = Self(32);
    #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR")]
    pub const HORIZONTAL_MIRROR_ROTATE180: Self = Self(64);
    #[doc(alias = "VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR")]
    pub const HORIZONTAL_MIRROR_ROTATE270: Self = Self(128);
    #[doc(alias = "VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR")]
    pub const INHERIT: Self = Self(256);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::IDENTITY.bits() => Some(Self(x)),
            x if x == Self::ROTATE90.bits() => Some(Self(x)),
            x if x == Self::ROTATE180.bits() => Some(Self(x)),
            x if x == Self::ROTATE270.bits() => Some(Self(x)),
            x if x == Self::HORIZONTAL_MIRROR.bits() => Some(Self(x)),
            x if x == Self::HORIZONTAL_MIRROR_ROTATE90.bits() => Some(Self(x)),
            x if x == Self::HORIZONTAL_MIRROR_ROTATE180.bits() => Some(Self(x)),
            x if x == Self::HORIZONTAL_MIRROR_ROTATE270.bits() => Some(Self(x)),
            x if x == Self::INHERIT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceTransformFlagBitsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceTransformFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPresentModeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum PresentModeKHR {
    #[doc(alias = "VK_PRESENT_MODE_IMMEDIATE_KHR")]
    Immediate = 0,
    #[doc(alias = "VK_PRESENT_MODE_MAILBOX_KHR")]
    Mailbox = 1,
    #[doc(alias = "VK_PRESENT_MODE_FIFO_KHR")]
    Fifo = 2,
    #[doc(alias = "VK_PRESENT_MODE_FIFO_RELAXED_KHR")]
    FifoRelaxed = 3,
    #[doc(alias = "VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR")]
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    SharedDemandRefresh = 1000111000,
    #[doc(alias = "VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR")]
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    SharedContinuousRefresh = 1000111001,
}
impl Default for PresentModeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PresentModeKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Immediate.bits() => Some(Self::Immediate),
            x if x == Self::Mailbox.bits() => Some(Self::Mailbox),
            x if x == Self::Fifo.bits() => Some(Self::Fifo),
            x if x == Self::FifoRelaxed.bits() => Some(Self::FifoRelaxed),
            #[cfg(feature = "VK_KHR_shared_presentable_image")]
            x if x == Self::SharedDemandRefresh.bits() => Some(Self::SharedDemandRefresh),
            #[cfg(feature = "VK_KHR_shared_presentable_image")]
            x if x == Self::SharedContinuousRefresh.bits() => Some(Self::SharedContinuousRefresh),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentModeKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentModeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkColorSpaceKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ColorSpaceKHR {
    #[doc(alias = "VK_COLOR_SPACE_SRGB_NONLINEAR_KHR")]
    #[doc(alias = "VK_COLORSPACE_SRGB_NONLINEAR_KHR")]
    SrgbNonlinear = 0,
    #[doc(alias = "VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DisplayP3NonlinearExt = 1000104001,
    #[doc(alias = "VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ExtendedSrgbLinearExt = 1000104002,
    #[doc(alias = "VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT")]
    #[doc(alias = "VK_COLOR_SPACE_DCI_P3_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DisplayP3LinearExt = 1000104003,
    #[doc(alias = "VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DciP3NonlinearExt = 1000104004,
    #[doc(alias = "VK_COLOR_SPACE_BT709_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    Bt709LinearExt = 1000104005,
    #[doc(alias = "VK_COLOR_SPACE_BT709_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    Bt709NonlinearExt = 1000104006,
    #[doc(alias = "VK_COLOR_SPACE_BT2020_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    Bt2020LinearExt = 1000104007,
    #[doc(alias = "VK_COLOR_SPACE_HDR10_ST2084_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    Hdr10St2084Ext = 1000104008,
    #[doc(alias = "VK_COLOR_SPACE_DOLBYVISION_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    DolbyvisionExt = 1000104009,
    #[doc(alias = "VK_COLOR_SPACE_HDR10_HLG_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    Hdr10HlgExt = 1000104010,
    #[doc(alias = "VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    AdobergbLinearExt = 1000104011,
    #[doc(alias = "VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    AdobergbNonlinearExt = 1000104012,
    #[doc(alias = "VK_COLOR_SPACE_PASS_THROUGH_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    PassThroughExt = 1000104013,
    #[doc(alias = "VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    ExtendedSrgbNonlinearExt = 1000104014,
    #[doc(alias = "VK_COLOR_SPACE_DISPLAY_NATIVE_AMD")]
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    DisplayNativeAmd = 1000213000,
}
impl Default for ColorSpaceKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ColorSpaceKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::SrgbNonlinear.bits() => Some(Self::SrgbNonlinear),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DisplayP3NonlinearExt.bits() => Some(Self::DisplayP3NonlinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::ExtendedSrgbLinearExt.bits() => Some(Self::ExtendedSrgbLinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DisplayP3LinearExt.bits() => Some(Self::DisplayP3LinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DciP3NonlinearExt.bits() => Some(Self::DciP3NonlinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::Bt709LinearExt.bits() => Some(Self::Bt709LinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::Bt709NonlinearExt.bits() => Some(Self::Bt709NonlinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::Bt2020LinearExt.bits() => Some(Self::Bt2020LinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::Hdr10St2084Ext.bits() => Some(Self::Hdr10St2084Ext),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DolbyvisionExt.bits() => Some(Self::DolbyvisionExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::Hdr10HlgExt.bits() => Some(Self::Hdr10HlgExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::AdobergbLinearExt.bits() => Some(Self::AdobergbLinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::AdobergbNonlinearExt.bits() => Some(Self::AdobergbNonlinearExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::PassThroughExt.bits() => Some(Self::PassThroughExt),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::ExtendedSrgbNonlinearExt.bits() => Some(Self::ExtendedSrgbNonlinearExt),
            #[cfg(feature = "VK_AMD_display_native_hdr")]
            x if x == Self::DisplayNativeAmd.bits() => Some(Self::DisplayNativeAmd),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ColorSpaceKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ColorSpaceKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
