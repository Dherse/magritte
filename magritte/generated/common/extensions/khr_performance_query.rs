use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkPerformanceCounterDescriptionFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceCounterDescriptionFlagsKHR(u32);
impl PerformanceCounterDescriptionFlagsKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR")]
    pub const PERFORMANCE_IMPACTING: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR")]
    pub const CONCURRENTLY_IMPACTED: Self = Self(2);
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
            all |= Self::PERFORMANCE_IMPACTING;
        }
        {
            all |= Self::CONCURRENTLY_IMPACTED;
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
impl std::ops::BitOr for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PerformanceCounterDescriptionFlagsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn extend<T: IntoIterator<Item = PerformanceCounterDescriptionFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<PerformanceCounterDescriptionFlagsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from_iter<T: IntoIterator<Item = PerformanceCounterDescriptionFlagsKHR>>(
        iterator: T,
    ) -> PerformanceCounterDescriptionFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<PerformanceCounterDescriptionFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for PerformanceCounterDescriptionFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from(bit: PerformanceCounterDescriptionFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn extend<T: IntoIterator<Item = PerformanceCounterDescriptionFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from_iter<T: IntoIterator<Item = PerformanceCounterDescriptionFlagBitsKHR>>(
        iterator: T,
    ) -> PerformanceCounterDescriptionFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<PerformanceCounterDescriptionFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PerformanceCounterDescriptionFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PerformanceCounterDescriptionFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PerformanceCounterDescriptionFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(PerformanceCounterDescriptionFlagsKHR::PERFORMANCE_IMPACTING)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PERFORMANCE_IMPACTING))?;
                    }
                    if self
                        .0
                        .contains(PerformanceCounterDescriptionFlagsKHR::CONCURRENTLY_IMPACTED)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CONCURRENTLY_IMPACTED))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PerformanceCounterDescriptionFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterDescriptionFlagsKHR {
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
unsafe impl crate::conv::FromLowLevel for PerformanceCounterDescriptionFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAcquireProfilingLockFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcquireProfilingLockFlagsKHR(u32);
impl AcquireProfilingLockFlagsKHR {
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
impl std::ops::BitOr for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<AcquireProfilingLockFlagsKHR> for AcquireProfilingLockFlagsKHR {
    fn extend<T: IntoIterator<Item = AcquireProfilingLockFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<AcquireProfilingLockFlagsKHR> for AcquireProfilingLockFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AcquireProfilingLockFlagsKHR>>(iterator: T) -> AcquireProfilingLockFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AcquireProfilingLockFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for AcquireProfilingLockFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn from(bit: AcquireProfilingLockFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn extend<T: IntoIterator<Item = AcquireProfilingLockFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AcquireProfilingLockFlagBitsKHR>>(iterator: T) -> AcquireProfilingLockFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AcquireProfilingLockFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for AcquireProfilingLockFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(AcquireProfilingLockFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == AcquireProfilingLockFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(AcquireProfilingLockFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AcquireProfilingLockFlagsKHR {
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
unsafe impl crate::conv::FromLowLevel for AcquireProfilingLockFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_performance_query");
#[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceCounterDescriptionFlagBitsKHR(u32);
impl Default for PerformanceCounterDescriptionFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PerformanceCounterDescriptionFlagBitsKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR")]
    pub const PERFORMANCE_IMPACTING: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR")]
    pub const CONCURRENTLY_IMPACTED: Self = Self(2);
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
            x if x == Self::PERFORMANCE_IMPACTING.bits() => Some(Self(x)),
            x if x == Self::CONCURRENTLY_IMPACTED.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterDescriptionFlagBitsKHR {
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
unsafe impl crate::conv::FromLowLevel for PerformanceCounterDescriptionFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AcquireProfilingLockFlagBitsKHR(u32);
impl Default for AcquireProfilingLockFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AcquireProfilingLockFlagBitsKHR {
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
unsafe impl crate::conv::IntoLowLevel for AcquireProfilingLockFlagBitsKHR {
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
unsafe impl crate::conv::FromLowLevel for AcquireProfilingLockFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPerformanceCounterScopeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum PerformanceCounterScopeKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR")]
    #[doc(alias = "VK_QUERY_SCOPE_COMMAND_BUFFER_KHR")]
    CommandBuffer = 0,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR")]
    #[doc(alias = "VK_QUERY_SCOPE_RENDER_PASS_KHR")]
    RenderPass = 1,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR")]
    #[doc(alias = "VK_QUERY_SCOPE_COMMAND_KHR")]
    Command = 2,
}
impl Default for PerformanceCounterScopeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PerformanceCounterScopeKHR {
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
            x if x == Self::CommandBuffer.bits() => Some(Self::CommandBuffer),
            x if x == Self::RenderPass.bits() => Some(Self::RenderPass),
            x if x == Self::Command.bits() => Some(Self::Command),
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
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterScopeKHR {
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
unsafe impl crate::conv::FromLowLevel for PerformanceCounterScopeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPerformanceCounterUnitKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum PerformanceCounterUnitKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR")]
    Generic = 0,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR")]
    Percentage = 1,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR")]
    Nanoseconds = 2,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR")]
    Bytes = 3,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR")]
    BytesPerSecond = 4,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR")]
    Kelvin = 5,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR")]
    Watts = 6,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR")]
    Volts = 7,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR")]
    Amps = 8,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR")]
    Hertz = 9,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR")]
    Cycles = 10,
}
impl Default for PerformanceCounterUnitKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PerformanceCounterUnitKHR {
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
            x if x == Self::Generic.bits() => Some(Self::Generic),
            x if x == Self::Percentage.bits() => Some(Self::Percentage),
            x if x == Self::Nanoseconds.bits() => Some(Self::Nanoseconds),
            x if x == Self::Bytes.bits() => Some(Self::Bytes),
            x if x == Self::BytesPerSecond.bits() => Some(Self::BytesPerSecond),
            x if x == Self::Kelvin.bits() => Some(Self::Kelvin),
            x if x == Self::Watts.bits() => Some(Self::Watts),
            x if x == Self::Volts.bits() => Some(Self::Volts),
            x if x == Self::Amps.bits() => Some(Self::Amps),
            x if x == Self::Hertz.bits() => Some(Self::Hertz),
            x if x == Self::Cycles.bits() => Some(Self::Cycles),
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
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterUnitKHR {
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
unsafe impl crate::conv::FromLowLevel for PerformanceCounterUnitKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkPerformanceCounterStorageKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum PerformanceCounterStorageKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR")]
    Int32 = 0,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR")]
    Int64 = 1,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR")]
    Uint32 = 2,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR")]
    Uint64 = 3,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR")]
    Float32 = 4,
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR")]
    Float64 = 5,
}
impl Default for PerformanceCounterStorageKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PerformanceCounterStorageKHR {
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
            x if x == Self::Int32.bits() => Some(Self::Int32),
            x if x == Self::Int64.bits() => Some(Self::Int64),
            x if x == Self::Uint32.bits() => Some(Self::Uint32),
            x if x == Self::Uint64.bits() => Some(Self::Uint64),
            x if x == Self::Float32.bits() => Some(Self::Float32),
            x if x == Self::Float64.bits() => Some(Self::Float64),
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
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterStorageKHR {
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
unsafe impl crate::conv::FromLowLevel for PerformanceCounterStorageKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
