//!# [VK_KHR_video_decode_queue](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_decode_queue.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/VK_KHR_video_decode_queue.md")]
use crate::{
    cstr,
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Buffer, CommandBuffer, DeviceSize, Extent2D, Offset2D, StructureType,
    },
};
use std::ffi::CStr;
///# [VkVideoDecodeCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/VkVideoDecodeCapabilitiesKHR.md")]
#[doc(alias = "VkVideoDecodeCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    flags: VideoDecodeCapabilityFlagsKHR,
}
///# [VkVideoDecodeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/VkVideoDecodeInfoKHR.md")]
#[doc(alias = "VkVideoDecodeInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoDecodeInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: VideoDecodeFlagsKHR,
    #[doc(alias = "codedOffset")]
    coded_offset: Offset2D,
    #[doc(alias = "codedExtent")]
    coded_extent: Extent2D,
    #[doc(alias = "srcBuffer")]
    src_buffer: Buffer,
    #[doc(alias = "srcBufferOffset")]
    src_buffer_offset: DeviceSize,
    #[doc(alias = "srcBufferRange")]
    src_buffer_range: DeviceSize,
    #[doc(alias = "dstPictureResource")]
    dst_picture_resource: VideoPictureResourceKHR,
    #[doc(alias = "pSetupReferenceSlot")]
    setup_reference_slot: *const VideoReferenceSlotKHR,
    #[doc(alias = "referenceSlotCount")]
    reference_slot_count: u32,
    #[doc(alias = "pReferenceSlots")]
    reference_slots: *const VideoReferenceSlotKHR,
}
///# [VkVideoDecodeCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/VkVideoDecodeCapabilityFlagBitsKHR.md")]
#[doc(alias = "VkVideoDecodeCapabilityFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoDecodeCapabilityFlagsKHR(u32);
impl VideoDecodeCapabilityFlagsKHR {
    #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR")]
    pub const DPB_AND_OUTPUT_COINCIDE: Self = Self(1);
    #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR")]
    pub const DPB_AND_OUTPUT_DISTINCT: Self = Self(2);
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
            all |= Self::DEFAULT;
        }
        {
            all |= Self::DPB_AND_OUTPUT_COINCIDE;
        }
        {
            all |= Self::DPB_AND_OUTPUT_DISTINCT;
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
impl const std::ops::BitOr for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoDecodeCapabilityFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoDecodeCapabilityFlagsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeCapabilityFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<VideoDecodeCapabilityFlagsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeCapabilityFlagsKHR>>(iterator: T) -> VideoDecodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeCapabilityFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoDecodeCapabilityFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn from(bit: VideoDecodeCapabilityFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeCapabilityFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeCapabilityFlagBitsKHR>>(
        iterator: T,
    ) -> VideoDecodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeCapabilityFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoDecodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoDecodeCapabilityFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoDecodeCapabilityFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoDecodeCapabilityFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoDecodeCapabilityFlagsKHR::DPB_AND_OUTPUT_COINCIDE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DPB_AND_OUTPUT_COINCIDE))?;
                    }
                    if self.0.contains(VideoDecodeCapabilityFlagsKHR::DPB_AND_OUTPUT_DISTINCT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DPB_AND_OUTPUT_DISTINCT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoDecodeCapabilityFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkVideoDecodeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/VkVideoDecodeFlagBitsKHR.md")]
#[doc(alias = "VkVideoDecodeFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoDecodeFlagsKHR(u32);
impl VideoDecodeFlagsKHR {
    #[doc(alias = "VK_VIDEO_DECODE_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_DECODE_RESERVED_0_BIT_KHR")]
    pub const RESERVED0: Self = Self(1);
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
            all |= Self::DEFAULT;
        }
        {
            all |= Self::RESERVED0;
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
impl const std::ops::BitOr for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoDecodeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoDecodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoDecodeFlagsKHR> for VideoDecodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<VideoDecodeFlagsKHR> for VideoDecodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeFlagsKHR>>(iterator: T) -> VideoDecodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoDecodeFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoDecodeFlagBitsKHR> for VideoDecodeFlagsKHR {
    fn from(bit: VideoDecodeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoDecodeFlagBitsKHR> for VideoDecodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoDecodeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<VideoDecodeFlagBitsKHR> for VideoDecodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoDecodeFlagBitsKHR>>(iterator: T) -> VideoDecodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoDecodeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoDecodeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoDecodeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoDecodeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoDecodeFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoDecodeFlagsKHR::RESERVED0) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RESERVED0))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoDecodeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_video_decode_queue");
///# [VkVideoDecodeCapabilityFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/VkVideoDecodeCapabilityFlagBitsKHR.md")]
#[doc(alias = "VkVideoDecodeCapabilityFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoDecodeCapabilityFlagBitsKHR(u32);
impl VideoDecodeCapabilityFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR")]
    pub const DPB_AND_OUTPUT_COINCIDE: Self = Self(1);
    #[doc(alias = "VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR")]
    pub const DPB_AND_OUTPUT_DISTINCT: Self = Self(2);
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
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::DPB_AND_OUTPUT_COINCIDE.bits() => Some(Self(x)),
            x if x == Self::DPB_AND_OUTPUT_DISTINCT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkVideoDecodeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkVideoDecodeFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/VkVideoDecodeFlagBitsKHR.md")]
#[doc(alias = "VkVideoDecodeFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoDecodeFlagBitsKHR(u32);
impl VideoDecodeFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_DECODE_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_DECODE_RESERVED_0_BIT_KHR")]
    pub const RESERVED0: Self = Self(1);
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
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::RESERVED0.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [vkCmdDecodeVideoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecodeVideoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_video_decode_queue/vkCmdDecodeVideoKHR.md")]
#[doc(alias = "vkCmdDecodeVideoKHR")]
pub type FNCmdDecodeVideoKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_frame_info: *const VideoDecodeInfoKHR);
