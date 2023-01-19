//!# [VK_KHR_display](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_display/VK_KHR_display.md")]
use crate::{
    cstr,
    extensions::khr_surface::{SurfaceKHR, SurfaceTransformFlagBitsKHR},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Extent2D, Instance, Offset2D, PhysicalDevice, StructureType,
        VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkDisplayPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayPropertiesKHR.md")]
#[doc(alias = "VkDisplayPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPropertiesKHR {
    display: DisplayKHR,
    #[doc(alias = "displayName")]
    display_name: *const CStr,
    #[doc(alias = "physicalDimensions")]
    physical_dimensions: Extent2D,
    #[doc(alias = "physicalResolution")]
    physical_resolution: Extent2D,
    #[doc(alias = "supportedTransforms")]
    supported_transforms: SurfaceTransformFlagsKHR,
    #[doc(alias = "planeReorderPossible")]
    plane_reorder_possible: Bool32,
    #[doc(alias = "persistentContent")]
    persistent_content: Bool32,
}
///# [VkDisplayPlanePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayPlanePropertiesKHR.md")]
#[doc(alias = "VkDisplayPlanePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlanePropertiesKHR {
    #[doc(alias = "currentDisplay")]
    current_display: DisplayKHR,
    #[doc(alias = "currentStackIndex")]
    current_stack_index: u32,
}
///# [VkDisplayModeParametersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayModeParametersKHR.md")]
#[doc(alias = "VkDisplayModeParametersKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayModeParametersKHR {
    #[doc(alias = "visibleRegion")]
    visible_region: Extent2D,
    #[doc(alias = "refreshRate")]
    refresh_rate: u32,
}
///# [VkDisplayModePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayModePropertiesKHR.md")]
#[doc(alias = "VkDisplayModePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayModePropertiesKHR {
    #[doc(alias = "displayMode")]
    display_mode: DisplayModeKHR,
    parameters: DisplayModeParametersKHR,
}
///# [VkDisplayModeCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayModeCreateInfoKHR.md")]
#[doc(alias = "VkDisplayModeCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayModeCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DisplayModeCreateFlagsKHR,
    parameters: DisplayModeParametersKHR,
}
///# [VkDisplayPlaneCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayPlaneCapabilitiesKHR.md")]
#[doc(alias = "VkDisplayPlaneCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneCapabilitiesKHR {
    #[doc(alias = "supportedAlpha")]
    supported_alpha: DisplayPlaneAlphaFlagsKHR,
    #[doc(alias = "minSrcPosition")]
    min_src_position: Offset2D,
    #[doc(alias = "maxSrcPosition")]
    max_src_position: Offset2D,
    #[doc(alias = "minSrcExtent")]
    min_src_extent: Extent2D,
    #[doc(alias = "maxSrcExtent")]
    max_src_extent: Extent2D,
    #[doc(alias = "minDstPosition")]
    min_dst_position: Offset2D,
    #[doc(alias = "maxDstPosition")]
    max_dst_position: Offset2D,
    #[doc(alias = "minDstExtent")]
    min_dst_extent: Extent2D,
    #[doc(alias = "maxDstExtent")]
    max_dst_extent: Extent2D,
}
///# [VkDisplaySurfaceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplaySurfaceCreateInfoKHR.md")]
#[doc(alias = "VkDisplaySurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DisplaySurfaceCreateFlagsKHR,
    #[doc(alias = "displayMode")]
    display_mode: DisplayModeKHR,
    #[doc(alias = "planeIndex")]
    plane_index: u32,
    #[doc(alias = "planeStackIndex")]
    plane_stack_index: u32,
    transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "globalAlpha")]
    global_alpha: f32,
    #[doc(alias = "alphaMode")]
    alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    #[doc(alias = "imageExtent")]
    image_extent: Extent2D,
}
///# [VkDisplayKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayKHR.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDisplayKHR")]
#[repr(transparent)]
pub struct DisplayKHR(u64);
impl DisplayKHR {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for DisplayKHR {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkDisplayModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayModeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayModeKHR.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDisplayModeKHR")]
#[repr(transparent)]
pub struct DisplayModeKHR(u64);
impl DisplayModeKHR {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for DisplayModeKHR {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkDisplayPlaneAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayPlaneAlphaFlagBitsKHR.md")]
#[doc(alias = "VkDisplayPlaneAlphaFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayPlaneAlphaFlagsKHR(u32);
impl DisplayPlaneAlphaFlagsKHR {
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR")]
    pub const GLOBAL: Self = Self(2);
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR")]
    pub const PER_PIXEL: Self = Self(4);
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR")]
    pub const PER_PIXEL_PREMULTIPLIED: Self = Self(8);
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
            all |= Self::GLOBAL;
        }
        {
            all |= Self::PER_PIXEL;
        }
        {
            all |= Self::PER_PIXEL_PREMULTIPLIED;
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
impl const std::ops::BitOr for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DisplayPlaneAlphaFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DisplayPlaneAlphaFlagsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = DisplayPlaneAlphaFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DisplayPlaneAlphaFlagsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DisplayPlaneAlphaFlagsKHR>>(iterator: T) -> DisplayPlaneAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DisplayPlaneAlphaFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for DisplayPlaneAlphaFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn from(bit: DisplayPlaneAlphaFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn extend<T: IntoIterator<Item = DisplayPlaneAlphaFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DisplayPlaneAlphaFlagBitsKHR>>(iterator: T) -> DisplayPlaneAlphaFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DisplayPlaneAlphaFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DisplayPlaneAlphaFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DisplayPlaneAlphaFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DisplayPlaneAlphaFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DisplayPlaneAlphaFlagsKHR::OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE))?;
                    }
                    if self.0.contains(DisplayPlaneAlphaFlagsKHR::GLOBAL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GLOBAL))?;
                    }
                    if self.0.contains(DisplayPlaneAlphaFlagsKHR::PER_PIXEL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PER_PIXEL))?;
                    }
                    if self.0.contains(DisplayPlaneAlphaFlagsKHR::PER_PIXEL_PREMULTIPLIED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PER_PIXEL_PREMULTIPLIED))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DisplayPlaneAlphaFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkSurfaceTransformFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceTransformFlagsKHR(u32);
impl SurfaceTransformFlagsKHR {
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
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::IDENTITY;
        }
        {
            all |= Self::ROTATE90;
        }
        {
            all |= Self::ROTATE180;
        }
        {
            all |= Self::ROTATE270;
        }
        {
            all |= Self::HORIZONTAL_MIRROR;
        }
        {
            all |= Self::HORIZONTAL_MIRROR_ROTATE90;
        }
        {
            all |= Self::HORIZONTAL_MIRROR_ROTATE180;
        }
        {
            all |= Self::HORIZONTAL_MIRROR_ROTATE270;
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
impl const std::ops::BitOr for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SurfaceTransformFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SurfaceTransformFlagsKHR> for SurfaceTransformFlagsKHR {
    fn extend<T: IntoIterator<Item = SurfaceTransformFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SurfaceTransformFlagsKHR> for SurfaceTransformFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SurfaceTransformFlagsKHR>>(iterator: T) -> SurfaceTransformFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SurfaceTransformFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for SurfaceTransformFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn from(bit: SurfaceTransformFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn extend<T: IntoIterator<Item = SurfaceTransformFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SurfaceTransformFlagBitsKHR>>(iterator: T) -> SurfaceTransformFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SurfaceTransformFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SurfaceTransformFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SurfaceTransformFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SurfaceTransformFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SurfaceTransformFlagsKHR::IDENTITY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IDENTITY))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::ROTATE90) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ROTATE90))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::ROTATE180) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ROTATE180))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::ROTATE270) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ROTATE270))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HORIZONTAL_MIRROR))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE90) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HORIZONTAL_MIRROR_ROTATE90))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE180) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HORIZONTAL_MIRROR_ROTATE180))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE270) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HORIZONTAL_MIRROR_ROTATE270))?;
                    }
                    if self.0.contains(SurfaceTransformFlagsKHR::INHERIT) {
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
        f.debug_tuple(stringify!(SurfaceTransformFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkDisplayModeCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplayModeCreateFlagsKHR(u32);
impl DisplayModeCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VkDisplaySurfaceCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisplaySurfaceCreateFlagsKHR(u32);
impl DisplaySurfaceCreateFlagsKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_KHR_DISPLAY_SPEC_VERSION")]
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;
#[doc(alias = "VK_KHR_DISPLAY_EXTENSION_NAME")]
pub const KHR_DISPLAY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_display");
///# [VkDisplayPlaneAlphaFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/VkDisplayPlaneAlphaFlagBitsKHR.md")]
#[doc(alias = "VkDisplayPlaneAlphaFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DisplayPlaneAlphaFlagBitsKHR(u32);
impl DisplayPlaneAlphaFlagBitsKHR {
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR")]
    pub const GLOBAL: Self = Self(2);
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR")]
    pub const PER_PIXEL: Self = Self(4);
    #[doc(alias = "VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR")]
    pub const PER_PIXEL_PREMULTIPLIED: Self = Self(8);
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
            x if x == Self::GLOBAL.bits() => Some(Self(x)),
            x if x == Self::PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::PER_PIXEL_PREMULTIPLIED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [vkGetPhysicalDeviceDisplayPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/vkGetPhysicalDeviceDisplayPropertiesKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
pub type FNGetPhysicalDeviceDisplayPropertiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPropertiesKHR,
) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceDisplayPlanePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
pub type FNGetPhysicalDeviceDisplayPlanePropertiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlanePropertiesKHR,
) -> VulkanResultCodes;
///# [vkGetDisplayPlaneSupportedDisplaysKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/vkGetDisplayPlaneSupportedDisplaysKHR.md")]
#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
pub type FNGetDisplayPlaneSupportedDisplaysKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    plane_index: u32,
    p_display_count: *mut u32,
    p_displays: *mut DisplayKHR,
) -> VulkanResultCodes;
///# [vkGetDisplayModePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/vkGetDisplayModePropertiesKHR.md")]
#[doc(alias = "vkGetDisplayModePropertiesKHR")]
pub type FNGetDisplayModePropertiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModePropertiesKHR,
) -> VulkanResultCodes;
///# [vkCreateDisplayModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/vkCreateDisplayModeKHR.md")]
#[doc(alias = "vkCreateDisplayModeKHR")]
pub type FNCreateDisplayModeKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_create_info: *const DisplayModeCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_mode: *mut DisplayModeKHR,
) -> VulkanResultCodes;
///# [vkGetDisplayPlaneCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/vkGetDisplayPlaneCapabilitiesKHR.md")]
#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
pub type FNGetDisplayPlaneCapabilitiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> VulkanResultCodes;
///# [vkCreateDisplayPlaneSurfaceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display/vkCreateDisplayPlaneSurfaceKHR.md")]
#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
pub type FNCreateDisplayPlaneSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DisplaySurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
