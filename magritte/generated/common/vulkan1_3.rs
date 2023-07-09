#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[doc(alias = "VkPipelineCreationFeedback")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineCreationFeedback {
    pub flags: PipelineCreationFeedbackFlags,
    pub duration: u64,
}
#[doc(alias = "VkPrivateDataSlotCreateFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PrivateDataSlotCreateFlags(u32);
impl PrivateDataSlotCreateFlags {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for PrivateDataSlotCreateFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PrivateDataSlotCreateFlags {
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
unsafe impl crate::conv::FromLowLevel for PrivateDataSlotCreateFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPipelineCreationFeedbackFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCreationFeedbackFlags(u32);
impl PipelineCreationFeedbackFlags {
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT")]
    pub const VALID: Self = Self(1);
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT")]
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(2);
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT")]
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(4);
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT")]
    pub const VALID_EXT: Self = Self::VALID;
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT")]
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT")]
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
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
            all |= Self::VALID;
        }
        {
            all |= Self::APPLICATION_PIPELINE_CACHE_HIT;
        }
        {
            all |= Self::BASE_PIPELINE_ACCELERATION;
        }
        {
            all |= Self::VALID_EXT;
        }
        {
            all |= Self::APPLICATION_PIPELINE_CACHE_HIT_EXT;
        }
        {
            all |= Self::BASE_PIPELINE_ACCELERATION_EXT;
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
impl std::ops::BitOr for PipelineCreationFeedbackFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for PipelineCreationFeedbackFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for PipelineCreationFeedbackFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for PipelineCreationFeedbackFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for PipelineCreationFeedbackFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for PipelineCreationFeedbackFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for PipelineCreationFeedbackFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for PipelineCreationFeedbackFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for PipelineCreationFeedbackFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PipelineCreationFeedbackFlags> for PipelineCreationFeedbackFlags {
    fn extend<T: IntoIterator<Item = PipelineCreationFeedbackFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<PipelineCreationFeedbackFlags> for PipelineCreationFeedbackFlags {
    fn from_iter<T: IntoIterator<Item = PipelineCreationFeedbackFlags>>(iterator: T) -> PipelineCreationFeedbackFlags {
        let mut out = Self::empty();
        <Self as Extend<PipelineCreationFeedbackFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for PipelineCreationFeedbackFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<PipelineCreationFeedbackFlagBits> for PipelineCreationFeedbackFlags {
    fn from(bit: PipelineCreationFeedbackFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<PipelineCreationFeedbackFlagBits> for PipelineCreationFeedbackFlags {
    fn extend<T: IntoIterator<Item = PipelineCreationFeedbackFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<PipelineCreationFeedbackFlagBits> for PipelineCreationFeedbackFlags {
    fn from_iter<T: IntoIterator<Item = PipelineCreationFeedbackFlagBits>>(
        iterator: T,
    ) -> PipelineCreationFeedbackFlags {
        let mut out = Self::empty();
        <Self as Extend<PipelineCreationFeedbackFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PipelineCreationFeedbackFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PipelineCreationFeedbackFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PipelineCreationFeedbackFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(PipelineCreationFeedbackFlags::VALID) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VALID))?;
                    }
                    if self
                        .0
                        .contains(PipelineCreationFeedbackFlags::APPLICATION_PIPELINE_CACHE_HIT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(APPLICATION_PIPELINE_CACHE_HIT))?;
                    }
                    if self
                        .0
                        .contains(PipelineCreationFeedbackFlags::BASE_PIPELINE_ACCELERATION)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(BASE_PIPELINE_ACCELERATION))?;
                    }
                    if self.0.contains(PipelineCreationFeedbackFlags::VALID_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VALID_EXT))?;
                    }
                    if self
                        .0
                        .contains(PipelineCreationFeedbackFlags::APPLICATION_PIPELINE_CACHE_HIT_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(APPLICATION_PIPELINE_CACHE_HIT_EXT))?;
                    }
                    if self
                        .0
                        .contains(PipelineCreationFeedbackFlags::BASE_PIPELINE_ACCELERATION_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(BASE_PIPELINE_ACCELERATION_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PipelineCreationFeedbackFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCreationFeedbackFlags {
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
unsafe impl crate::conv::FromLowLevel for PipelineCreationFeedbackFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccessFlags2")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccessFlags2(u64);
impl AccessFlags2 {
    #[doc(alias = "VK_ACCESS_2_NONE")]
    pub const ACCESS2_NONE: Self = Self(0);
    #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT")]
    pub const ACCESS2_INDIRECT_COMMAND_READ: Self = Self(1);
    #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT")]
    pub const ACCESS2_INDEX_READ: Self = Self(2);
    #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT")]
    pub const ACCESS2_VERTEX_ATTRIBUTE_READ: Self = Self(4);
    #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT")]
    pub const ACCESS2_UNIFORM_READ: Self = Self(8);
    #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT")]
    pub const ACCESS2_INPUT_ATTACHMENT_READ: Self = Self(16);
    #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT")]
    pub const ACCESS2_SHADER_READ: Self = Self(32);
    #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT")]
    pub const ACCESS2_SHADER_WRITE: Self = Self(64);
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT")]
    pub const ACCESS2_COLOR_ATTACHMENT_READ: Self = Self(128);
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT")]
    pub const ACCESS2_COLOR_ATTACHMENT_WRITE: Self = Self(256);
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512);
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024);
    #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT")]
    pub const ACCESS2_TRANSFER_READ: Self = Self(2048);
    #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT")]
    pub const ACCESS2_TRANSFER_WRITE: Self = Self(4096);
    #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT")]
    pub const ACCESS2_HOST_READ: Self = Self(8192);
    #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT")]
    pub const ACCESS2_HOST_WRITE: Self = Self(16384);
    #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT")]
    pub const ACCESS2_MEMORY_READ: Self = Self(32768);
    #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT")]
    pub const ACCESS2_MEMORY_WRITE: Self = Self(65536);
    #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT")]
    pub const ACCESS2_SHADER_SAMPLED_READ: Self = Self(4294967296);
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT")]
    pub const ACCESS2_SHADER_STORAGE_READ: Self = Self(8589934592);
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT")]
    pub const ACCESS2_SHADER_STORAGE_WRITE: Self = Self(17179869184);
    #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const ACCESS2_VIDEO_DECODE_READ_KHR: Self = Self(34359738368);
    #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const ACCESS2_VIDEO_DECODE_WRITE_KHR: Self = Self(68719476736);
    #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const ACCESS2_VIDEO_ENCODE_READ_KHR: Self = Self(137438953472);
    #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const ACCESS2_VIDEO_ENCODE_WRITE_KHR: Self = Self(274877906944);
    #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(33554432);
    #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(67108864);
    #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(134217728);
    #[doc(alias = "VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576);
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_COMMAND_PREPROCESS_READ_NV: Self = Self(131072);
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_COMMAND_PREPROCESS_WRITE_NV: Self = Self(262144);
    #[doc(alias = "VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608);
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152);
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304);
    #[doc(alias = "VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216);
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288);
    #[doc(alias = "VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI")]
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    pub const ACCESS2_INVOCATION_MASK_READ_HUAWEI: Self = Self(549755813888);
    #[doc(alias = "VK_ACCESS_2_NONE_KHR")]
    pub const ACCESS2_NONE_KHR: Self = Self::ACCESS2_NONE;
    #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR")]
    pub const ACCESS2_INDIRECT_COMMAND_READ_KHR: Self = Self::ACCESS2_INDIRECT_COMMAND_READ;
    #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT_KHR")]
    pub const ACCESS2_INDEX_READ_KHR: Self = Self::ACCESS2_INDEX_READ;
    #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR")]
    pub const ACCESS2_VERTEX_ATTRIBUTE_READ_KHR: Self = Self::ACCESS2_VERTEX_ATTRIBUTE_READ;
    #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT_KHR")]
    pub const ACCESS2_UNIFORM_READ_KHR: Self = Self::ACCESS2_UNIFORM_READ;
    #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR")]
    pub const ACCESS2_INPUT_ATTACHMENT_READ_KHR: Self = Self::ACCESS2_INPUT_ATTACHMENT_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT_KHR")]
    pub const ACCESS2_SHADER_READ_KHR: Self = Self::ACCESS2_SHADER_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT_KHR")]
    pub const ACCESS2_SHADER_WRITE_KHR: Self = Self::ACCESS2_SHADER_WRITE;
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR")]
    pub const ACCESS2_COLOR_ATTACHMENT_READ_KHR: Self = Self::ACCESS2_COLOR_ATTACHMENT_READ;
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR")]
    pub const ACCESS2_COLOR_ATTACHMENT_WRITE_KHR: Self = Self::ACCESS2_COLOR_ATTACHMENT_WRITE;
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ;
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE;
    #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT_KHR")]
    pub const ACCESS2_TRANSFER_READ_KHR: Self = Self::ACCESS2_TRANSFER_READ;
    #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR")]
    pub const ACCESS2_TRANSFER_WRITE_KHR: Self = Self::ACCESS2_TRANSFER_WRITE;
    #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT_KHR")]
    pub const ACCESS2_HOST_READ_KHR: Self = Self::ACCESS2_HOST_READ;
    #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT_KHR")]
    pub const ACCESS2_HOST_WRITE_KHR: Self = Self::ACCESS2_HOST_WRITE;
    #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT_KHR")]
    pub const ACCESS2_MEMORY_READ_KHR: Self = Self::ACCESS2_MEMORY_READ;
    #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT_KHR")]
    pub const ACCESS2_MEMORY_WRITE_KHR: Self = Self::ACCESS2_MEMORY_WRITE;
    #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR")]
    pub const ACCESS2_SHADER_SAMPLED_READ_KHR: Self = Self::ACCESS2_SHADER_SAMPLED_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR")]
    pub const ACCESS2_SHADER_STORAGE_READ_KHR: Self = Self::ACCESS2_SHADER_STORAGE_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR")]
    pub const ACCESS2_SHADER_STORAGE_WRITE_KHR: Self = Self::ACCESS2_SHADER_STORAGE_WRITE;
    #[doc(alias = "VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_SHADING_RATE_IMAGE_READ_NV: Self = Self::ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCESS2_ACCELERATION_STRUCTURE_READ_KHR;
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR;
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
            all |= Self::ACCESS2_NONE;
        }
        {
            all |= Self::ACCESS2_INDIRECT_COMMAND_READ;
        }
        {
            all |= Self::ACCESS2_INDEX_READ;
        }
        {
            all |= Self::ACCESS2_VERTEX_ATTRIBUTE_READ;
        }
        {
            all |= Self::ACCESS2_UNIFORM_READ;
        }
        {
            all |= Self::ACCESS2_INPUT_ATTACHMENT_READ;
        }
        {
            all |= Self::ACCESS2_SHADER_READ;
        }
        {
            all |= Self::ACCESS2_SHADER_WRITE;
        }
        {
            all |= Self::ACCESS2_COLOR_ATTACHMENT_READ;
        }
        {
            all |= Self::ACCESS2_COLOR_ATTACHMENT_WRITE;
        }
        {
            all |= Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ;
        }
        {
            all |= Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE;
        }
        {
            all |= Self::ACCESS2_TRANSFER_READ;
        }
        {
            all |= Self::ACCESS2_TRANSFER_WRITE;
        }
        {
            all |= Self::ACCESS2_HOST_READ;
        }
        {
            all |= Self::ACCESS2_HOST_WRITE;
        }
        {
            all |= Self::ACCESS2_MEMORY_READ;
        }
        {
            all |= Self::ACCESS2_MEMORY_WRITE;
        }
        {
            all |= Self::ACCESS2_SHADER_SAMPLED_READ;
        }
        {
            all |= Self::ACCESS2_SHADER_STORAGE_READ;
        }
        {
            all |= Self::ACCESS2_SHADER_STORAGE_WRITE;
        }
        #[cfg(feature = "VK_KHR_video_decode_queue")]
        {
            all |= Self::ACCESS2_VIDEO_DECODE_READ_KHR;
        }
        #[cfg(feature = "VK_KHR_video_decode_queue")]
        {
            all |= Self::ACCESS2_VIDEO_DECODE_WRITE_KHR;
        }
        #[cfg(feature = "VK_KHR_video_encode_queue")]
        {
            all |= Self::ACCESS2_VIDEO_ENCODE_READ_KHR;
        }
        #[cfg(feature = "VK_KHR_video_encode_queue")]
        {
            all |= Self::ACCESS2_VIDEO_ENCODE_WRITE_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_TRANSFORM_FEEDBACK_WRITE_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_CONDITIONAL_RENDERING_READ_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_COMMAND_PREPROCESS_READ_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_COMMAND_PREPROCESS_WRITE_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_ACCELERATION_STRUCTURE_READ_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_FRAGMENT_DENSITY_MAP_READ_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT;
        }
        #[cfg(feature = "VK_HUAWEI_invocation_mask")]
        {
            all |= Self::ACCESS2_INVOCATION_MASK_READ_HUAWEI;
        }
        {
            all |= Self::ACCESS2_NONE_KHR;
        }
        {
            all |= Self::ACCESS2_INDIRECT_COMMAND_READ_KHR;
        }
        {
            all |= Self::ACCESS2_INDEX_READ_KHR;
        }
        {
            all |= Self::ACCESS2_VERTEX_ATTRIBUTE_READ_KHR;
        }
        {
            all |= Self::ACCESS2_UNIFORM_READ_KHR;
        }
        {
            all |= Self::ACCESS2_INPUT_ATTACHMENT_READ_KHR;
        }
        {
            all |= Self::ACCESS2_SHADER_READ_KHR;
        }
        {
            all |= Self::ACCESS2_SHADER_WRITE_KHR;
        }
        {
            all |= Self::ACCESS2_COLOR_ATTACHMENT_READ_KHR;
        }
        {
            all |= Self::ACCESS2_COLOR_ATTACHMENT_WRITE_KHR;
        }
        {
            all |= Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ_KHR;
        }
        {
            all |= Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE_KHR;
        }
        {
            all |= Self::ACCESS2_TRANSFER_READ_KHR;
        }
        {
            all |= Self::ACCESS2_TRANSFER_WRITE_KHR;
        }
        {
            all |= Self::ACCESS2_HOST_READ_KHR;
        }
        {
            all |= Self::ACCESS2_HOST_WRITE_KHR;
        }
        {
            all |= Self::ACCESS2_MEMORY_READ_KHR;
        }
        {
            all |= Self::ACCESS2_MEMORY_WRITE_KHR;
        }
        {
            all |= Self::ACCESS2_SHADER_SAMPLED_READ_KHR;
        }
        {
            all |= Self::ACCESS2_SHADER_STORAGE_READ_KHR;
        }
        {
            all |= Self::ACCESS2_SHADER_STORAGE_WRITE_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_SHADING_RATE_IMAGE_READ_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_ACCELERATION_STRUCTURE_READ_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::ACCESS2_ACCELERATION_STRUCTURE_WRITE_NV;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u64 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u64) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u64) -> Self {
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
impl std::ops::BitOr for AccessFlags2 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for AccessFlags2 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for AccessFlags2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for AccessFlags2 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for AccessFlags2 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for AccessFlags2 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for AccessFlags2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for AccessFlags2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for AccessFlags2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<AccessFlags2> for AccessFlags2 {
    fn extend<T: IntoIterator<Item = AccessFlags2>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<AccessFlags2> for AccessFlags2 {
    fn from_iter<T: IntoIterator<Item = AccessFlags2>>(iterator: T) -> AccessFlags2 {
        let mut out = Self::empty();
        <Self as Extend<AccessFlags2>>::extend(&mut out, iterator);
        out
    }
}
impl Default for AccessFlags2 {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<AccessFlagBits2> for AccessFlags2 {
    fn from(bit: AccessFlagBits2) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<AccessFlagBits2> for AccessFlags2 {
    fn extend<T: IntoIterator<Item = AccessFlagBits2>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<AccessFlagBits2> for AccessFlags2 {
    fn from_iter<T: IntoIterator<Item = AccessFlagBits2>>(iterator: T) -> AccessFlags2 {
        let mut out = Self::empty();
        <Self as Extend<AccessFlagBits2>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for AccessFlags2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(AccessFlags2);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == AccessFlags2::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(AccessFlags2::ACCESS2_NONE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_NONE))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_INDIRECT_COMMAND_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_INDIRECT_COMMAND_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_INDEX_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_INDEX_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_VERTEX_ATTRIBUTE_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_VERTEX_ATTRIBUTE_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_UNIFORM_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_UNIFORM_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_INPUT_ATTACHMENT_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_INPUT_ATTACHMENT_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_WRITE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_WRITE))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_COLOR_ATTACHMENT_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_COLOR_ATTACHMENT_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_COLOR_ATTACHMENT_WRITE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_COLOR_ATTACHMENT_WRITE))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_TRANSFER_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_TRANSFER_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_TRANSFER_WRITE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_TRANSFER_WRITE))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_HOST_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_HOST_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_HOST_WRITE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_HOST_WRITE))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_MEMORY_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_MEMORY_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_MEMORY_WRITE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_MEMORY_WRITE))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_SAMPLED_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_SAMPLED_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_STORAGE_READ) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_STORAGE_READ))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_STORAGE_WRITE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_STORAGE_WRITE))?;
                    }
                    #[cfg(feature = "VK_KHR_video_decode_queue")]
                    if self.0.contains(AccessFlags2::ACCESS2_VIDEO_DECODE_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_VIDEO_DECODE_READ_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_video_decode_queue")]
                    if self.0.contains(AccessFlags2::ACCESS2_VIDEO_DECODE_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_VIDEO_DECODE_WRITE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_video_encode_queue")]
                    if self.0.contains(AccessFlags2::ACCESS2_VIDEO_ENCODE_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_VIDEO_ENCODE_READ_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_video_encode_queue")]
                    if self.0.contains(AccessFlags2::ACCESS2_VIDEO_ENCODE_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_VIDEO_ENCODE_WRITE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_TRANSFORM_FEEDBACK_WRITE_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_TRANSFORM_FEEDBACK_WRITE_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(AccessFlags2::ACCESS2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(AccessFlags2::ACCESS2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_CONDITIONAL_RENDERING_READ_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_CONDITIONAL_RENDERING_READ_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_COMMAND_PREPROCESS_READ_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_COMMAND_PREPROCESS_READ_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_COMMAND_PREPROCESS_WRITE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_COMMAND_PREPROCESS_WRITE_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(AccessFlags2::ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_ACCELERATION_STRUCTURE_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_ACCELERATION_STRUCTURE_READ_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_FRAGMENT_DENSITY_MAP_READ_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_FRAGMENT_DENSITY_MAP_READ_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(AccessFlags2::ACCESS2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT))?;
                    }
                    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
                    if self.0.contains(AccessFlags2::ACCESS2_INVOCATION_MASK_READ_HUAWEI) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_INVOCATION_MASK_READ_HUAWEI))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_NONE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_NONE_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_INDIRECT_COMMAND_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_INDIRECT_COMMAND_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_INDEX_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_INDEX_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_VERTEX_ATTRIBUTE_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_VERTEX_ATTRIBUTE_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_UNIFORM_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_UNIFORM_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_INPUT_ATTACHMENT_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_INPUT_ATTACHMENT_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_WRITE_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_COLOR_ATTACHMENT_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_COLOR_ATTACHMENT_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_COLOR_ATTACHMENT_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_COLOR_ATTACHMENT_WRITE_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ_KHR))?;
                    }
                    if self
                        .0
                        .contains(AccessFlags2::ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_TRANSFER_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_TRANSFER_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_TRANSFER_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_TRANSFER_WRITE_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_HOST_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_HOST_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_HOST_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_HOST_WRITE_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_MEMORY_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_MEMORY_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_MEMORY_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_MEMORY_WRITE_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_SAMPLED_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_SAMPLED_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_STORAGE_READ_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_STORAGE_READ_KHR))?;
                    }
                    if self.0.contains(AccessFlags2::ACCESS2_SHADER_STORAGE_WRITE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADER_STORAGE_WRITE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_SHADING_RATE_IMAGE_READ_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_SHADING_RATE_IMAGE_READ_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_ACCELERATION_STRUCTURE_READ_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_ACCELERATION_STRUCTURE_READ_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(AccessFlags2::ACCESS2_ACCELERATION_STRUCTURE_WRITE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ACCESS2_ACCELERATION_STRUCTURE_WRITE_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(AccessFlags2)).field(&Flags(*self)).finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccessFlags2 {
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
unsafe impl crate::conv::FromLowLevel for AccessFlags2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPipelineStageFlags2")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineStageFlags2(u64);
impl PipelineStageFlags2 {
    #[doc(alias = "VK_PIPELINE_STAGE_2_NONE")]
    pub const PIPELINE_STAGE2_NONE: Self = Self(0);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT")]
    pub const PIPELINE_STAGE2_TOP_OF_PIPE: Self = Self(1);
    #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT")]
    pub const PIPELINE_STAGE2_DRAW_INDIRECT: Self = Self(2);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT")]
    pub const PIPELINE_STAGE2_VERTEX_INPUT: Self = Self(4);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT")]
    pub const PIPELINE_STAGE2_VERTEX_SHADER: Self = Self(8);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT")]
    pub const PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER: Self = Self(16);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT")]
    pub const PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER: Self = Self(32);
    #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT")]
    pub const PIPELINE_STAGE2_GEOMETRY_SHADER: Self = Self(64);
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT")]
    pub const PIPELINE_STAGE2_FRAGMENT_SHADER: Self = Self(128);
    #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT")]
    pub const PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS: Self = Self(256);
    #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT")]
    pub const PIPELINE_STAGE2_LATE_FRAGMENT_TESTS: Self = Self(512);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT")]
    pub const PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT: Self = Self(1024);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT")]
    pub const PIPELINE_STAGE2_COMPUTE_SHADER: Self = Self(2048);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT")]
    pub const PIPELINE_STAGE2_ALL_TRANSFER: Self = Self(4096);
    #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT")]
    pub const PIPELINE_STAGE2_BOTTOM_OF_PIPE: Self = Self(8192);
    #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT")]
    pub const PIPELINE_STAGE2_HOST: Self = Self(16384);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT")]
    pub const PIPELINE_STAGE2_ALL_GRAPHICS: Self = Self(32768);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT")]
    pub const PIPELINE_STAGE2_ALL_COMMANDS: Self = Self(65536);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT")]
    pub const PIPELINE_STAGE2_COPY: Self = Self(4294967296);
    #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT")]
    pub const PIPELINE_STAGE2_RESOLVE: Self = Self(8589934592);
    #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT")]
    pub const PIPELINE_STAGE2_BLIT: Self = Self(17179869184);
    #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT")]
    pub const PIPELINE_STAGE2_CLEAR: Self = Self(34359738368);
    #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT")]
    pub const PIPELINE_STAGE2_INDEX_INPUT: Self = Self(68719476736);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT")]
    pub const PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT: Self = Self(137438953472);
    #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT")]
    pub const PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS: Self = Self(274877906944);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const PIPELINE_STAGE2_VIDEO_DECODE_KHR: Self = Self(67108864);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const PIPELINE_STAGE2_VIDEO_ENCODE_KHR: Self = Self(134217728);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT: Self = Self(16777216);
    #[doc(alias = "VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT: Self = Self(262144);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_COMMAND_PREPROCESS_NV: Self = Self(131072);
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432);
    #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR: Self = Self(2097152);
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_TASK_SHADER_NV: Self = Self(524288);
    #[doc(alias = "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_MESH_SHADER_NV: Self = Self(1048576);
    #[doc(alias = "VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI")]
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    pub const PIPELINE_STAGE2_SUBPASS_SHADING_HUAWEI: Self = Self(549755813888);
    #[doc(alias = "VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    pub const PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI: Self = Self(1099511627776);
    #[doc(alias = "VK_PIPELINE_STAGE_2_NONE_KHR")]
    pub const PIPELINE_STAGE2_NONE_KHR: Self = Self::PIPELINE_STAGE2_NONE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR")]
    pub const PIPELINE_STAGE2_TOP_OF_PIPE_KHR: Self = Self::PIPELINE_STAGE2_TOP_OF_PIPE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR")]
    pub const PIPELINE_STAGE2_DRAW_INDIRECT_KHR: Self = Self::PIPELINE_STAGE2_DRAW_INDIRECT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_VERTEX_INPUT_KHR: Self = Self::PIPELINE_STAGE2_VERTEX_INPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_VERTEX_SHADER_KHR: Self = Self::PIPELINE_STAGE2_VERTEX_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER_KHR: Self = Self::PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER_KHR: Self =
        Self::PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_GEOMETRY_SHADER_KHR: Self = Self::PIPELINE_STAGE2_GEOMETRY_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_FRAGMENT_SHADER_KHR: Self = Self::PIPELINE_STAGE2_FRAGMENT_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR")]
    pub const PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS_KHR: Self = Self::PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR")]
    pub const PIPELINE_STAGE2_LATE_FRAGMENT_TESTS_KHR: Self = Self::PIPELINE_STAGE2_LATE_FRAGMENT_TESTS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_COMPUTE_SHADER_KHR: Self = Self::PIPELINE_STAGE2_COMPUTE_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR")]
    pub const PIPELINE_STAGE2_ALL_TRANSFER_KHR: Self = Self::PIPELINE_STAGE2_ALL_TRANSFER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT")]
    pub const PIPELINE_STAGE2_TRANSFER: Self = Self::PIPELINE_STAGE2_ALL_TRANSFER_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR")]
    pub const PIPELINE_STAGE2_TRANSFER_KHR: Self = Self::PIPELINE_STAGE2_TRANSFER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR")]
    pub const PIPELINE_STAGE2_BOTTOM_OF_PIPE_KHR: Self = Self::PIPELINE_STAGE2_BOTTOM_OF_PIPE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT_KHR")]
    pub const PIPELINE_STAGE2_HOST_KHR: Self = Self::PIPELINE_STAGE2_HOST;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR")]
    pub const PIPELINE_STAGE2_ALL_GRAPHICS_KHR: Self = Self::PIPELINE_STAGE2_ALL_GRAPHICS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR")]
    pub const PIPELINE_STAGE2_ALL_COMMANDS_KHR: Self = Self::PIPELINE_STAGE2_ALL_COMMANDS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT_KHR")]
    pub const PIPELINE_STAGE2_COPY_KHR: Self = Self::PIPELINE_STAGE2_COPY;
    #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR")]
    pub const PIPELINE_STAGE2_RESOLVE_KHR: Self = Self::PIPELINE_STAGE2_RESOLVE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT_KHR")]
    pub const PIPELINE_STAGE2_BLIT_KHR: Self = Self::PIPELINE_STAGE2_BLIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR")]
    pub const PIPELINE_STAGE2_CLEAR_KHR: Self = Self::PIPELINE_STAGE2_CLEAR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_INDEX_INPUT_KHR: Self = Self::PIPELINE_STAGE2_INDEX_INPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR")]
    pub const PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_SHADING_RATE_IMAGE_NV: Self = Self::PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_RAY_TRACING_SHADER_NV: Self = Self::PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_NV: Self =
        Self::PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR;
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
            all |= Self::PIPELINE_STAGE2_NONE;
        }
        {
            all |= Self::PIPELINE_STAGE2_TOP_OF_PIPE;
        }
        {
            all |= Self::PIPELINE_STAGE2_DRAW_INDIRECT;
        }
        {
            all |= Self::PIPELINE_STAGE2_VERTEX_INPUT;
        }
        {
            all |= Self::PIPELINE_STAGE2_VERTEX_SHADER;
        }
        {
            all |= Self::PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER;
        }
        {
            all |= Self::PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER;
        }
        {
            all |= Self::PIPELINE_STAGE2_GEOMETRY_SHADER;
        }
        {
            all |= Self::PIPELINE_STAGE2_FRAGMENT_SHADER;
        }
        {
            all |= Self::PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS;
        }
        {
            all |= Self::PIPELINE_STAGE2_LATE_FRAGMENT_TESTS;
        }
        {
            all |= Self::PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT;
        }
        {
            all |= Self::PIPELINE_STAGE2_COMPUTE_SHADER;
        }
        {
            all |= Self::PIPELINE_STAGE2_ALL_TRANSFER;
        }
        {
            all |= Self::PIPELINE_STAGE2_BOTTOM_OF_PIPE;
        }
        {
            all |= Self::PIPELINE_STAGE2_HOST;
        }
        {
            all |= Self::PIPELINE_STAGE2_ALL_GRAPHICS;
        }
        {
            all |= Self::PIPELINE_STAGE2_ALL_COMMANDS;
        }
        {
            all |= Self::PIPELINE_STAGE2_COPY;
        }
        {
            all |= Self::PIPELINE_STAGE2_RESOLVE;
        }
        {
            all |= Self::PIPELINE_STAGE2_BLIT;
        }
        {
            all |= Self::PIPELINE_STAGE2_CLEAR;
        }
        {
            all |= Self::PIPELINE_STAGE2_INDEX_INPUT;
        }
        {
            all |= Self::PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT;
        }
        {
            all |= Self::PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS;
        }
        #[cfg(feature = "VK_KHR_video_decode_queue")]
        {
            all |= Self::PIPELINE_STAGE2_VIDEO_DECODE_KHR;
        }
        #[cfg(feature = "VK_KHR_video_encode_queue")]
        {
            all |= Self::PIPELINE_STAGE2_VIDEO_ENCODE_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_COMMAND_PREPROCESS_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_TASK_SHADER_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_MESH_SHADER_NV;
        }
        #[cfg(feature = "VK_HUAWEI_subpass_shading")]
        {
            all |= Self::PIPELINE_STAGE2_SUBPASS_SHADING_HUAWEI;
        }
        #[cfg(feature = "VK_HUAWEI_invocation_mask")]
        {
            all |= Self::PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI;
        }
        {
            all |= Self::PIPELINE_STAGE2_NONE_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_TOP_OF_PIPE_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_DRAW_INDIRECT_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_VERTEX_INPUT_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_VERTEX_SHADER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_GEOMETRY_SHADER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_FRAGMENT_SHADER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_LATE_FRAGMENT_TESTS_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_COMPUTE_SHADER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_ALL_TRANSFER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_TRANSFER;
        }
        {
            all |= Self::PIPELINE_STAGE2_TRANSFER_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_BOTTOM_OF_PIPE_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_HOST_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_ALL_GRAPHICS_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_ALL_COMMANDS_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_COPY_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_RESOLVE_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_BLIT_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_CLEAR_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_INDEX_INPUT_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT_KHR;
        }
        {
            all |= Self::PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS_KHR;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_SHADING_RATE_IMAGE_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_RAY_TRACING_SHADER_NV;
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            all |= Self::PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_NV;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u64 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u64) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u64) -> Self {
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
impl std::ops::BitOr for PipelineStageFlags2 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for PipelineStageFlags2 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for PipelineStageFlags2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for PipelineStageFlags2 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for PipelineStageFlags2 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for PipelineStageFlags2 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for PipelineStageFlags2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for PipelineStageFlags2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for PipelineStageFlags2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PipelineStageFlags2> for PipelineStageFlags2 {
    fn extend<T: IntoIterator<Item = PipelineStageFlags2>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<PipelineStageFlags2> for PipelineStageFlags2 {
    fn from_iter<T: IntoIterator<Item = PipelineStageFlags2>>(iterator: T) -> PipelineStageFlags2 {
        let mut out = Self::empty();
        <Self as Extend<PipelineStageFlags2>>::extend(&mut out, iterator);
        out
    }
}
impl Default for PipelineStageFlags2 {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<PipelineStageFlagBits2> for PipelineStageFlags2 {
    fn from(bit: PipelineStageFlagBits2) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<PipelineStageFlagBits2> for PipelineStageFlags2 {
    fn extend<T: IntoIterator<Item = PipelineStageFlagBits2>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<PipelineStageFlagBits2> for PipelineStageFlags2 {
    fn from_iter<T: IntoIterator<Item = PipelineStageFlagBits2>>(iterator: T) -> PipelineStageFlags2 {
        let mut out = Self::empty();
        <Self as Extend<PipelineStageFlagBits2>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PipelineStageFlags2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PipelineStageFlags2);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PipelineStageFlags2::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_NONE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_NONE))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_TOP_OF_PIPE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TOP_OF_PIPE))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_DRAW_INDIRECT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_DRAW_INDIRECT))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_VERTEX_INPUT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VERTEX_INPUT))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_VERTEX_SHADER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VERTEX_SHADER))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_GEOMETRY_SHADER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_GEOMETRY_SHADER))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_FRAGMENT_SHADER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_FRAGMENT_SHADER))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_LATE_FRAGMENT_TESTS)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_LATE_FRAGMENT_TESTS))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_COMPUTE_SHADER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_COMPUTE_SHADER))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_ALL_TRANSFER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ALL_TRANSFER))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_BOTTOM_OF_PIPE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_BOTTOM_OF_PIPE))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_HOST) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_HOST))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_ALL_GRAPHICS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ALL_GRAPHICS))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_ALL_COMMANDS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ALL_COMMANDS))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_COPY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_COPY))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_RESOLVE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_RESOLVE))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_BLIT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_BLIT))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_CLEAR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_CLEAR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_INDEX_INPUT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_INDEX_INPUT))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS))?;
                    }
                    #[cfg(feature = "VK_KHR_video_decode_queue")]
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_VIDEO_DECODE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VIDEO_DECODE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_video_encode_queue")]
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_VIDEO_ENCODE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VIDEO_ENCODE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_COMMAND_PREPROCESS_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_COMMAND_PREPROCESS_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_TASK_SHADER_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TASK_SHADER_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_MESH_SHADER_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_MESH_SHADER_NV))?;
                    }
                    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_SUBPASS_SHADING_HUAWEI)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_SUBPASS_SHADING_HUAWEI))?;
                    }
                    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_NONE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_NONE_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_TOP_OF_PIPE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TOP_OF_PIPE_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_DRAW_INDIRECT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_DRAW_INDIRECT_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_VERTEX_INPUT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VERTEX_INPUT_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_VERTEX_SHADER_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VERTEX_SHADER_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_GEOMETRY_SHADER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_GEOMETRY_SHADER_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_FRAGMENT_SHADER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_FRAGMENT_SHADER_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_LATE_FRAGMENT_TESTS_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_LATE_FRAGMENT_TESTS_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_COMPUTE_SHADER_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_COMPUTE_SHADER_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_ALL_TRANSFER_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ALL_TRANSFER_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_TRANSFER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TRANSFER))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_TRANSFER_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_TRANSFER_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_BOTTOM_OF_PIPE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_BOTTOM_OF_PIPE_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_HOST_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_HOST_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_ALL_GRAPHICS_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ALL_GRAPHICS_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_ALL_COMMANDS_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ALL_COMMANDS_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_COPY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_COPY_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_RESOLVE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_RESOLVE_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_BLIT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_BLIT_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_CLEAR_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_CLEAR_KHR))?;
                    }
                    if self.0.contains(PipelineStageFlags2::PIPELINE_STAGE2_INDEX_INPUT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_INDEX_INPUT_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT_KHR))?;
                    }
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_SHADING_RATE_IMAGE_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_SHADING_RATE_IMAGE_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_RAY_TRACING_SHADER_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_RAY_TRACING_SHADER_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_synchronization2")]
                    if self
                        .0
                        .contains(PipelineStageFlags2::PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PipelineStageFlags2))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineStageFlags2 {
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
unsafe impl crate::conv::FromLowLevel for PipelineStageFlags2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkFormatFeatureFlags2")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FormatFeatureFlags2(u64);
impl FormatFeatureFlags2 {
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE: Self = Self(1);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE: Self = Self(2);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC: Self = Self(4);
    #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT")]
    pub const FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER: Self = Self(8);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER: Self = Self(16);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT")]
    pub const FORMAT_FEATURE2_VERTEX_BUFFER: Self = Self(64);
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT: Self = Self(128);
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND: Self = Self(256);
    #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT")]
    pub const FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT: Self = Self(512);
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT")]
    pub const FORMAT_FEATURE2_BLIT_SRC: Self = Self(1024);
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT")]
    pub const FORMAT_FEATURE2_BLIT_DST: Self = Self(2048);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(8192);
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT")]
    pub const FORMAT_FEATURE2_TRANSFER_SRC: Self = Self(16384);
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT")]
    pub const FORMAT_FEATURE2_TRANSFER_DST: Self = Self(32768);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(65536);
    #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT")]
    pub const FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES: Self = Self(131072);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(262144);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(524288);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(1048576);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(2097152);
    #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT")]
    pub const FORMAT_FEATURE2_DISJOINT: Self = Self(4194304);
    #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT")]
    pub const FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES: Self = Self(8388608);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT: Self = Self(2147483648);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(4294967296);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(8589934592);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_DECODE_DPB_KHR: Self = Self(67108864);
    #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub const FORMAT_FEATURE2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(536870912);
    #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    pub const FORMAT_FEATURE2_FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216);
    #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    pub const FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1073741824);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_ENCODE_DPB_KHR: Self = Self(268435456);
    #[doc(alias = "VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV")]
    #[cfg(feature = "VK_NV_linear_color_attachment")]
    pub const FORMAT_FEATURE2_LINEAR_COLOR_ATTACHMENT_NV: Self = Self(274877906944);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_KHR: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_IMAGE;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR")]
    pub const FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER_KHR: Self = Self::FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR")]
    pub const FORMAT_FEATURE2_VERTEX_BUFFER_KHR: Self = Self::FORMAT_FEATURE2_VERTEX_BUFFER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT_KHR: Self = Self::FORMAT_FEATURE2_COLOR_ATTACHMENT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND_KHR: Self = Self::FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND;
    #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR")]
    pub const FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR")]
    pub const FORMAT_FEATURE2_BLIT_SRC_KHR: Self = Self::FORMAT_FEATURE2_BLIT_SRC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR")]
    pub const FORMAT_FEATURE2_BLIT_DST_KHR: Self = Self::FORMAT_FEATURE2_BLIT_DST;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR")]
    pub const FORMAT_FEATURE2_TRANSFER_SRC_KHR: Self = Self::FORMAT_FEATURE2_TRANSFER_SRC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR")]
    pub const FORMAT_FEATURE2_TRANSFER_DST_KHR: Self = Self::FORMAT_FEATURE2_TRANSFER_DST;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX;
    #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR")]
    pub const FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR"
    )]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR")]
    pub const FORMAT_FEATURE2_DISJOINT_KHR: Self = Self::FORMAT_FEATURE2_DISJOINT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR")]
    pub const FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES_KHR: Self = Self::FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self =
        Self::FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON;
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
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_IMAGE;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC;
        }
        {
            all |= Self::FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC;
        }
        {
            all |= Self::FORMAT_FEATURE2_VERTEX_BUFFER;
        }
        {
            all |= Self::FORMAT_FEATURE2_COLOR_ATTACHMENT;
        }
        {
            all |= Self::FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND;
        }
        {
            all |= Self::FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT;
        }
        {
            all |= Self::FORMAT_FEATURE2_BLIT_SRC;
        }
        {
            all |= Self::FORMAT_FEATURE2_BLIT_DST;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC;
        }
        {
            all |= Self::FORMAT_FEATURE2_TRANSFER_SRC;
        }
        {
            all |= Self::FORMAT_FEATURE2_TRANSFER_DST;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX;
        }
        {
            all |= Self::FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
        }
        {
            all |= Self::FORMAT_FEATURE2_DISJOINT;
        }
        {
            all |= Self::FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON;
        }
        #[cfg(feature = "VK_KHR_video_decode_queue")]
        {
            all |= Self::FORMAT_FEATURE2_VIDEO_DECODE_OUTPUT_KHR;
        }
        #[cfg(feature = "VK_KHR_video_decode_queue")]
        {
            all |= Self::FORMAT_FEATURE2_VIDEO_DECODE_DPB_KHR;
        }
        #[cfg(feature = "VK_KHR_acceleration_structure")]
        {
            all |= Self::FORMAT_FEATURE2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR;
        }
        #[cfg(feature = "VK_EXT_fragment_density_map")]
        {
            all |= Self::FORMAT_FEATURE2_FRAGMENT_DENSITY_MAP_EXT;
        }
        #[cfg(feature = "VK_KHR_fragment_shading_rate")]
        {
            all |= Self::FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
        }
        #[cfg(feature = "VK_KHR_video_encode_queue")]
        {
            all |= Self::FORMAT_FEATURE2_VIDEO_ENCODE_INPUT_KHR;
        }
        #[cfg(feature = "VK_KHR_video_encode_queue")]
        {
            all |= Self::FORMAT_FEATURE2_VIDEO_ENCODE_DPB_KHR;
        }
        #[cfg(feature = "VK_NV_linear_color_attachment")]
        {
            all |= Self::FORMAT_FEATURE2_LINEAR_COLOR_ATTACHMENT_NV;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_IMAGE_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_VERTEX_BUFFER_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_COLOR_ATTACHMENT_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_BLIT_SRC_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_BLIT_DST_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC_EXT;
        }
        {
            all |= Self::FORMAT_FEATURE2_TRANSFER_SRC_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_TRANSFER_DST_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_DISJOINT_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT_KHR;
        }
        {
            all |= Self::FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON_KHR;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u64 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u64) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u64) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u64) -> Self {
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
impl std::ops::BitOr for FormatFeatureFlags2 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for FormatFeatureFlags2 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for FormatFeatureFlags2 {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for FormatFeatureFlags2 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for FormatFeatureFlags2 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for FormatFeatureFlags2 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for FormatFeatureFlags2 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for FormatFeatureFlags2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for FormatFeatureFlags2 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<FormatFeatureFlags2> for FormatFeatureFlags2 {
    fn extend<T: IntoIterator<Item = FormatFeatureFlags2>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<FormatFeatureFlags2> for FormatFeatureFlags2 {
    fn from_iter<T: IntoIterator<Item = FormatFeatureFlags2>>(iterator: T) -> FormatFeatureFlags2 {
        let mut out = Self::empty();
        <Self as Extend<FormatFeatureFlags2>>::extend(&mut out, iterator);
        out
    }
}
impl Default for FormatFeatureFlags2 {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<FormatFeatureFlagBits2> for FormatFeatureFlags2 {
    fn from(bit: FormatFeatureFlagBits2) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<FormatFeatureFlagBits2> for FormatFeatureFlags2 {
    fn extend<T: IntoIterator<Item = FormatFeatureFlagBits2>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<FormatFeatureFlagBits2> for FormatFeatureFlags2 {
    fn from_iter<T: IntoIterator<Item = FormatFeatureFlagBits2>>(iterator: T) -> FormatFeatureFlags2 {
        let mut out = Self::empty();
        <Self as Extend<FormatFeatureFlagBits2>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for FormatFeatureFlags2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(FormatFeatureFlags2);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == FormatFeatureFlags2::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_IMAGE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_IMAGE))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_VERTEX_BUFFER) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_VERTEX_BUFFER))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_COLOR_ATTACHMENT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_COLOR_ATTACHMENT))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_BLIT_SRC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_BLIT_SRC))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_BLIT_DST) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_BLIT_DST))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_TRANSFER_SRC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_TRANSFER_SRC))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_TRANSFER_DST) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_TRANSFER_DST))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER))?;
                    }
                    if self . 0 . contains (FormatFeatureFlags2 :: FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER) { if ! first { f . write_str (" | ") ? ; } first = false ; f . write_str (stringify ! (FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER)) ? ; }
                    if self . 0 . contains (FormatFeatureFlags2 :: FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT) { if ! first { f . write_str (" | ") ? ; } first = false ; f . write_str (stringify ! (FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT)) ? ; }
                    if self . 0 . contains (FormatFeatureFlags2 :: FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE) { if ! first { f . write_str (" | ") ? ; } first = false ; f . write_str (stringify ! (FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE)) ? ; }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_DISJOINT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_DISJOINT))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON))?;
                    }
                    #[cfg(feature = "VK_KHR_video_decode_queue")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_VIDEO_DECODE_OUTPUT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_VIDEO_DECODE_OUTPUT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_video_decode_queue")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_VIDEO_DECODE_DPB_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_VIDEO_DECODE_DPB_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_acceleration_structure")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR))?;
                    }
                    #[cfg(feature = "VK_EXT_fragment_density_map")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_FRAGMENT_DENSITY_MAP_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_FRAGMENT_DENSITY_MAP_EXT))?;
                    }
                    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_video_encode_queue")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_VIDEO_ENCODE_INPUT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_VIDEO_ENCODE_INPUT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_video_encode_queue")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_VIDEO_ENCODE_DPB_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_VIDEO_ENCODE_DPB_KHR))?;
                    }
                    #[cfg(feature = "VK_NV_linear_color_attachment")]
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_LINEAR_COLOR_ATTACHMENT_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_LINEAR_COLOR_ATTACHMENT_NV))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_KHR))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_IMAGE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_IMAGE_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC_KHR))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_VERTEX_BUFFER_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_VERTEX_BUFFER_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_COLOR_ATTACHMENT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_COLOR_ATTACHMENT_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT_KHR))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_BLIT_SRC_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_BLIT_SRC_KHR))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_BLIT_DST_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_BLIT_DST_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC_EXT))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_TRANSFER_SRC_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_TRANSFER_SRC_KHR))?;
                    }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_TRANSFER_DST_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_TRANSFER_DST_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(
                            FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR
                        ))?;
                    }
                    if self . 0 . contains (FormatFeatureFlags2 :: FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR) { if ! first { f . write_str (" | ") ? ; } first = false ; f . write_str (stringify ! (FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR)) ? ; }
                    if self . 0 . contains (FormatFeatureFlags2 :: FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR) { if ! first { f . write_str (" | ") ? ; } first = false ; f . write_str (stringify ! (FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR)) ? ; }
                    if self . 0 . contains (FormatFeatureFlags2 :: FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR) { if ! first { f . write_str (" | ") ? ; } first = false ; f . write_str (stringify ! (FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR)) ? ; }
                    if self.0.contains(FormatFeatureFlags2::FORMAT_FEATURE2_DISJOINT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_DISJOINT_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT_KHR))?;
                    }
                    if self
                        .0
                        .contains(FormatFeatureFlags2::FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON_KHR)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(FormatFeatureFlags2))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FormatFeatureFlags2 {
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
unsafe impl crate::conv::FromLowLevel for FormatFeatureFlags2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkRenderingFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderingFlags(u32);
impl RenderingFlags {
    #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT")]
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1);
    #[doc(alias = "VK_RENDERING_SUSPENDING_BIT")]
    pub const SUSPENDING: Self = Self(2);
    #[doc(alias = "VK_RENDERING_RESUMING_BIT")]
    pub const RESUMING: Self = Self(4);
    #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR")]
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
    #[doc(alias = "VK_RENDERING_SUSPENDING_BIT_KHR")]
    pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
    #[doc(alias = "VK_RENDERING_RESUMING_BIT_KHR")]
    pub const RESUMING_KHR: Self = Self::RESUMING;
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
            all |= Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
        }
        {
            all |= Self::SUSPENDING;
        }
        {
            all |= Self::RESUMING;
        }
        {
            all |= Self::CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR;
        }
        {
            all |= Self::SUSPENDING_KHR;
        }
        {
            all |= Self::RESUMING_KHR;
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
impl std::ops::BitOr for RenderingFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for RenderingFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for RenderingFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for RenderingFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for RenderingFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for RenderingFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for RenderingFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for RenderingFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for RenderingFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<RenderingFlags> for RenderingFlags {
    fn extend<T: IntoIterator<Item = RenderingFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<RenderingFlags> for RenderingFlags {
    fn from_iter<T: IntoIterator<Item = RenderingFlags>>(iterator: T) -> RenderingFlags {
        let mut out = Self::empty();
        <Self as Extend<RenderingFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for RenderingFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<RenderingFlagBits> for RenderingFlags {
    fn from(bit: RenderingFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<RenderingFlagBits> for RenderingFlags {
    fn extend<T: IntoIterator<Item = RenderingFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<RenderingFlagBits> for RenderingFlags {
    fn from_iter<T: IntoIterator<Item = RenderingFlagBits>>(iterator: T) -> RenderingFlags {
        let mut out = Self::empty();
        <Self as Extend<RenderingFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for RenderingFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(RenderingFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == RenderingFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(RenderingFlags::CONTENTS_SECONDARY_COMMAND_BUFFERS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CONTENTS_SECONDARY_COMMAND_BUFFERS))?;
                    }
                    if self.0.contains(RenderingFlags::SUSPENDING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SUSPENDING))?;
                    }
                    if self.0.contains(RenderingFlags::RESUMING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RESUMING))?;
                    }
                    if self.0.contains(RenderingFlags::CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR))?;
                    }
                    if self.0.contains(RenderingFlags::SUSPENDING_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SUSPENDING_KHR))?;
                    }
                    if self.0.contains(RenderingFlags::RESUMING_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RESUMING_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(RenderingFlags)).field(&Flags(*self)).finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for RenderingFlags {
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
unsafe impl crate::conv::FromLowLevel for RenderingFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkToolPurposeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ToolPurposeFlags(u32);
impl ToolPurposeFlags {
    #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT")]
    pub const VALIDATION: Self = Self(1);
    #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT")]
    pub const PROFILING: Self = Self(2);
    #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT")]
    pub const TRACING: Self = Self(4);
    #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT")]
    pub const ADDITIONAL_FEATURES: Self = Self(8);
    #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT")]
    pub const MODIFYING_FEATURES: Self = Self(16);
    #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT")]
    #[cfg(feature = "VK_EXT_tooling_info")]
    pub const DEBUG_REPORTING_EXT: Self = Self(32);
    #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT")]
    #[cfg(feature = "VK_EXT_tooling_info")]
    pub const DEBUG_MARKERS_EXT: Self = Self(64);
    #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT_EXT")]
    pub const VALIDATION_EXT: Self = Self::VALIDATION;
    #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT_EXT")]
    pub const PROFILING_EXT: Self = Self::PROFILING;
    #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT_EXT")]
    pub const TRACING_EXT: Self = Self::TRACING;
    #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT")]
    pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
    #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT")]
    pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
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
            all |= Self::VALIDATION;
        }
        {
            all |= Self::PROFILING;
        }
        {
            all |= Self::TRACING;
        }
        {
            all |= Self::ADDITIONAL_FEATURES;
        }
        {
            all |= Self::MODIFYING_FEATURES;
        }
        #[cfg(feature = "VK_EXT_tooling_info")]
        {
            all |= Self::DEBUG_REPORTING_EXT;
        }
        #[cfg(feature = "VK_EXT_tooling_info")]
        {
            all |= Self::DEBUG_MARKERS_EXT;
        }
        {
            all |= Self::VALIDATION_EXT;
        }
        {
            all |= Self::PROFILING_EXT;
        }
        {
            all |= Self::TRACING_EXT;
        }
        {
            all |= Self::ADDITIONAL_FEATURES_EXT;
        }
        {
            all |= Self::MODIFYING_FEATURES_EXT;
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
impl std::ops::BitOr for ToolPurposeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ToolPurposeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ToolPurposeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ToolPurposeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ToolPurposeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ToolPurposeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ToolPurposeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ToolPurposeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ToolPurposeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ToolPurposeFlags> for ToolPurposeFlags {
    fn extend<T: IntoIterator<Item = ToolPurposeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ToolPurposeFlags> for ToolPurposeFlags {
    fn from_iter<T: IntoIterator<Item = ToolPurposeFlags>>(iterator: T) -> ToolPurposeFlags {
        let mut out = Self::empty();
        <Self as Extend<ToolPurposeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ToolPurposeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ToolPurposeFlagBits> for ToolPurposeFlags {
    fn from(bit: ToolPurposeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ToolPurposeFlagBits> for ToolPurposeFlags {
    fn extend<T: IntoIterator<Item = ToolPurposeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ToolPurposeFlagBits> for ToolPurposeFlags {
    fn from_iter<T: IntoIterator<Item = ToolPurposeFlagBits>>(iterator: T) -> ToolPurposeFlags {
        let mut out = Self::empty();
        <Self as Extend<ToolPurposeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ToolPurposeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ToolPurposeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ToolPurposeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ToolPurposeFlags::VALIDATION) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VALIDATION))?;
                    }
                    if self.0.contains(ToolPurposeFlags::PROFILING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROFILING))?;
                    }
                    if self.0.contains(ToolPurposeFlags::TRACING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRACING))?;
                    }
                    if self.0.contains(ToolPurposeFlags::ADDITIONAL_FEATURES) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ADDITIONAL_FEATURES))?;
                    }
                    if self.0.contains(ToolPurposeFlags::MODIFYING_FEATURES) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MODIFYING_FEATURES))?;
                    }
                    #[cfg(feature = "VK_EXT_tooling_info")]
                    if self.0.contains(ToolPurposeFlags::DEBUG_REPORTING_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEBUG_REPORTING_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_tooling_info")]
                    if self.0.contains(ToolPurposeFlags::DEBUG_MARKERS_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEBUG_MARKERS_EXT))?;
                    }
                    if self.0.contains(ToolPurposeFlags::VALIDATION_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VALIDATION_EXT))?;
                    }
                    if self.0.contains(ToolPurposeFlags::PROFILING_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROFILING_EXT))?;
                    }
                    if self.0.contains(ToolPurposeFlags::TRACING_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRACING_EXT))?;
                    }
                    if self.0.contains(ToolPurposeFlags::ADDITIONAL_FEATURES_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ADDITIONAL_FEATURES_EXT))?;
                    }
                    if self.0.contains(ToolPurposeFlags::MODIFYING_FEATURES_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MODIFYING_FEATURES_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ToolPurposeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ToolPurposeFlags {
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
unsafe impl crate::conv::FromLowLevel for ToolPurposeFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSubmitFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitFlags(u32);
impl SubmitFlags {
    #[doc(alias = "VK_SUBMIT_PROTECTED_BIT")]
    pub const PROTECTED: Self = Self(1);
    #[doc(alias = "VK_SUBMIT_PROTECTED_BIT_KHR")]
    pub const PROTECTED_KHR: Self = Self::PROTECTED;
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
            all |= Self::PROTECTED;
        }
        {
            all |= Self::PROTECTED_KHR;
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
impl std::ops::BitOr for SubmitFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SubmitFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for SubmitFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SubmitFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for SubmitFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SubmitFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for SubmitFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SubmitFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for SubmitFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SubmitFlags> for SubmitFlags {
    fn extend<T: IntoIterator<Item = SubmitFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SubmitFlags> for SubmitFlags {
    fn from_iter<T: IntoIterator<Item = SubmitFlags>>(iterator: T) -> SubmitFlags {
        let mut out = Self::empty();
        <Self as Extend<SubmitFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for SubmitFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<SubmitFlagBits> for SubmitFlags {
    fn from(bit: SubmitFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SubmitFlagBits> for SubmitFlags {
    fn extend<T: IntoIterator<Item = SubmitFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SubmitFlagBits> for SubmitFlags {
    fn from_iter<T: IntoIterator<Item = SubmitFlagBits>>(iterator: T) -> SubmitFlags {
        let mut out = Self::empty();
        <Self as Extend<SubmitFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SubmitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SubmitFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SubmitFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SubmitFlags::PROTECTED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED))?;
                    }
                    if self.0.contains(SubmitFlags::PROTECTED_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SubmitFlags)).field(&Flags(*self)).finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubmitFlags {
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
unsafe impl crate::conv::FromLowLevel for SubmitFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPipelineCreationFeedbackFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PipelineCreationFeedbackFlagBits(u32);
impl Default for PipelineCreationFeedbackFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PipelineCreationFeedbackFlagBits {
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT")]
    pub const VALID: Self = Self(1);
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT")]
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(2);
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT")]
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(4);
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT")]
    pub const VALID_EXT: Self = Self::VALID;
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT")]
    pub const APPLICATION_PIPELINE_CACHE_HIT_EXT: Self = Self::APPLICATION_PIPELINE_CACHE_HIT;
    #[doc(alias = "VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT")]
    pub const BASE_PIPELINE_ACCELERATION_EXT: Self = Self::BASE_PIPELINE_ACCELERATION;
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
            x if x == Self::VALID.bits() => Some(Self(x)),
            x if x == Self::APPLICATION_PIPELINE_CACHE_HIT.bits() => Some(Self(x)),
            x if x == Self::BASE_PIPELINE_ACCELERATION.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for PipelineCreationFeedbackFlagBits {
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
unsafe impl crate::conv::FromLowLevel for PipelineCreationFeedbackFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkToolPurposeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ToolPurposeFlagBits(u32);
impl Default for ToolPurposeFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ToolPurposeFlagBits {
    #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT")]
    pub const VALIDATION: Self = Self(1);
    #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT")]
    pub const PROFILING: Self = Self(2);
    #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT")]
    pub const TRACING: Self = Self(4);
    #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT")]
    pub const ADDITIONAL_FEATURES: Self = Self(8);
    #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT")]
    pub const MODIFYING_FEATURES: Self = Self(16);
    #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT")]
    #[cfg(feature = "VK_EXT_tooling_info")]
    pub const DEBUG_REPORTING_EXT: Self = Self(32);
    #[doc(alias = "VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT")]
    #[cfg(feature = "VK_EXT_tooling_info")]
    pub const DEBUG_MARKERS_EXT: Self = Self(64);
    #[doc(alias = "VK_TOOL_PURPOSE_VALIDATION_BIT_EXT")]
    pub const VALIDATION_EXT: Self = Self::VALIDATION;
    #[doc(alias = "VK_TOOL_PURPOSE_PROFILING_BIT_EXT")]
    pub const PROFILING_EXT: Self = Self::PROFILING;
    #[doc(alias = "VK_TOOL_PURPOSE_TRACING_BIT_EXT")]
    pub const TRACING_EXT: Self = Self::TRACING;
    #[doc(alias = "VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT")]
    pub const ADDITIONAL_FEATURES_EXT: Self = Self::ADDITIONAL_FEATURES;
    #[doc(alias = "VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT")]
    pub const MODIFYING_FEATURES_EXT: Self = Self::MODIFYING_FEATURES;
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
            x if x == Self::VALIDATION.bits() => Some(Self(x)),
            x if x == Self::PROFILING.bits() => Some(Self(x)),
            x if x == Self::TRACING.bits() => Some(Self(x)),
            x if x == Self::ADDITIONAL_FEATURES.bits() => Some(Self(x)),
            x if x == Self::MODIFYING_FEATURES.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_tooling_info")]
            x if x == Self::DEBUG_REPORTING_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_tooling_info")]
            x if x == Self::DEBUG_MARKERS_EXT.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ToolPurposeFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ToolPurposeFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccessFlagBits2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccessFlagBits2(u64);
impl Default for AccessFlagBits2 {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AccessFlagBits2 {
    #[doc(alias = "VK_ACCESS_2_NONE")]
    pub const ACCESS2_NONE: Self = Self(0);
    #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT")]
    pub const ACCESS2_INDIRECT_COMMAND_READ: Self = Self(1);
    #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT")]
    pub const ACCESS2_INDEX_READ: Self = Self(2);
    #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT")]
    pub const ACCESS2_VERTEX_ATTRIBUTE_READ: Self = Self(4);
    #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT")]
    pub const ACCESS2_UNIFORM_READ: Self = Self(8);
    #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT")]
    pub const ACCESS2_INPUT_ATTACHMENT_READ: Self = Self(16);
    #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT")]
    pub const ACCESS2_SHADER_READ: Self = Self(32);
    #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT")]
    pub const ACCESS2_SHADER_WRITE: Self = Self(64);
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT")]
    pub const ACCESS2_COLOR_ATTACHMENT_READ: Self = Self(128);
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT")]
    pub const ACCESS2_COLOR_ATTACHMENT_WRITE: Self = Self(256);
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512);
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024);
    #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT")]
    pub const ACCESS2_TRANSFER_READ: Self = Self(2048);
    #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT")]
    pub const ACCESS2_TRANSFER_WRITE: Self = Self(4096);
    #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT")]
    pub const ACCESS2_HOST_READ: Self = Self(8192);
    #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT")]
    pub const ACCESS2_HOST_WRITE: Self = Self(16384);
    #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT")]
    pub const ACCESS2_MEMORY_READ: Self = Self(32768);
    #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT")]
    pub const ACCESS2_MEMORY_WRITE: Self = Self(65536);
    #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT")]
    pub const ACCESS2_SHADER_SAMPLED_READ: Self = Self(4294967296);
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT")]
    pub const ACCESS2_SHADER_STORAGE_READ: Self = Self(8589934592);
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT")]
    pub const ACCESS2_SHADER_STORAGE_WRITE: Self = Self(17179869184);
    #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const ACCESS2_VIDEO_DECODE_READ_KHR: Self = Self(34359738368);
    #[doc(alias = "VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const ACCESS2_VIDEO_DECODE_WRITE_KHR: Self = Self(68719476736);
    #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const ACCESS2_VIDEO_ENCODE_READ_KHR: Self = Self(137438953472);
    #[doc(alias = "VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const ACCESS2_VIDEO_ENCODE_WRITE_KHR: Self = Self(274877906944);
    #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(33554432);
    #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(67108864);
    #[doc(alias = "VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(134217728);
    #[doc(alias = "VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576);
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_COMMAND_PREPROCESS_READ_NV: Self = Self(131072);
    #[doc(alias = "VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_COMMAND_PREPROCESS_WRITE_NV: Self = Self(262144);
    #[doc(alias = "VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608);
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152);
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304);
    #[doc(alias = "VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216);
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288);
    #[doc(alias = "VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI")]
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    pub const ACCESS2_INVOCATION_MASK_READ_HUAWEI: Self = Self(549755813888);
    #[doc(alias = "VK_ACCESS_2_NONE_KHR")]
    pub const ACCESS2_NONE_KHR: Self = Self::ACCESS2_NONE;
    #[doc(alias = "VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR")]
    pub const ACCESS2_INDIRECT_COMMAND_READ_KHR: Self = Self::ACCESS2_INDIRECT_COMMAND_READ;
    #[doc(alias = "VK_ACCESS_2_INDEX_READ_BIT_KHR")]
    pub const ACCESS2_INDEX_READ_KHR: Self = Self::ACCESS2_INDEX_READ;
    #[doc(alias = "VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR")]
    pub const ACCESS2_VERTEX_ATTRIBUTE_READ_KHR: Self = Self::ACCESS2_VERTEX_ATTRIBUTE_READ;
    #[doc(alias = "VK_ACCESS_2_UNIFORM_READ_BIT_KHR")]
    pub const ACCESS2_UNIFORM_READ_KHR: Self = Self::ACCESS2_UNIFORM_READ;
    #[doc(alias = "VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR")]
    pub const ACCESS2_INPUT_ATTACHMENT_READ_KHR: Self = Self::ACCESS2_INPUT_ATTACHMENT_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_READ_BIT_KHR")]
    pub const ACCESS2_SHADER_READ_KHR: Self = Self::ACCESS2_SHADER_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_WRITE_BIT_KHR")]
    pub const ACCESS2_SHADER_WRITE_KHR: Self = Self::ACCESS2_SHADER_WRITE;
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR")]
    pub const ACCESS2_COLOR_ATTACHMENT_READ_KHR: Self = Self::ACCESS2_COLOR_ATTACHMENT_READ;
    #[doc(alias = "VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR")]
    pub const ACCESS2_COLOR_ATTACHMENT_WRITE_KHR: Self = Self::ACCESS2_COLOR_ATTACHMENT_WRITE;
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ_KHR: Self = Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ;
    #[doc(alias = "VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR")]
    pub const ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE_KHR: Self = Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE;
    #[doc(alias = "VK_ACCESS_2_TRANSFER_READ_BIT_KHR")]
    pub const ACCESS2_TRANSFER_READ_KHR: Self = Self::ACCESS2_TRANSFER_READ;
    #[doc(alias = "VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR")]
    pub const ACCESS2_TRANSFER_WRITE_KHR: Self = Self::ACCESS2_TRANSFER_WRITE;
    #[doc(alias = "VK_ACCESS_2_HOST_READ_BIT_KHR")]
    pub const ACCESS2_HOST_READ_KHR: Self = Self::ACCESS2_HOST_READ;
    #[doc(alias = "VK_ACCESS_2_HOST_WRITE_BIT_KHR")]
    pub const ACCESS2_HOST_WRITE_KHR: Self = Self::ACCESS2_HOST_WRITE;
    #[doc(alias = "VK_ACCESS_2_MEMORY_READ_BIT_KHR")]
    pub const ACCESS2_MEMORY_READ_KHR: Self = Self::ACCESS2_MEMORY_READ;
    #[doc(alias = "VK_ACCESS_2_MEMORY_WRITE_BIT_KHR")]
    pub const ACCESS2_MEMORY_WRITE_KHR: Self = Self::ACCESS2_MEMORY_WRITE;
    #[doc(alias = "VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR")]
    pub const ACCESS2_SHADER_SAMPLED_READ_KHR: Self = Self::ACCESS2_SHADER_SAMPLED_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR")]
    pub const ACCESS2_SHADER_STORAGE_READ_KHR: Self = Self::ACCESS2_SHADER_STORAGE_READ;
    #[doc(alias = "VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR")]
    pub const ACCESS2_SHADER_STORAGE_WRITE_KHR: Self = Self::ACCESS2_SHADER_STORAGE_WRITE;
    #[doc(alias = "VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_SHADING_RATE_IMAGE_READ_NV: Self = Self::ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCESS2_ACCELERATION_STRUCTURE_READ_KHR;
    #[doc(alias = "VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const ACCESS2_ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u64 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u64) -> Option<Self> {
        match bits {
            x if x == Self::ACCESS2_NONE.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_INDIRECT_COMMAND_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_INDEX_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_VERTEX_ATTRIBUTE_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_UNIFORM_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_INPUT_ATTACHMENT_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_SHADER_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_SHADER_WRITE.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_COLOR_ATTACHMENT_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_COLOR_ATTACHMENT_WRITE.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_DEPTH_STENCIL_ATTACHMENT_WRITE.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_TRANSFER_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_TRANSFER_WRITE.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_HOST_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_HOST_WRITE.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_MEMORY_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_MEMORY_WRITE.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_SHADER_SAMPLED_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_SHADER_STORAGE_READ.bits() => Some(Self(x)),
            x if x == Self::ACCESS2_SHADER_STORAGE_WRITE.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            x if x == Self::ACCESS2_VIDEO_DECODE_READ_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            x if x == Self::ACCESS2_VIDEO_DECODE_WRITE_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            x if x == Self::ACCESS2_VIDEO_ENCODE_READ_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            x if x == Self::ACCESS2_VIDEO_ENCODE_WRITE_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_TRANSFORM_FEEDBACK_WRITE_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_CONDITIONAL_RENDERING_READ_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_COMMAND_PREPROCESS_READ_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_COMMAND_PREPROCESS_WRITE_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_ACCELERATION_STRUCTURE_READ_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_ACCELERATION_STRUCTURE_WRITE_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_FRAGMENT_DENSITY_MAP_READ_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::ACCESS2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_AMD_extension_317")]
            x if x == Self::ACCESS2_RESERVED_41_AMD.bits() => Some(Self(x)),
            #[cfg(feature = "VK_HUAWEI_invocation_mask")]
            x if x == Self::ACCESS2_INVOCATION_MASK_READ_HUAWEI.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_extension_387")]
            x if x == Self::ACCESS2_RESERVED_387_KHR.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u64) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccessFlagBits2 {
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
unsafe impl crate::conv::FromLowLevel for AccessFlagBits2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPipelineStageFlagBits2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PipelineStageFlagBits2(u64);
impl Default for PipelineStageFlagBits2 {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PipelineStageFlagBits2 {
    #[doc(alias = "VK_PIPELINE_STAGE_2_NONE")]
    pub const PIPELINE_STAGE2_NONE: Self = Self(0);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT")]
    pub const PIPELINE_STAGE2_TOP_OF_PIPE: Self = Self(1);
    #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT")]
    pub const PIPELINE_STAGE2_DRAW_INDIRECT: Self = Self(2);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT")]
    pub const PIPELINE_STAGE2_VERTEX_INPUT: Self = Self(4);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT")]
    pub const PIPELINE_STAGE2_VERTEX_SHADER: Self = Self(8);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT")]
    pub const PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER: Self = Self(16);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT")]
    pub const PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER: Self = Self(32);
    #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT")]
    pub const PIPELINE_STAGE2_GEOMETRY_SHADER: Self = Self(64);
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT")]
    pub const PIPELINE_STAGE2_FRAGMENT_SHADER: Self = Self(128);
    #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT")]
    pub const PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS: Self = Self(256);
    #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT")]
    pub const PIPELINE_STAGE2_LATE_FRAGMENT_TESTS: Self = Self(512);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT")]
    pub const PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT: Self = Self(1024);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT")]
    pub const PIPELINE_STAGE2_COMPUTE_SHADER: Self = Self(2048);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT")]
    pub const PIPELINE_STAGE2_ALL_TRANSFER: Self = Self(4096);
    #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT")]
    pub const PIPELINE_STAGE2_BOTTOM_OF_PIPE: Self = Self(8192);
    #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT")]
    pub const PIPELINE_STAGE2_HOST: Self = Self(16384);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT")]
    pub const PIPELINE_STAGE2_ALL_GRAPHICS: Self = Self(32768);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT")]
    pub const PIPELINE_STAGE2_ALL_COMMANDS: Self = Self(65536);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT")]
    pub const PIPELINE_STAGE2_COPY: Self = Self(4294967296);
    #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT")]
    pub const PIPELINE_STAGE2_RESOLVE: Self = Self(8589934592);
    #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT")]
    pub const PIPELINE_STAGE2_BLIT: Self = Self(17179869184);
    #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT")]
    pub const PIPELINE_STAGE2_CLEAR: Self = Self(34359738368);
    #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT")]
    pub const PIPELINE_STAGE2_INDEX_INPUT: Self = Self(68719476736);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT")]
    pub const PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT: Self = Self(137438953472);
    #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT")]
    pub const PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS: Self = Self(274877906944);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const PIPELINE_STAGE2_VIDEO_DECODE_KHR: Self = Self(67108864);
    #[doc(alias = "VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const PIPELINE_STAGE2_VIDEO_ENCODE_KHR: Self = Self(134217728);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT: Self = Self(16777216);
    #[doc(alias = "VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT: Self = Self(262144);
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_COMMAND_PREPROCESS_NV: Self = Self(131072);
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304);
    #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432);
    #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR: Self = Self(2097152);
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608);
    #[doc(alias = "VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_TASK_SHADER_NV: Self = Self(524288);
    #[doc(alias = "VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_MESH_SHADER_NV: Self = Self(1048576);
    #[doc(alias = "VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI")]
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    pub const PIPELINE_STAGE2_SUBPASS_SHADING_HUAWEI: Self = Self(549755813888);
    #[doc(alias = "VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI")]
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    pub const PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI: Self = Self(1099511627776);
    #[doc(alias = "VK_PIPELINE_STAGE_2_NONE_KHR")]
    pub const PIPELINE_STAGE2_NONE_KHR: Self = Self::PIPELINE_STAGE2_NONE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR")]
    pub const PIPELINE_STAGE2_TOP_OF_PIPE_KHR: Self = Self::PIPELINE_STAGE2_TOP_OF_PIPE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR")]
    pub const PIPELINE_STAGE2_DRAW_INDIRECT_KHR: Self = Self::PIPELINE_STAGE2_DRAW_INDIRECT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_VERTEX_INPUT_KHR: Self = Self::PIPELINE_STAGE2_VERTEX_INPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_VERTEX_SHADER_KHR: Self = Self::PIPELINE_STAGE2_VERTEX_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER_KHR: Self = Self::PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER_KHR: Self =
        Self::PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_GEOMETRY_SHADER_KHR: Self = Self::PIPELINE_STAGE2_GEOMETRY_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_FRAGMENT_SHADER_KHR: Self = Self::PIPELINE_STAGE2_FRAGMENT_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR")]
    pub const PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS_KHR: Self = Self::PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR")]
    pub const PIPELINE_STAGE2_LATE_FRAGMENT_TESTS_KHR: Self = Self::PIPELINE_STAGE2_LATE_FRAGMENT_TESTS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT_KHR: Self = Self::PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR")]
    pub const PIPELINE_STAGE2_COMPUTE_SHADER_KHR: Self = Self::PIPELINE_STAGE2_COMPUTE_SHADER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR")]
    pub const PIPELINE_STAGE2_ALL_TRANSFER_KHR: Self = Self::PIPELINE_STAGE2_ALL_TRANSFER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT")]
    pub const PIPELINE_STAGE2_TRANSFER: Self = Self::PIPELINE_STAGE2_ALL_TRANSFER_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR")]
    pub const PIPELINE_STAGE2_TRANSFER_KHR: Self = Self::PIPELINE_STAGE2_TRANSFER;
    #[doc(alias = "VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR")]
    pub const PIPELINE_STAGE2_BOTTOM_OF_PIPE_KHR: Self = Self::PIPELINE_STAGE2_BOTTOM_OF_PIPE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_HOST_BIT_KHR")]
    pub const PIPELINE_STAGE2_HOST_KHR: Self = Self::PIPELINE_STAGE2_HOST;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR")]
    pub const PIPELINE_STAGE2_ALL_GRAPHICS_KHR: Self = Self::PIPELINE_STAGE2_ALL_GRAPHICS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR")]
    pub const PIPELINE_STAGE2_ALL_COMMANDS_KHR: Self = Self::PIPELINE_STAGE2_ALL_COMMANDS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_COPY_BIT_KHR")]
    pub const PIPELINE_STAGE2_COPY_KHR: Self = Self::PIPELINE_STAGE2_COPY;
    #[doc(alias = "VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR")]
    pub const PIPELINE_STAGE2_RESOLVE_KHR: Self = Self::PIPELINE_STAGE2_RESOLVE;
    #[doc(alias = "VK_PIPELINE_STAGE_2_BLIT_BIT_KHR")]
    pub const PIPELINE_STAGE2_BLIT_KHR: Self = Self::PIPELINE_STAGE2_BLIT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR")]
    pub const PIPELINE_STAGE2_CLEAR_KHR: Self = Self::PIPELINE_STAGE2_CLEAR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_INDEX_INPUT_KHR: Self = Self::PIPELINE_STAGE2_INDEX_INPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR")]
    pub const PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT_KHR: Self = Self::PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT;
    #[doc(alias = "VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR")]
    pub const PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS_KHR: Self = Self::PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS;
    #[doc(alias = "VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_SHADING_RATE_IMAGE_NV: Self = Self::PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_RAY_TRACING_SHADER_NV: Self = Self::PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR;
    #[doc(alias = "VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV")]
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub const PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_NV: Self =
        Self::PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u64 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u64) -> Option<Self> {
        match bits {
            x if x == Self::PIPELINE_STAGE2_NONE.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_TOP_OF_PIPE.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_DRAW_INDIRECT.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_VERTEX_INPUT.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_VERTEX_SHADER.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_TESSELLATION_CONTROL_SHADER.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_TESSELLATION_EVALUATION_SHADER.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_GEOMETRY_SHADER.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_FRAGMENT_SHADER.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_EARLY_FRAGMENT_TESTS.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_LATE_FRAGMENT_TESTS.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_COLOR_ATTACHMENT_OUTPUT.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_COMPUTE_SHADER.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_ALL_TRANSFER.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_BOTTOM_OF_PIPE.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_HOST.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_ALL_GRAPHICS.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_ALL_COMMANDS.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_COPY.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_RESOLVE.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_BLIT.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_CLEAR.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_INDEX_INPUT.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_VERTEX_ATTRIBUTE_INPUT.bits() => Some(Self(x)),
            x if x == Self::PIPELINE_STAGE2_PRE_RASTERIZATION_SHADERS.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            x if x == Self::PIPELINE_STAGE2_VIDEO_DECODE_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            x if x == Self::PIPELINE_STAGE2_VIDEO_ENCODE_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_TRANSFORM_FEEDBACK_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_CONDITIONAL_RENDERING_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_COMMAND_PREPROCESS_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_ACCELERATION_STRUCTURE_BUILD_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_RAY_TRACING_SHADER_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_FRAGMENT_DENSITY_PROCESS_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_TASK_SHADER_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_synchronization2")]
            x if x == Self::PIPELINE_STAGE2_MESH_SHADER_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_HUAWEI_subpass_shading")]
            x if x == Self::PIPELINE_STAGE2_SUBPASS_SHADING_HUAWEI.bits() => Some(Self(x)),
            #[cfg(feature = "VK_HUAWEI_invocation_mask")]
            x if x == Self::PIPELINE_STAGE2_INVOCATION_MASK_HUAWEI.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_extension_387")]
            x if x == Self::PIPELINE_STAGE2_RESERVED_387_KHR.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u64) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineStageFlagBits2 {
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
unsafe impl crate::conv::FromLowLevel for PipelineStageFlagBits2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSubmitFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SubmitFlagBits(u32);
impl Default for SubmitFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SubmitFlagBits {
    #[doc(alias = "VK_SUBMIT_PROTECTED_BIT")]
    pub const PROTECTED: Self = Self(1);
    #[doc(alias = "VK_SUBMIT_PROTECTED_BIT_KHR")]
    pub const PROTECTED_KHR: Self = Self::PROTECTED;
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
            x if x == Self::PROTECTED.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for SubmitFlagBits {
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
unsafe impl crate::conv::FromLowLevel for SubmitFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkFormatFeatureFlagBits2")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FormatFeatureFlagBits2(u64);
impl Default for FormatFeatureFlagBits2 {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl FormatFeatureFlagBits2 {
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE: Self = Self(1);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE: Self = Self(2);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC: Self = Self(4);
    #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT")]
    pub const FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER: Self = Self(8);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER: Self = Self(16);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT")]
    pub const FORMAT_FEATURE2_VERTEX_BUFFER: Self = Self(64);
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT: Self = Self(128);
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND: Self = Self(256);
    #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT")]
    pub const FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT: Self = Self(512);
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT")]
    pub const FORMAT_FEATURE2_BLIT_SRC: Self = Self(1024);
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT")]
    pub const FORMAT_FEATURE2_BLIT_DST: Self = Self(2048);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(8192);
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT")]
    pub const FORMAT_FEATURE2_TRANSFER_SRC: Self = Self(16384);
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT")]
    pub const FORMAT_FEATURE2_TRANSFER_DST: Self = Self(32768);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(65536);
    #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT")]
    pub const FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES: Self = Self(131072);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(262144);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(524288);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(1048576);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(2097152);
    #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT")]
    pub const FORMAT_FEATURE2_DISJOINT: Self = Self(4194304);
    #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT")]
    pub const FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES: Self = Self(8388608);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT: Self = Self(2147483648);
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT")]
    pub const FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(4294967296);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(8589934592);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_DECODE_DPB_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_DECODE_DPB_KHR: Self = Self(67108864);
    #[doc(alias = "VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR")]
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub const FORMAT_FEATURE2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(536870912);
    #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT")]
    #[cfg(feature = "VK_EXT_fragment_density_map")]
    pub const FORMAT_FEATURE2_FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216);
    #[doc(alias = "VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    pub const FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1073741824);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728);
    #[doc(alias = "VK_FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_BIT_KHR")]
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    pub const FORMAT_FEATURE2_VIDEO_ENCODE_DPB_KHR: Self = Self(268435456);
    #[doc(alias = "VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV")]
    #[cfg(feature = "VK_NV_linear_color_attachment")]
    pub const FORMAT_FEATURE2_LINEAR_COLOR_ATTACHMENT_NV: Self = Self(274877906944);
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_KHR: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_IMAGE;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR")]
    pub const FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER_KHR: Self = Self::FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR")]
    pub const FORMAT_FEATURE2_VERTEX_BUFFER_KHR: Self = Self::FORMAT_FEATURE2_VERTEX_BUFFER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT_KHR: Self = Self::FORMAT_FEATURE2_COLOR_ATTACHMENT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR")]
    pub const FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND_KHR: Self = Self::FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND;
    #[doc(alias = "VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR")]
    pub const FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self::FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR")]
    pub const FORMAT_FEATURE2_BLIT_SRC_KHR: Self = Self::FORMAT_FEATURE2_BLIT_SRC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR")]
    pub const FORMAT_FEATURE2_BLIT_DST_KHR: Self = Self::FORMAT_FEATURE2_BLIT_DST;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR")]
    pub const FORMAT_FEATURE2_TRANSFER_SRC_KHR: Self = Self::FORMAT_FEATURE2_TRANSFER_SRC;
    #[doc(alias = "VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR")]
    pub const FORMAT_FEATURE2_TRANSFER_DST_KHR: Self = Self::FORMAT_FEATURE2_TRANSFER_DST;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX;
    #[doc(alias = "VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR")]
    pub const FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    #[doc(
        alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR"
    )]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    #[doc(alias = "VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR")]
    pub const FORMAT_FEATURE2_DISJOINT_KHR: Self = Self::FORMAT_FEATURE2_DISJOINT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR")]
    pub const FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES_KHR: Self = Self::FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self::FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR")]
    pub const FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self =
        Self::FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT;
    #[doc(alias = "VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR")]
    pub const FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self =
        Self::FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u64 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u64) -> Option<Self> {
        match bits {
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_STORAGE_IMAGE.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_STORAGE_IMAGE_ATOMIC.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_UNIFORM_TEXEL_BUFFER.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_STORAGE_TEXEL_BUFFER_ATOMIC.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_VERTEX_BUFFER.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_COLOR_ATTACHMENT.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_COLOR_ATTACHMENT_BLEND.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_DEPTH_STENCIL_ATTACHMENT.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_BLIT_SRC.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_BLIT_DST.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_LINEAR.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_CUBIC.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_TRANSFER_SRC.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_TRANSFER_DST.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_FILTER_MINMAX.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_MIDPOINT_CHROMA_SAMPLES.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.bits() => {
                Some(Self(x))
            },
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.bits() => {
                Some(Self(x))
            },
            x if x
                == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE
                    .bits() =>
            {
                Some(Self(x))
            },
            x if x == Self::FORMAT_FEATURE2_DISJOINT.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_COSITED_CHROMA_SAMPLES.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_STORAGE_READ_WITHOUT_FORMAT.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_STORAGE_WRITE_WITHOUT_FORMAT.bits() => Some(Self(x)),
            x if x == Self::FORMAT_FEATURE2_SAMPLED_IMAGE_DEPTH_COMPARISON.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            x if x == Self::FORMAT_FEATURE2_VIDEO_DECODE_OUTPUT_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_decode_queue")]
            x if x == Self::FORMAT_FEATURE2_VIDEO_DECODE_DPB_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_acceleration_structure")]
            x if x == Self::FORMAT_FEATURE2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_fragment_density_map")]
            x if x == Self::FORMAT_FEATURE2_FRAGMENT_DENSITY_MAP_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_fragment_shading_rate")]
            x if x == Self::FORMAT_FEATURE2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            x if x == Self::FORMAT_FEATURE2_VIDEO_ENCODE_INPUT_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            x if x == Self::FORMAT_FEATURE2_VIDEO_ENCODE_DPB_KHR.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_linear_color_attachment")]
            x if x == Self::FORMAT_FEATURE2_LINEAR_COLOR_ATTACHMENT_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_QCOM_extension_441")]
            x if x == Self::FORMAT_FEATURE2_RESERVED_34_QCOM.bits() => Some(Self(x)),
            #[cfg(feature = "VK_QCOM_extension_441")]
            x if x == Self::FORMAT_FEATURE2_RESERVED_35_QCOM.bits() => Some(Self(x)),
            #[cfg(feature = "VK_QCOM_extension_441")]
            x if x == Self::FORMAT_FEATURE2_RESERVED_36_QCOM.bits() => Some(Self(x)),
            #[cfg(feature = "VK_QCOM_extension_441")]
            x if x == Self::FORMAT_FEATURE2_RESERVED_37_QCOM.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_extension_461")]
            x if x == Self::FORMAT_FEATURE2_RESERVED_39_EXT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u64) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FormatFeatureFlagBits2 {
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
unsafe impl crate::conv::FromLowLevel for FormatFeatureFlagBits2 {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkRenderingFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct RenderingFlagBits(u32);
impl Default for RenderingFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl RenderingFlagBits {
    #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT")]
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1);
    #[doc(alias = "VK_RENDERING_SUSPENDING_BIT")]
    pub const SUSPENDING: Self = Self(2);
    #[doc(alias = "VK_RENDERING_RESUMING_BIT")]
    pub const RESUMING: Self = Self(4);
    #[doc(alias = "VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR")]
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self::CONTENTS_SECONDARY_COMMAND_BUFFERS;
    #[doc(alias = "VK_RENDERING_SUSPENDING_BIT_KHR")]
    pub const SUSPENDING_KHR: Self = Self::SUSPENDING;
    #[doc(alias = "VK_RENDERING_RESUMING_BIT_KHR")]
    pub const RESUMING_KHR: Self = Self::RESUMING;
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
            x if x == Self::CONTENTS_SECONDARY_COMMAND_BUFFERS.bits() => Some(Self(x)),
            x if x == Self::SUSPENDING.bits() => Some(Self(x)),
            x if x == Self::RESUMING.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for RenderingFlagBits {
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
unsafe impl crate::conv::FromLowLevel for RenderingFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
