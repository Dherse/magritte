//!# [VK_KHR_surface](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_surface/VK_KHR_surface.md")]
#[cfg(feature = "VK_KHR_display")]
use crate::extensions::khr_display::SurfaceTransformFlagsKHR;
use crate::{
    cstr,
    vulkan1_0::{
        AllocationCallbacks, Bool32, Extent2D, Format, ImageUsageFlags, Instance, PhysicalDevice, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkSurfaceCapabilitiesKHR.md")]
#[doc(alias = "VkSurfaceCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    #[doc(alias = "minImageCount")]
    min_image_count: u32,
    #[doc(alias = "maxImageCount")]
    max_image_count: u32,
    #[doc(alias = "currentExtent")]
    current_extent: Extent2D,
    #[doc(alias = "minImageExtent")]
    min_image_extent: Extent2D,
    #[doc(alias = "maxImageExtent")]
    max_image_extent: Extent2D,
    #[doc(alias = "maxImageArrayLayers")]
    max_image_array_layers: u32,
    #[doc(alias = "supportedTransforms")]
    supported_transforms: SurfaceTransformFlagsKHR,
    #[doc(alias = "currentTransform")]
    current_transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "supportedCompositeAlpha")]
    supported_composite_alpha: CompositeAlphaFlagsKHR,
    #[doc(alias = "supportedUsageFlags")]
    supported_usage_flags: ImageUsageFlags,
}
///# [VkSurfaceFormatKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkSurfaceFormatKHR.md")]
#[doc(alias = "VkSurfaceFormatKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceFormatKHR {
    format: Format,
    #[doc(alias = "colorSpace")]
    color_space: ColorSpaceKHR,
}
///# [VkSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkSurfaceKHR.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSurfaceKHR")]
#[repr(transparent)]
pub struct SurfaceKHR(u64);
impl SurfaceKHR {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for SurfaceKHR {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkCompositeAlphaFlagBitsKHR.md")]
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
impl const std::ops::BitOr for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for CompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for CompositeAlphaFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for CompositeAlphaFlagsKHR {
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
impl const Default for CompositeAlphaFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
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
#[doc(alias = "VK_KHR_SURFACE_SPEC_VERSION")]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
#[doc(alias = "VK_KHR_SURFACE_EXTENSION_NAME")]
pub const KHR_SURFACE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_surface");
///# [VkCompositeAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCompositeAlphaFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkCompositeAlphaFlagBitsKHR.md")]
#[doc(alias = "VkCompositeAlphaFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CompositeAlphaFlagBitsKHR(u32);
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
///# [VkSurfaceTransformFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceTransformFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkSurfaceTransformFlagBitsKHR.md")]
#[doc(alias = "VkSurfaceTransformFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SurfaceTransformFlagBitsKHR(u32);
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
///# [VkPresentModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkPresentModeKHR.md")]
#[doc(alias = "VkPresentModeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PresentModeKHR(i32);
impl PresentModeKHR {
    #[doc(alias = "VK_PRESENT_MODE_IMMEDIATE_KHR")]
    pub const IMMEDIATE: Self = Self(0);
    #[doc(alias = "VK_PRESENT_MODE_MAILBOX_KHR")]
    pub const MAILBOX: Self = Self(1);
    #[doc(alias = "VK_PRESENT_MODE_FIFO_KHR")]
    pub const FIFO: Self = Self(2);
    #[doc(alias = "VK_PRESENT_MODE_FIFO_RELAXED_KHR")]
    pub const FIFO_RELAXED: Self = Self(3);
    #[doc(alias = "VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR")]
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    pub const SHARED_DEMAND_REFRESH: Self = Self(1000111000);
    #[doc(alias = "VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR")]
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    pub const SHARED_CONTINUOUS_REFRESH: Self = Self(1000111001);
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
            x if x == Self::IMMEDIATE.bits() => Some(Self(x)),
            x if x == Self::MAILBOX.bits() => Some(Self(x)),
            x if x == Self::FIFO.bits() => Some(Self(x)),
            x if x == Self::FIFO_RELAXED.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_shared_presentable_image")]
            x if x == Self::SHARED_DEMAND_REFRESH.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_shared_presentable_image")]
            x if x == Self::SHARED_CONTINUOUS_REFRESH.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkColorSpaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/VkColorSpaceKHR.md")]
#[doc(alias = "VkColorSpaceKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ColorSpaceKHR(i32);
impl ColorSpaceKHR {
    #[doc(alias = "VK_COLOR_SPACE_SRGB_NONLINEAR_KHR")]
    pub const SRGB_NONLINEAR: Self = Self(0);
    #[doc(alias = "VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    #[doc(alias = "VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    #[doc(alias = "VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    #[doc(alias = "VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    #[doc(alias = "VK_COLOR_SPACE_BT709_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const BT709_LINEAR_EXT: Self = Self(1000104005);
    #[doc(alias = "VK_COLOR_SPACE_BT709_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
    #[doc(alias = "VK_COLOR_SPACE_BT2020_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
    #[doc(alias = "VK_COLOR_SPACE_HDR10_ST2084_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const HDR10_ST2084_EXT: Self = Self(1000104008);
    #[doc(alias = "VK_COLOR_SPACE_DOLBYVISION_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DOLBYVISION_EXT: Self = Self(1000104009);
    #[doc(alias = "VK_COLOR_SPACE_HDR10_HLG_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const HDR10_HLG_EXT: Self = Self(1000104010);
    #[doc(alias = "VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    #[doc(alias = "VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    #[doc(alias = "VK_COLOR_SPACE_PASS_THROUGH_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const PASS_THROUGH_EXT: Self = Self(1000104013);
    #[doc(alias = "VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    #[doc(alias = "VK_COLOR_SPACE_DISPLAY_NATIVE_AMD")]
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
    #[doc(alias = "VK_COLORSPACE_SRGB_NONLINEAR_KHR")]
    pub const COLORSPACE_SRGB_NONLINEAR: Self = Self::SRGB_NONLINEAR;
    #[doc(alias = "VK_COLOR_SPACE_DCI_P3_LINEAR_EXT")]
    #[cfg(feature = "VK_EXT_swapchain_colorspace")]
    pub const DCI_P3_LINEAR_EXT: Self = Self::DISPLAY_P3_LINEAR_EXT;
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
            x if x == Self::SRGB_NONLINEAR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DISPLAY_P3_NONLINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::EXTENDED_SRGB_LINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DISPLAY_P3_LINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DCI_P3_NONLINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::BT709_LINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::BT709_NONLINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::BT2020_LINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::HDR10_ST2084_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::DOLBYVISION_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::HDR10_HLG_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::ADOBERGB_LINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::ADOBERGB_NONLINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::PASS_THROUGH_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_swapchain_colorspace")]
            x if x == Self::EXTENDED_SRGB_NONLINEAR_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_AMD_display_native_hdr")]
            x if x == Self::DISPLAY_NATIVE_AMD.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkDestroySurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/vkDestroySurfaceKHR.md")]
#[doc(alias = "vkDestroySurfaceKHR")]
pub type FNDestroySurfaceKhr =
    unsafe extern "system" fn(instance: Instance, surface: SurfaceKHR, p_allocator: *const AllocationCallbacks);
///# [vkGetPhysicalDeviceSurfaceSupportKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/vkGetPhysicalDeviceSurfaceSupportKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceSurfaceSupportKHR")]
pub type FNGetPhysicalDeviceSurfaceSupportKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    p_supported: *mut Bool32,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilitiesKHR")]
pub type FNGetPhysicalDeviceSurfaceCapabilitiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceSurfaceFormatsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/vkGetPhysicalDeviceSurfaceFormatsKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceSurfaceFormatsKHR")]
pub type FNGetPhysicalDeviceSurfaceFormatsKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormatKHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_surface/vkGetPhysicalDeviceSurfacePresentModesKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModesKHR")]
pub type FNGetPhysicalDeviceSurfacePresentModesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> VulkanResultCodes;
