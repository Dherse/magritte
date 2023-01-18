use crate::{
    cstr,
    extensions::{
        khr_display::SurfaceTransformFlagsKHR,
        khr_surface::{CompositeAlphaFlagsKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR},
    },
    vulkan1_0::{BaseOutStructure, Extent2D, ImageUsageFlags, PhysicalDevice, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkSurfaceCapabilities2EXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilities2EXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
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
    #[doc(alias = "supportedSurfaceCounters")]
    supported_surface_counters: SurfaceCounterFlagsEXT,
}
#[doc(alias = "VkSurfaceCounterFlagsEXT")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceCounterFlagsEXT(u32);
impl SurfaceCounterFlagsEXT {
    #[doc(alias = "VK_SURFACE_COUNTER_VBLANK_BIT_EXT")]
    pub const VBLANK: Self = Self(1);
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
            all |= Self::VBLANK;
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
impl const std::ops::BitOr for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SurfaceCounterFlagsEXT> for SurfaceCounterFlagsEXT {
    fn extend<T: IntoIterator<Item = SurfaceCounterFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SurfaceCounterFlagsEXT> for SurfaceCounterFlagsEXT {
    fn from_iter<T: IntoIterator<Item = SurfaceCounterFlagsEXT>>(iterator: T) -> SurfaceCounterFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<SurfaceCounterFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for SurfaceCounterFlagsEXT {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn from(bit: SurfaceCounterFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn extend<T: IntoIterator<Item = SurfaceCounterFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn from_iter<T: IntoIterator<Item = SurfaceCounterFlagBitsEXT>>(iterator: T) -> SurfaceCounterFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<SurfaceCounterFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SurfaceCounterFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SurfaceCounterFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SurfaceCounterFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SurfaceCounterFlagsEXT::VBLANK) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VBLANK))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SurfaceCounterFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION")]
pub const EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME")]
pub const EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_display_surface_counter");
#[doc(alias = "VkSurfaceCounterFlagBitsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SurfaceCounterFlagBitsEXT(u32);
impl SurfaceCounterFlagBitsEXT {
    #[doc(alias = "VK_SURFACE_COUNTER_VBLANK_BIT_EXT")]
    pub const VBLANK: Self = Self(1);
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
            x if x == Self::VBLANK.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
pub type FNGetPhysicalDeviceSurfaceCapabilities2Ext = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> VulkanResultCodes;
