use crate::common::vulkan1_0::{DescriptorType, ImageAspectFlags};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[doc(alias = "VkExternalMemoryProperties")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalMemoryProperties {
    #[doc(alias = "externalMemoryFeatures")]
    pub external_memory_features: ExternalMemoryFeatureFlags,
    #[doc(alias = "exportFromImportedHandleTypes")]
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    #[doc(alias = "compatibleHandleTypes")]
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
#[doc(alias = "VkDescriptorUpdateTemplateEntry")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    #[doc(alias = "dstBinding")]
    pub dst_binding: u32,
    #[doc(alias = "dstArrayElement")]
    pub dst_array_element: u32,
    #[doc(alias = "descriptorCount")]
    pub descriptor_count: u32,
    #[doc(alias = "descriptorType")]
    pub descriptor_type: DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
#[doc(alias = "VkInputAttachmentAspectReference")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    #[doc(alias = "inputAttachmentIndex")]
    pub input_attachment_index: u32,
    #[doc(alias = "aspectMask")]
    pub aspect_mask: ImageAspectFlags,
}
#[doc(alias = "VkSubgroupFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubgroupFeatureFlags(u32);
impl SubgroupFeatureFlags {
    #[doc(alias = "VK_SUBGROUP_FEATURE_BASIC_BIT")]
    pub const BASIC: Self = Self(1);
    #[doc(alias = "VK_SUBGROUP_FEATURE_VOTE_BIT")]
    pub const VOTE: Self = Self(2);
    #[doc(alias = "VK_SUBGROUP_FEATURE_ARITHMETIC_BIT")]
    pub const ARITHMETIC: Self = Self(4);
    #[doc(alias = "VK_SUBGROUP_FEATURE_BALLOT_BIT")]
    pub const BALLOT: Self = Self(8);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_BIT")]
    pub const SHUFFLE: Self = Self(16);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT")]
    pub const SHUFFLE_RELATIVE: Self = Self(32);
    #[doc(alias = "VK_SUBGROUP_FEATURE_CLUSTERED_BIT")]
    pub const CLUSTERED: Self = Self(64);
    #[doc(alias = "VK_SUBGROUP_FEATURE_QUAD_BIT")]
    pub const QUAD: Self = Self(128);
    #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV")]
    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
    pub const PARTITIONED_NV: Self = Self(256);
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
            all |= Self::BASIC;
        }
        {
            all |= Self::VOTE;
        }
        {
            all |= Self::ARITHMETIC;
        }
        {
            all |= Self::BALLOT;
        }
        {
            all |= Self::SHUFFLE;
        }
        {
            all |= Self::SHUFFLE_RELATIVE;
        }
        {
            all |= Self::CLUSTERED;
        }
        {
            all |= Self::QUAD;
        }
        #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
        {
            all |= Self::PARTITIONED_NV;
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
impl std::ops::BitOr for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SubgroupFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SubgroupFeatureFlags> for SubgroupFeatureFlags {
    fn extend<T: IntoIterator<Item = SubgroupFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SubgroupFeatureFlags> for SubgroupFeatureFlags {
    fn from_iter<T: IntoIterator<Item = SubgroupFeatureFlags>>(iterator: T) -> SubgroupFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<SubgroupFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for SubgroupFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn from(bit: SubgroupFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn extend<T: IntoIterator<Item = SubgroupFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn from_iter<T: IntoIterator<Item = SubgroupFeatureFlagBits>>(iterator: T) -> SubgroupFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<SubgroupFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SubgroupFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SubgroupFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SubgroupFeatureFlags::BASIC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(BASIC))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::VOTE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VOTE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::ARITHMETIC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ARITHMETIC))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::BALLOT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(BALLOT))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SHUFFLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SHUFFLE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SHUFFLE_RELATIVE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SHUFFLE_RELATIVE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::CLUSTERED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CLUSTERED))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::QUAD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(QUAD))?;
                    }
                    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
                    if self.0.contains(SubgroupFeatureFlags::PARTITIONED_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PARTITIONED_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SubgroupFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SubgroupFeatureFlags {
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
unsafe impl crate::conv::FromLowLevel for SubgroupFeatureFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorUpdateTemplateCreateFlags(u32);
impl DescriptorUpdateTemplateCreateFlags {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for DescriptorUpdateTemplateCreateFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorUpdateTemplateCreateFlags {
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
unsafe impl crate::conv::FromLowLevel for DescriptorUpdateTemplateCreateFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPeerMemoryFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PeerMemoryFeatureFlags(u32);
impl PeerMemoryFeatureFlags {
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT")]
    pub const COPY_SRC: Self = Self(1);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT")]
    pub const COPY_DST: Self = Self(2);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT")]
    pub const GENERIC_SRC: Self = Self(4);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT")]
    pub const GENERIC_DST: Self = Self(8);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
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
            all |= Self::COPY_SRC;
        }
        {
            all |= Self::COPY_DST;
        }
        {
            all |= Self::GENERIC_SRC;
        }
        {
            all |= Self::GENERIC_DST;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::COPY_SRC_KHR;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::COPY_DST_KHR;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::GENERIC_SRC_KHR;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::GENERIC_DST_KHR;
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
impl std::ops::BitOr for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PeerMemoryFeatureFlags> for PeerMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = PeerMemoryFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<PeerMemoryFeatureFlags> for PeerMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = PeerMemoryFeatureFlags>>(iterator: T) -> PeerMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<PeerMemoryFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for PeerMemoryFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn from(bit: PeerMemoryFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = PeerMemoryFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = PeerMemoryFeatureFlagBits>>(iterator: T) -> PeerMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<PeerMemoryFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PeerMemoryFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PeerMemoryFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_SRC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_SRC))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_DST) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_DST))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_SRC) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_SRC))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_DST) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_DST))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_SRC_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_SRC_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::COPY_DST_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(COPY_DST_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_SRC_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_SRC_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(PeerMemoryFeatureFlags::GENERIC_DST_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(GENERIC_DST_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PeerMemoryFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PeerMemoryFeatureFlags {
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
unsafe impl crate::conv::FromLowLevel for PeerMemoryFeatureFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkMemoryAllocateFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryAllocateFlags(u32);
impl MemoryAllocateFlags {
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT")]
    pub const DEVICE_MASK: Self = Self(1);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS: Self = Self(2);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
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
            all |= Self::DEVICE_MASK;
        }
        #[cfg(feature = "VULKAN_1_2")]
        {
            all |= Self::DEVICE_ADDRESS;
        }
        #[cfg(feature = "VULKAN_1_2")]
        {
            all |= Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        }
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::DEVICE_MASK_KHR;
        }
        #[cfg(feature = "VK_KHR_buffer_device_address")]
        {
            all |= Self::DEVICE_ADDRESS_KHR;
        }
        #[cfg(feature = "VK_KHR_buffer_device_address")]
        {
            all |= Self::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR;
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
impl std::ops::BitOr for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for MemoryAllocateFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for MemoryAllocateFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for MemoryAllocateFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for MemoryAllocateFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<MemoryAllocateFlags> for MemoryAllocateFlags {
    fn extend<T: IntoIterator<Item = MemoryAllocateFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<MemoryAllocateFlags> for MemoryAllocateFlags {
    fn from_iter<T: IntoIterator<Item = MemoryAllocateFlags>>(iterator: T) -> MemoryAllocateFlags {
        let mut out = Self::empty();
        <Self as Extend<MemoryAllocateFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for MemoryAllocateFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn from(bit: MemoryAllocateFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn extend<T: IntoIterator<Item = MemoryAllocateFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn from_iter<T: IntoIterator<Item = MemoryAllocateFlagBits>>(iterator: T) -> MemoryAllocateFlags {
        let mut out = Self::empty();
        <Self as Extend<MemoryAllocateFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for MemoryAllocateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(MemoryAllocateFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == MemoryAllocateFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(MemoryAllocateFlags::DEVICE_MASK) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_MASK))?;
                    }
                    #[cfg(feature = "VULKAN_1_2")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS))?;
                    }
                    #[cfg(feature = "VULKAN_1_2")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_CAPTURE_REPLAY))?;
                    }
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_MASK_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_MASK_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_buffer_device_address")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_buffer_device_address")]
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_CAPTURE_REPLAY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(MemoryAllocateFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for MemoryAllocateFlags {
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
unsafe impl crate::conv::FromLowLevel for MemoryAllocateFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkCommandPoolTrimFlags")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandPoolTrimFlags(u32);
impl CommandPoolTrimFlags {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for CommandPoolTrimFlags {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CommandPoolTrimFlags {
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
unsafe impl crate::conv::FromLowLevel for CommandPoolTrimFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalMemoryHandleTypeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryHandleTypeFlags(u32);
impl ExternalMemoryHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT")]
    pub const D3D11_TEXTURE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT")]
    pub const D3D11_TEXTURE_KMT: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT")]
    pub const D3D12_HEAP: Self = Self(32);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT")]
    pub const D3D12_RESOURCE: Self = Self(64);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    pub const DMA_BUF_EXT: Self = Self(512);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID")]
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1024);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_ALLOCATION_EXT: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(2048);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV")]
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    pub const RDMA_ADDRESS_NV: Self = Self(4096);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
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
            all |= Self::OPAQUE_FD;
        }
        {
            all |= Self::OPAQUE_WIN32;
        }
        {
            all |= Self::OPAQUE_WIN32_KMT;
        }
        {
            all |= Self::D3D11_TEXTURE;
        }
        {
            all |= Self::D3D11_TEXTURE_KMT;
        }
        {
            all |= Self::D3D12_HEAP;
        }
        {
            all |= Self::D3D12_RESOURCE;
        }
        #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
        {
            all |= Self::DMA_BUF_EXT;
        }
        #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
        {
            all |= Self::ANDROID_HARDWARE_BUFFER_ANDROID;
        }
        #[cfg(feature = "VK_EXT_external_memory_host")]
        {
            all |= Self::HOST_ALLOCATION_EXT;
        }
        #[cfg(feature = "VK_EXT_external_memory_host")]
        {
            all |= Self::HOST_MAPPED_FOREIGN_MEMORY_EXT;
        }
        #[cfg(feature = "VK_FUCHSIA_external_memory")]
        {
            all |= Self::ZIRCON_VMO_FUCHSIA;
        }
        #[cfg(feature = "VK_NV_external_memory_rdma")]
        {
            all |= Self::RDMA_ADDRESS_NV;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::OPAQUE_FD_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D11_TEXTURE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D11_TEXTURE_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D12_HEAP_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::D3D12_RESOURCE_KHR;
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
impl std::ops::BitOr for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryHandleTypeFlags> for ExternalMemoryHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlags> for ExternalMemoryHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlags>>(iterator: T) -> ExternalMemoryHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalMemoryHandleTypeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn from(bit: ExternalMemoryHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBits>>(
        iterator: T,
    ) -> ExternalMemoryHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE_KMT))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_HEAP) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_HEAP))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_RESOURCE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_RESOURCE))?;
                    }
                    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::DMA_BUF_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DMA_BUF_EXT))?;
                    }
                    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ANDROID_HARDWARE_BUFFER_ANDROID))?;
                    }
                    #[cfg(feature = "VK_EXT_external_memory_host")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::HOST_ALLOCATION_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HOST_ALLOCATION_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_external_memory_host")]
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::HOST_MAPPED_FOREIGN_MEMORY_EXT)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(HOST_MAPPED_FOREIGN_MEMORY_EXT))?;
                    }
                    #[cfg(feature = "VK_FUCHSIA_external_memory")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::ZIRCON_VMO_FUCHSIA) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ZIRCON_VMO_FUCHSIA))?;
                    }
                    #[cfg(feature = "VK_NV_external_memory_rdma")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::RDMA_ADDRESS_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(RDMA_ADDRESS_NV))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_TEXTURE_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_HEAP_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_HEAP_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryHandleTypeFlags::D3D12_RESOURCE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_RESOURCE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryHandleTypeFlags {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryHandleTypeFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalMemoryFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryFeatureFlags(u32);
impl ExternalMemoryFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT")]
    pub const DEDICATED_ONLY: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
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
            all |= Self::DEDICATED_ONLY;
        }
        {
            all |= Self::EXPORTABLE;
        }
        {
            all |= Self::IMPORTABLE;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::DEDICATED_ONLY_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::EXPORTABLE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_memory_capabilities")]
        {
            all |= Self::IMPORTABLE_KHR;
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
impl std::ops::BitOr for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryFeatureFlags> for ExternalMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlags> for ExternalMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlags>>(iterator: T) -> ExternalMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalMemoryFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn from(bit: ExternalMemoryFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlagBits>>(iterator: T) -> ExternalMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalMemoryFeatureFlags::DEDICATED_ONLY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEDICATED_ONLY))?;
                    }
                    if self.0.contains(ExternalMemoryFeatureFlags::EXPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE))?;
                    }
                    if self.0.contains(ExternalMemoryFeatureFlags::IMPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryFeatureFlags::DEDICATED_ONLY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEDICATED_ONLY_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryFeatureFlags::EXPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
                    if self.0.contains(ExternalMemoryFeatureFlags::IMPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryFeatureFlags {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryFeatureFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalSemaphoreHandleTypeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalSemaphoreHandleTypeFlags(u32);
impl ExternalSemaphoreHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT")]
    pub const D3D12_FENCE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT")]
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
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
            all |= Self::OPAQUE_FD;
        }
        {
            all |= Self::OPAQUE_WIN32;
        }
        {
            all |= Self::OPAQUE_WIN32_KMT;
        }
        {
            all |= Self::D3D12_FENCE;
        }
        {
            all |= Self::SYNC_FD;
        }
        #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
        {
            all |= Self::ZIRCON_EVENT_FUCHSIA;
        }
        {
            all |= Self::D3D11_FENCE;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::OPAQUE_FD_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::D3D12_FENCE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::SYNC_FD_KHR;
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
impl std::ops::BitOr for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalSemaphoreHandleTypeFlags> for ExternalSemaphoreHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalSemaphoreHandleTypeFlags> for ExternalSemaphoreHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlags>>(
        iterator: T,
    ) -> ExternalSemaphoreHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalSemaphoreHandleTypeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn from(bit: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlagBits>>(
        iterator: T,
    ) -> ExternalSemaphoreHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalSemaphoreHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalSemaphoreHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::D3D12_FENCE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_FENCE))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::SYNC_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD))?;
                    }
                    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::ZIRCON_EVENT_FUCHSIA) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ZIRCON_EVENT_FUCHSIA))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::D3D11_FENCE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_FENCE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::D3D12_FENCE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D12_FENCE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::SYNC_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalSemaphoreHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalSemaphoreHandleTypeFlags {
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
unsafe impl crate::conv::FromLowLevel for ExternalSemaphoreHandleTypeFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalSemaphoreFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalSemaphoreFeatureFlags(u32);
impl ExternalSemaphoreFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
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
            all |= Self::EXPORTABLE;
        }
        {
            all |= Self::IMPORTABLE;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::EXPORTABLE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
        {
            all |= Self::IMPORTABLE_KHR;
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
impl std::ops::BitOr for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalSemaphoreFeatureFlags> for ExternalSemaphoreFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalSemaphoreFeatureFlags> for ExternalSemaphoreFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreFeatureFlags>>(iterator: T) -> ExternalSemaphoreFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalSemaphoreFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn from(bit: ExternalSemaphoreFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreFeatureFlagBits>>(
        iterator: T,
    ) -> ExternalSemaphoreFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalSemaphoreFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalSemaphoreFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalSemaphoreFeatureFlags::EXPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE))?;
                    }
                    if self.0.contains(ExternalSemaphoreFeatureFlags::IMPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreFeatureFlags::EXPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
                    if self.0.contains(ExternalSemaphoreFeatureFlags::IMPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalSemaphoreFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalSemaphoreFeatureFlags {
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
unsafe impl crate::conv::FromLowLevel for ExternalSemaphoreFeatureFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSemaphoreImportFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreImportFlags(u32);
impl SemaphoreImportFlags {
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
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
            all |= Self::TEMPORARY;
        }
        #[cfg(feature = "VK_KHR_external_semaphore")]
        {
            all |= Self::TEMPORARY_KHR;
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
impl std::ops::BitOr for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SemaphoreImportFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SemaphoreImportFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SemaphoreImportFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SemaphoreImportFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SemaphoreImportFlags> for SemaphoreImportFlags {
    fn extend<T: IntoIterator<Item = SemaphoreImportFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SemaphoreImportFlags> for SemaphoreImportFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreImportFlags>>(iterator: T) -> SemaphoreImportFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreImportFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for SemaphoreImportFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn from(bit: SemaphoreImportFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn extend<T: IntoIterator<Item = SemaphoreImportFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreImportFlagBits>>(iterator: T) -> SemaphoreImportFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreImportFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SemaphoreImportFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SemaphoreImportFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SemaphoreImportFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SemaphoreImportFlags::TEMPORARY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY))?;
                    }
                    #[cfg(feature = "VK_KHR_external_semaphore")]
                    if self.0.contains(SemaphoreImportFlags::TEMPORARY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SemaphoreImportFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SemaphoreImportFlags {
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
unsafe impl crate::conv::FromLowLevel for SemaphoreImportFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalFenceHandleTypeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalFenceHandleTypeFlags(u32);
impl ExternalFenceHandleTypeFlags {
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
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
            all |= Self::OPAQUE_FD;
        }
        {
            all |= Self::OPAQUE_WIN32;
        }
        {
            all |= Self::OPAQUE_WIN32_KMT;
        }
        {
            all |= Self::SYNC_FD;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::OPAQUE_FD_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::OPAQUE_WIN32_KMT_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::SYNC_FD_KHR;
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
impl std::ops::BitOr for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalFenceHandleTypeFlags> for ExternalFenceHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalFenceHandleTypeFlags> for ExternalFenceHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceHandleTypeFlags>>(iterator: T) -> ExternalFenceHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalFenceHandleTypeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn from(bit: ExternalFenceHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceHandleTypeFlagBits>>(iterator: T) -> ExternalFenceHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalFenceHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalFenceHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::SYNC_FD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_FD_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KMT_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceHandleTypeFlags::SYNC_FD_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SYNC_FD_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalFenceHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalFenceHandleTypeFlags {
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
unsafe impl crate::conv::FromLowLevel for ExternalFenceHandleTypeFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalFenceFeatureFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalFenceFeatureFlags(u32);
impl ExternalFenceFeatureFlags {
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
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
            all |= Self::EXPORTABLE;
        }
        {
            all |= Self::IMPORTABLE;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::EXPORTABLE_KHR;
        }
        #[cfg(feature = "VK_KHR_external_fence_capabilities")]
        {
            all |= Self::IMPORTABLE_KHR;
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
impl std::ops::BitOr for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalFenceFeatureFlags> for ExternalFenceFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalFenceFeatureFlags> for ExternalFenceFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceFeatureFlags>>(iterator: T) -> ExternalFenceFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalFenceFeatureFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn from(bit: ExternalFenceFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceFeatureFlagBits>>(iterator: T) -> ExternalFenceFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalFenceFeatureFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalFenceFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalFenceFeatureFlags::EXPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE))?;
                    }
                    if self.0.contains(ExternalFenceFeatureFlags::IMPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceFeatureFlags::EXPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
                    if self.0.contains(ExternalFenceFeatureFlags::IMPORTABLE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalFenceFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalFenceFeatureFlags {
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
unsafe impl crate::conv::FromLowLevel for ExternalFenceFeatureFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkFenceImportFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FenceImportFlags(u32);
impl FenceImportFlags {
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
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
            all |= Self::TEMPORARY;
        }
        #[cfg(feature = "VK_KHR_external_fence")]
        {
            all |= Self::TEMPORARY_KHR;
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
impl std::ops::BitOr for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for FenceImportFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for FenceImportFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for FenceImportFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for FenceImportFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<FenceImportFlags> for FenceImportFlags {
    fn extend<T: IntoIterator<Item = FenceImportFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<FenceImportFlags> for FenceImportFlags {
    fn from_iter<T: IntoIterator<Item = FenceImportFlags>>(iterator: T) -> FenceImportFlags {
        let mut out = Self::empty();
        <Self as Extend<FenceImportFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for FenceImportFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<FenceImportFlagBits> for FenceImportFlags {
    fn from(bit: FenceImportFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<FenceImportFlagBits> for FenceImportFlags {
    fn extend<T: IntoIterator<Item = FenceImportFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<FenceImportFlagBits> for FenceImportFlags {
    fn from_iter<T: IntoIterator<Item = FenceImportFlagBits>>(iterator: T) -> FenceImportFlags {
        let mut out = Self::empty();
        <Self as Extend<FenceImportFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for FenceImportFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(FenceImportFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == FenceImportFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(FenceImportFlags::TEMPORARY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY))?;
                    }
                    #[cfg(feature = "VK_KHR_external_fence")]
                    if self.0.contains(FenceImportFlags::TEMPORARY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TEMPORARY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(FenceImportFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FenceImportFlags {
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
unsafe impl crate::conv::FromLowLevel for FenceImportFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkDeviceQueueCreateFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DeviceQueueCreateFlagBits(u32);
impl Default for DeviceQueueCreateFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl DeviceQueueCreateFlagBits {
    #[doc(alias = "VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT")]
    pub const PROTECTED: Self = Self(1);
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
            #[cfg(feature = "VK_QCOM_extension_440")]
            x if x == Self::RESERVED1_QCOM.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for DeviceQueueCreateFlagBits {
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
unsafe impl crate::conv::FromLowLevel for DeviceQueueCreateFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSubgroupFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SubgroupFeatureFlagBits(u32);
impl Default for SubgroupFeatureFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SubgroupFeatureFlagBits {
    #[doc(alias = "VK_SUBGROUP_FEATURE_BASIC_BIT")]
    pub const BASIC: Self = Self(1);
    #[doc(alias = "VK_SUBGROUP_FEATURE_VOTE_BIT")]
    pub const VOTE: Self = Self(2);
    #[doc(alias = "VK_SUBGROUP_FEATURE_ARITHMETIC_BIT")]
    pub const ARITHMETIC: Self = Self(4);
    #[doc(alias = "VK_SUBGROUP_FEATURE_BALLOT_BIT")]
    pub const BALLOT: Self = Self(8);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_BIT")]
    pub const SHUFFLE: Self = Self(16);
    #[doc(alias = "VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT")]
    pub const SHUFFLE_RELATIVE: Self = Self(32);
    #[doc(alias = "VK_SUBGROUP_FEATURE_CLUSTERED_BIT")]
    pub const CLUSTERED: Self = Self(64);
    #[doc(alias = "VK_SUBGROUP_FEATURE_QUAD_BIT")]
    pub const QUAD: Self = Self(128);
    #[doc(alias = "VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV")]
    #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
    pub const PARTITIONED_NV: Self = Self(256);
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
            x if x == Self::BASIC.bits() => Some(Self(x)),
            x if x == Self::VOTE.bits() => Some(Self(x)),
            x if x == Self::ARITHMETIC.bits() => Some(Self(x)),
            x if x == Self::BALLOT.bits() => Some(Self(x)),
            x if x == Self::SHUFFLE.bits() => Some(Self(x)),
            x if x == Self::SHUFFLE_RELATIVE.bits() => Some(Self(x)),
            x if x == Self::CLUSTERED.bits() => Some(Self(x)),
            x if x == Self::QUAD.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
            x if x == Self::PARTITIONED_NV.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for SubgroupFeatureFlagBits {
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
unsafe impl crate::conv::FromLowLevel for SubgroupFeatureFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalMemoryHandleTypeFlagBits(u32);
impl Default for ExternalMemoryHandleTypeFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalMemoryHandleTypeFlagBits {
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT")]
    pub const D3D11_TEXTURE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT")]
    pub const D3D11_TEXTURE_KMT: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT")]
    pub const D3D12_HEAP: Self = Self(32);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT")]
    pub const D3D12_RESOURCE: Self = Self(64);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
    pub const DMA_BUF_EXT: Self = Self(512);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID")]
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1024);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_ALLOCATION_EXT: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT")]
    #[cfg(feature = "VK_EXT_external_memory_host")]
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(2048);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV")]
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    pub const RDMA_ADDRESS_NV: Self = Self(4096);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
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
            x if x == Self::OPAQUE_FD.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32_KMT.bits() => Some(Self(x)),
            x if x == Self::D3D11_TEXTURE.bits() => Some(Self(x)),
            x if x == Self::D3D11_TEXTURE_KMT.bits() => Some(Self(x)),
            x if x == Self::D3D12_HEAP.bits() => Some(Self(x)),
            x if x == Self::D3D12_RESOURCE.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_external_memory_dma_buf")]
            x if x == Self::DMA_BUF_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
            x if x == Self::ANDROID_HARDWARE_BUFFER_ANDROID.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_external_memory_host")]
            x if x == Self::HOST_ALLOCATION_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_EXT_external_memory_host")]
            x if x == Self::HOST_MAPPED_FOREIGN_MEMORY_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_FUCHSIA_external_memory")]
            x if x == Self::ZIRCON_VMO_FUCHSIA.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_external_memory_rdma")]
            x if x == Self::RDMA_ADDRESS_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_375")]
            x if x == Self::RESERVED13_NV.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryHandleTypeFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryHandleTypeFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalMemoryFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalMemoryFeatureFlagBits(u32);
impl Default for ExternalMemoryFeatureFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalMemoryFeatureFlagBits {
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT")]
    pub const DEDICATED_ONLY: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
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
            x if x == Self::DEDICATED_ONLY.bits() => Some(Self(x)),
            x if x == Self::EXPORTABLE.bits() => Some(Self(x)),
            x if x == Self::IMPORTABLE.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryFeatureFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryFeatureFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalSemaphoreHandleTypeFlagBits(u32);
impl Default for ExternalSemaphoreHandleTypeFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalSemaphoreHandleTypeFlagBits {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT")]
    pub const D3D12_FENCE: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(16);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA")]
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(128);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT")]
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
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
            x if x == Self::OPAQUE_FD.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32_KMT.bits() => Some(Self(x)),
            x if x == Self::D3D12_FENCE.bits() => Some(Self(x)),
            x if x == Self::SYNC_FD.bits() => Some(Self(x)),
            #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
            x if x == Self::ZIRCON_EVENT_FUCHSIA.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED5_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED6_NV.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ExternalSemaphoreHandleTypeFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ExternalSemaphoreHandleTypeFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalSemaphoreFeatureFlagBits(u32);
impl Default for ExternalSemaphoreFeatureFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalSemaphoreFeatureFlagBits {
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
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
            x if x == Self::EXPORTABLE.bits() => Some(Self(x)),
            x if x == Self::IMPORTABLE.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ExternalSemaphoreFeatureFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ExternalSemaphoreFeatureFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSemaphoreImportFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SemaphoreImportFlagBits(u32);
impl Default for SemaphoreImportFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SemaphoreImportFlagBits {
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_semaphore")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
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
            x if x == Self::TEMPORARY.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for SemaphoreImportFlagBits {
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
unsafe impl crate::conv::FromLowLevel for SemaphoreImportFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalFenceHandleTypeFlagBits(u32);
impl Default for ExternalFenceHandleTypeFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalFenceHandleTypeFlagBits {
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT")]
    pub const OPAQUE_FD: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT")]
    pub const OPAQUE_WIN32: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT")]
    pub const OPAQUE_WIN32_KMT: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT")]
    pub const SYNC_FD: Self = Self(8);
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    #[doc(alias = "VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
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
            x if x == Self::OPAQUE_FD.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32_KMT.bits() => Some(Self(x)),
            x if x == Self::SYNC_FD.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED4_NV.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_extension_374")]
            x if x == Self::RESERVED5_NV.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ExternalFenceHandleTypeFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ExternalFenceHandleTypeFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalFenceFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalFenceFeatureFlagBits(u32);
impl Default for ExternalFenceFeatureFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalFenceFeatureFlagBits {
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT")]
    pub const EXPORTABLE: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT")]
    pub const IMPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    #[doc(alias = "VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
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
            x if x == Self::EXPORTABLE.bits() => Some(Self(x)),
            x if x == Self::IMPORTABLE.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ExternalFenceFeatureFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ExternalFenceFeatureFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkFenceImportFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FenceImportFlagBits(u32);
impl Default for FenceImportFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl FenceImportFlagBits {
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT")]
    pub const TEMPORARY: Self = Self(1);
    #[doc(alias = "VK_FENCE_IMPORT_TEMPORARY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_external_fence")]
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
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
            x if x == Self::TEMPORARY.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for FenceImportFlagBits {
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
unsafe impl crate::conv::FromLowLevel for FenceImportFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPeerMemoryFeatureFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PeerMemoryFeatureFlagBits(u32);
impl Default for PeerMemoryFeatureFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PeerMemoryFeatureFlagBits {
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT")]
    pub const COPY_SRC: Self = Self(1);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT")]
    pub const COPY_DST: Self = Self(2);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT")]
    pub const GENERIC_SRC: Self = Self(4);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT")]
    pub const GENERIC_DST: Self = Self(8);
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    #[doc(alias = "VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
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
            x if x == Self::COPY_SRC.bits() => Some(Self(x)),
            x if x == Self::COPY_DST.bits() => Some(Self(x)),
            x if x == Self::GENERIC_SRC.bits() => Some(Self(x)),
            x if x == Self::GENERIC_DST.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for PeerMemoryFeatureFlagBits {
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
unsafe impl crate::conv::FromLowLevel for PeerMemoryFeatureFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkMemoryAllocateFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct MemoryAllocateFlagBits(u32);
impl Default for MemoryAllocateFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl MemoryAllocateFlagBits {
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT")]
    pub const DEVICE_MASK: Self = Self(1);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS: Self = Self(2);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT")]
    #[cfg(feature = "VULKAN_1_2")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4);
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    #[doc(alias = "VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
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
            x if x == Self::DEVICE_MASK.bits() => Some(Self(x)),
            #[cfg(feature = "VULKAN_1_2")]
            x if x == Self::DEVICE_ADDRESS.bits() => Some(Self(x)),
            #[cfg(feature = "VULKAN_1_2")]
            x if x == Self::DEVICE_ADDRESS_CAPTURE_REPLAY.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for MemoryAllocateFlagBits {
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
unsafe impl crate::conv::FromLowLevel for MemoryAllocateFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkDescriptorUpdateTemplateType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum DescriptorUpdateTemplateType {
    #[doc(alias = "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET")]
    #[doc(alias = "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR")]
    DescriptorSet = 0,
    #[doc(alias = "VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR")]
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    PushDescriptorsKhr = 1,
}
impl Default for DescriptorUpdateTemplateType {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl DescriptorUpdateTemplateType {
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
            x if x == Self::DescriptorSet.bits() => Some(Self::DescriptorSet),
            #[cfg(feature = "VK_KHR_descriptor_update_template")]
            x if x == Self::PushDescriptorsKhr.bits() => Some(Self::PushDescriptorsKhr),
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
unsafe impl crate::conv::IntoLowLevel for DescriptorUpdateTemplateType {
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
unsafe impl crate::conv::FromLowLevel for DescriptorUpdateTemplateType {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPointClippingBehavior")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum PointClippingBehavior {
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES")]
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR")]
    AllClipPlanes = 0,
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY")]
    #[doc(alias = "VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR")]
    UserClipPlanesOnly = 1,
}
impl Default for PointClippingBehavior {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PointClippingBehavior {
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
            x if x == Self::AllClipPlanes.bits() => Some(Self::AllClipPlanes),
            x if x == Self::UserClipPlanesOnly.bits() => Some(Self::UserClipPlanesOnly),
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
unsafe impl crate::conv::IntoLowLevel for PointClippingBehavior {
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
unsafe impl crate::conv::FromLowLevel for PointClippingBehavior {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkTessellationDomainOrigin")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum TessellationDomainOrigin {
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT")]
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR")]
    UpperLeft = 0,
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT")]
    #[doc(alias = "VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR")]
    LowerLeft = 1,
}
impl Default for TessellationDomainOrigin {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl TessellationDomainOrigin {
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
            x if x == Self::UpperLeft.bits() => Some(Self::UpperLeft),
            x if x == Self::LowerLeft.bits() => Some(Self::LowerLeft),
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
unsafe impl crate::conv::IntoLowLevel for TessellationDomainOrigin {
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
unsafe impl crate::conv::FromLowLevel for TessellationDomainOrigin {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSamplerYcbcrModelConversion")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum SamplerYcbcrModelConversion {
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY")]
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR")]
    RgbIdentity = 0,
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY")]
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR")]
    YcbcrIdentity = 1,
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709")]
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR")]
    Ycbcr709 = 2,
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601")]
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR")]
    Ycbcr601 = 3,
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020")]
    #[doc(alias = "VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR")]
    Ycbcr2020 = 4,
}
impl Default for SamplerYcbcrModelConversion {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SamplerYcbcrModelConversion {
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
            x if x == Self::RgbIdentity.bits() => Some(Self::RgbIdentity),
            x if x == Self::YcbcrIdentity.bits() => Some(Self::YcbcrIdentity),
            x if x == Self::Ycbcr709.bits() => Some(Self::Ycbcr709),
            x if x == Self::Ycbcr601.bits() => Some(Self::Ycbcr601),
            x if x == Self::Ycbcr2020.bits() => Some(Self::Ycbcr2020),
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
unsafe impl crate::conv::IntoLowLevel for SamplerYcbcrModelConversion {
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
unsafe impl crate::conv::FromLowLevel for SamplerYcbcrModelConversion {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSamplerYcbcrRange")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum SamplerYcbcrRange {
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_FULL")]
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR")]
    ItuFull = 0,
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_NARROW")]
    #[doc(alias = "VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR")]
    ItuNarrow = 1,
}
impl Default for SamplerYcbcrRange {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SamplerYcbcrRange {
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
            x if x == Self::ItuFull.bits() => Some(Self::ItuFull),
            x if x == Self::ItuNarrow.bits() => Some(Self::ItuNarrow),
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
unsafe impl crate::conv::IntoLowLevel for SamplerYcbcrRange {
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
unsafe impl crate::conv::FromLowLevel for SamplerYcbcrRange {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkChromaLocation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ChromaLocation {
    #[doc(alias = "VK_CHROMA_LOCATION_COSITED_EVEN")]
    #[doc(alias = "VK_CHROMA_LOCATION_COSITED_EVEN_KHR")]
    CositedEven = 0,
    #[doc(alias = "VK_CHROMA_LOCATION_MIDPOINT")]
    #[doc(alias = "VK_CHROMA_LOCATION_MIDPOINT_KHR")]
    Midpoint = 1,
}
impl Default for ChromaLocation {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ChromaLocation {
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
            x if x == Self::CositedEven.bits() => Some(Self::CositedEven),
            x if x == Self::Midpoint.bits() => Some(Self::Midpoint),
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
unsafe impl crate::conv::IntoLowLevel for ChromaLocation {
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
unsafe impl crate::conv::FromLowLevel for ChromaLocation {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
