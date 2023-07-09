use crate::{
    common::vulkan1_0::{DeviceAddress, IndexType},
    cstr,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkBindShaderGroupIndirectCommandNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
    #[doc(alias = "groupIndex")]
    pub group_index: u32,
}
#[doc(alias = "VkBindIndexBufferIndirectCommandNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
    #[doc(alias = "bufferAddress")]
    pub buffer_address: DeviceAddress,
    pub size: u32,
    #[doc(alias = "indexType")]
    pub index_type: IndexType,
}
#[doc(alias = "VkBindVertexBufferIndirectCommandNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
    #[doc(alias = "bufferAddress")]
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}
#[doc(alias = "VkSetStateFlagsIndirectCommandNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
    pub data: u32,
}
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagsNV")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndirectCommandsLayoutUsageFlagsNV(u32);
impl IndirectCommandsLayoutUsageFlagsNV {
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV")]
    pub const EXPLICIT_PREPROCESS: Self = Self(1);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV")]
    pub const INDEXED_SEQUENCES: Self = Self(2);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV")]
    pub const UNORDERED_SEQUENCES: Self = Self(4);
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
            all |= Self::EXPLICIT_PREPROCESS;
        }
        {
            all |= Self::INDEXED_SEQUENCES;
        }
        {
            all |= Self::UNORDERED_SEQUENCES;
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
impl std::ops::BitOr for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<IndirectCommandsLayoutUsageFlagsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<IndirectCommandsLayoutUsageFlagsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagsNV>>(
        iterator: T,
    ) -> IndirectCommandsLayoutUsageFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectCommandsLayoutUsageFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl Default for IndirectCommandsLayoutUsageFlagsNV {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from(bit: IndirectCommandsLayoutUsageFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagBitsNV>>(
        iterator: T,
    ) -> IndirectCommandsLayoutUsageFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectCommandsLayoutUsageFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutUsageFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(IndirectCommandsLayoutUsageFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == IndirectCommandsLayoutUsageFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::EXPLICIT_PREPROCESS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPLICIT_PREPROCESS))?;
                    }
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::INDEXED_SEQUENCES) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INDEXED_SEQUENCES))?;
                    }
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::UNORDERED_SEQUENCES) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UNORDERED_SEQUENCES))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(IndirectCommandsLayoutUsageFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for IndirectCommandsLayoutUsageFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for IndirectCommandsLayoutUsageFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkIndirectStateFlagsNV")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IndirectStateFlagsNV(u32);
impl IndirectStateFlagsNV {
    #[doc(alias = "VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV")]
    pub const FLAG_FRONTFACE: Self = Self(1);
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
            all |= Self::FLAG_FRONTFACE;
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
impl std::ops::BitOr for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for IndirectStateFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<IndirectStateFlagsNV> for IndirectStateFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectStateFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<IndirectStateFlagsNV> for IndirectStateFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectStateFlagsNV>>(iterator: T) -> IndirectStateFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectStateFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl Default for IndirectStateFlagsNV {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn from(bit: IndirectStateFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectStateFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectStateFlagBitsNV>>(iterator: T) -> IndirectStateFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectStateFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for IndirectStateFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(IndirectStateFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == IndirectStateFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(IndirectStateFlagsNV::FLAG_FRONTFACE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FLAG_FRONTFACE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(IndirectStateFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for IndirectStateFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for IndirectStateFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_device_generated_commands");
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(u32);
impl Default for IndirectCommandsLayoutUsageFlagBitsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl IndirectCommandsLayoutUsageFlagBitsNV {
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV")]
    pub const EXPLICIT_PREPROCESS: Self = Self(1);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV")]
    pub const INDEXED_SEQUENCES: Self = Self(2);
    #[doc(alias = "VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV")]
    pub const UNORDERED_SEQUENCES: Self = Self(4);
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
            x if x == Self::EXPLICIT_PREPROCESS.bits() => Some(Self(x)),
            x if x == Self::INDEXED_SEQUENCES.bits() => Some(Self(x)),
            x if x == Self::UNORDERED_SEQUENCES.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for IndirectCommandsLayoutUsageFlagBitsNV {
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
unsafe impl crate::conv::FromLowLevel for IndirectCommandsLayoutUsageFlagBitsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkIndirectStateFlagBitsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct IndirectStateFlagBitsNV(u32);
impl Default for IndirectStateFlagBitsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl IndirectStateFlagBitsNV {
    #[doc(alias = "VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV")]
    pub const FLAG_FRONTFACE: Self = Self(1);
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
            x if x == Self::FLAG_FRONTFACE.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for IndirectStateFlagBitsNV {
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
unsafe impl crate::conv::FromLowLevel for IndirectStateFlagBitsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkIndirectCommandsTokenTypeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum IndirectCommandsTokenTypeNV {
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV")]
    ShaderGroup = 0,
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV")]
    StateFlags = 1,
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV")]
    IndexBuffer = 2,
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV")]
    VertexBuffer = 3,
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV")]
    PushConstant = 4,
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV")]
    DrawIndexed = 5,
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV")]
    Draw = 6,
    #[doc(alias = "VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV")]
    DrawTasks = 7,
}
impl Default for IndirectCommandsTokenTypeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl IndirectCommandsTokenTypeNV {
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
            x if x == Self::ShaderGroup.bits() => Some(Self::ShaderGroup),
            x if x == Self::StateFlags.bits() => Some(Self::StateFlags),
            x if x == Self::IndexBuffer.bits() => Some(Self::IndexBuffer),
            x if x == Self::VertexBuffer.bits() => Some(Self::VertexBuffer),
            x if x == Self::PushConstant.bits() => Some(Self::PushConstant),
            x if x == Self::DrawIndexed.bits() => Some(Self::DrawIndexed),
            x if x == Self::Draw.bits() => Some(Self::Draw),
            x if x == Self::DrawTasks.bits() => Some(Self::DrawTasks),
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
unsafe impl crate::conv::IntoLowLevel for IndirectCommandsTokenTypeNV {
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
unsafe impl crate::conv::FromLowLevel for IndirectCommandsTokenTypeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
