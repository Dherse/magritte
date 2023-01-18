use crate::{
    cstr,
    extensions::khr_video_queue::{VideoPictureResourceKHR, VideoReferenceSlotKHR},
    vulkan1_0::{BaseInStructure, Buffer, CommandBuffer, DeviceSize, Extent2D, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkVideoEncodeInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: VideoEncodeFlagsKHR,
    #[doc(alias = "qualityLevel")]
    quality_level: u32,
    #[doc(alias = "codedExtent")]
    coded_extent: Extent2D,
    #[doc(alias = "dstBitstreamBuffer")]
    dst_bitstream_buffer: Buffer,
    #[doc(alias = "dstBitstreamBufferOffset")]
    dst_bitstream_buffer_offset: DeviceSize,
    #[doc(alias = "dstBitstreamBufferMaxRange")]
    dst_bitstream_buffer_max_range: DeviceSize,
    #[doc(alias = "srcPictureResource")]
    src_picture_resource: VideoPictureResourceKHR,
    #[doc(alias = "pSetupReferenceSlot")]
    setup_reference_slot: *const VideoReferenceSlotKHR,
    #[doc(alias = "referenceSlotCount")]
    reference_slot_count: u32,
    #[doc(alias = "pReferenceSlots")]
    reference_slots: *const VideoReferenceSlotKHR,
    #[doc(alias = "precedingExternallyEncodedBytes")]
    preceding_externally_encoded_bytes: u32,
}
#[doc(alias = "VkVideoEncodeRateControlInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeRateControlInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: VideoEncodeRateControlFlagsKHR,
    #[doc(alias = "rateControlMode")]
    rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    #[doc(alias = "layerCount")]
    layer_count: u8,
    #[doc(alias = "pLayerConfigs")]
    layer_configs: *const VideoEncodeRateControlLayerInfoKHR,
}
#[doc(alias = "VkVideoEncodeRateControlLayerInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeRateControlLayerInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "averageBitrate")]
    average_bitrate: u32,
    #[doc(alias = "maxBitrate")]
    max_bitrate: u32,
    #[doc(alias = "frameRateNumerator")]
    frame_rate_numerator: u32,
    #[doc(alias = "frameRateDenominator")]
    frame_rate_denominator: u32,
    #[doc(alias = "virtualBufferSizeInMs")]
    virtual_buffer_size_in_ms: u32,
    #[doc(alias = "initialVirtualBufferSizeInMs")]
    initial_virtual_buffer_size_in_ms: u32,
}
#[doc(alias = "VkVideoEncodeCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VideoEncodeCapabilitiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: VideoEncodeCapabilityFlagsKHR,
    #[doc(alias = "rateControlModes")]
    rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    #[doc(alias = "rateControlLayerCount")]
    rate_control_layer_count: u8,
    #[doc(alias = "qualityLevelCount")]
    quality_level_count: u8,
    #[doc(alias = "inputImageDataFillAlignment")]
    input_image_data_fill_alignment: Extent2D,
}
#[doc(alias = "VkVideoEncodeFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeFlagsKHR(u32);
impl VideoEncodeFlagsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_RESERVED_0_BIT_KHR")]
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
impl const std::ops::BitOr for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeFlagsKHR> for VideoEncodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<VideoEncodeFlagsKHR> for VideoEncodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeFlagsKHR>>(iterator: T) -> VideoEncodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoEncodeFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn from(bit: VideoEncodeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeFlagBitsKHR>>(iterator: T) -> VideoEncodeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoEncodeFlagsKHR::RESERVED0) {
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
        f.debug_tuple(stringify!(VideoEncodeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkVideoEncodeCapabilityFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeCapabilityFlagsKHR(u32);
impl VideoEncodeCapabilityFlagsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR")]
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES: Self = Self(1);
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
            all |= Self::PRECEDING_EXTERNALLY_ENCODED_BYTES;
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
impl const std::ops::BitOr for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeCapabilityFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeCapabilityFlagsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeCapabilityFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<VideoEncodeCapabilityFlagsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeCapabilityFlagsKHR>>(iterator: T) -> VideoEncodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeCapabilityFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoEncodeCapabilityFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn from(bit: VideoEncodeCapabilityFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeCapabilityFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeCapabilityFlagBitsKHR>>(
        iterator: T,
    ) -> VideoEncodeCapabilityFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeCapabilityFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeCapabilityFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeCapabilityFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeCapabilityFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeCapabilityFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self
                        .0
                        .contains(VideoEncodeCapabilityFlagsKHR::PRECEDING_EXTERNALLY_ENCODED_BYTES)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PRECEDING_EXTERNALLY_ENCODED_BYTES))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeCapabilityFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkVideoEncodeRateControlFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeRateControlFlagsKHR(u32);
impl VideoEncodeRateControlFlagsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_RESERVED_0_BIT_KHR")]
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
impl const std::ops::BitOr for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeRateControlFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeRateControlFlagsKHR> for VideoEncodeRateControlFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<VideoEncodeRateControlFlagsKHR> for VideoEncodeRateControlFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlFlagsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoEncodeRateControlFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoEncodeRateControlFlagBitsKHR> for VideoEncodeRateControlFlagsKHR {
    fn from(bit: VideoEncodeRateControlFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoEncodeRateControlFlagBitsKHR> for VideoEncodeRateControlFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<VideoEncodeRateControlFlagBitsKHR> for VideoEncodeRateControlFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlFlagBitsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeRateControlFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeRateControlFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeRateControlFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeRateControlFlagsKHR::DEFAULT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEFAULT))?;
                    }
                    if self.0.contains(VideoEncodeRateControlFlagsKHR::RESERVED0) {
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
        f.debug_tuple(stringify!(VideoEncodeRateControlFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VkVideoEncodeRateControlModeFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoEncodeRateControlModeFlagsKHR(u32);
impl VideoEncodeRateControlModeFlagsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR")]
    pub const CBR: Self = Self(1);
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR")]
    pub const VBR: Self = Self(2);
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
            all |= Self::NONE;
        }
        {
            all |= Self::CBR;
        }
        {
            all |= Self::VBR;
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
impl const std::ops::BitOr for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for VideoEncodeRateControlModeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<VideoEncodeRateControlModeFlagsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlModeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<VideoEncodeRateControlModeFlagsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlModeFlagsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlModeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for VideoEncodeRateControlModeFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<VideoEncodeRateControlModeFlagBitsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn from(bit: VideoEncodeRateControlModeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<VideoEncodeRateControlModeFlagBitsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn extend<T: IntoIterator<Item = VideoEncodeRateControlModeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<VideoEncodeRateControlModeFlagBitsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = VideoEncodeRateControlModeFlagBitsKHR>>(
        iterator: T,
    ) -> VideoEncodeRateControlModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<VideoEncodeRateControlModeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for VideoEncodeRateControlModeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(VideoEncodeRateControlModeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == VideoEncodeRateControlModeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(VideoEncodeRateControlModeFlagsKHR::NONE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NONE))?;
                    }
                    if self.0.contains(VideoEncodeRateControlModeFlagsKHR::CBR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CBR))?;
                    }
                    if self.0.contains(VideoEncodeRateControlModeFlagsKHR::VBR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VBR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(VideoEncodeRateControlModeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_video_encode_queue");
#[doc(alias = "VkVideoEncodeFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoEncodeFlagBitsKHR(u32);
impl VideoEncodeFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_RESERVED_0_BIT_KHR")]
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
#[doc(alias = "VkVideoEncodeCapabilityFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoEncodeCapabilityFlagBitsKHR(u32);
impl VideoEncodeCapabilityFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR")]
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES: Self = Self(1);
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
            x if x == Self::PRECEDING_EXTERNALLY_ENCODED_BYTES.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkVideoEncodeRateControlFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoEncodeRateControlFlagBitsKHR(u32);
impl VideoEncodeRateControlFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_DEFAULT_KHR")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_RESERVED_0_BIT_KHR")]
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
#[doc(alias = "VkVideoEncodeRateControlModeFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct VideoEncodeRateControlModeFlagBitsKHR(u32);
impl VideoEncodeRateControlModeFlagBitsKHR {
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_NONE_BIT_KHR")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR")]
    pub const CBR: Self = Self(1);
    #[doc(alias = "VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR")]
    pub const VBR: Self = Self(2);
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
            x if x == Self::NONE.bits() => Some(Self(x)),
            x if x == Self::CBR.bits() => Some(Self(x)),
            x if x == Self::VBR.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkCmdEncodeVideoKHR")]
pub type FNCmdEncodeVideoKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_encode_info: *const VideoEncodeInfoKHR);
