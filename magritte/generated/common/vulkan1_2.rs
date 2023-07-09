#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[doc(alias = "VkConformanceVersion")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}
#[doc(alias = "VkSemaphoreWaitFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemaphoreWaitFlags(u32);
impl SemaphoreWaitFlags {
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT")]
    pub const ANY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const ANY_KHR: Self = Self::ANY;
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
            all |= Self::ANY;
        }
        #[cfg(feature = "VK_KHR_timeline_semaphore")]
        {
            all |= Self::ANY_KHR;
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
impl std::ops::BitOr for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SemaphoreWaitFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SemaphoreWaitFlags> for SemaphoreWaitFlags {
    fn extend<T: IntoIterator<Item = SemaphoreWaitFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SemaphoreWaitFlags> for SemaphoreWaitFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreWaitFlags>>(iterator: T) -> SemaphoreWaitFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreWaitFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for SemaphoreWaitFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn from(bit: SemaphoreWaitFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn extend<T: IntoIterator<Item = SemaphoreWaitFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreWaitFlagBits>>(iterator: T) -> SemaphoreWaitFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreWaitFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SemaphoreWaitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SemaphoreWaitFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SemaphoreWaitFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SemaphoreWaitFlags::ANY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ANY))?;
                    }
                    #[cfg(feature = "VK_KHR_timeline_semaphore")]
                    if self.0.contains(SemaphoreWaitFlags::ANY_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ANY_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SemaphoreWaitFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SemaphoreWaitFlags {
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
unsafe impl crate::conv::FromLowLevel for SemaphoreWaitFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkDescriptorBindingFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DescriptorBindingFlags(u32);
impl DescriptorBindingFlags {
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT")]
    pub const UPDATE_AFTER_BIND: Self = Self(1);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT")]
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(2);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT")]
    pub const PARTIALLY_BOUND: Self = Self(4);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT")]
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(8);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
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
            all |= Self::UPDATE_AFTER_BIND;
        }
        {
            all |= Self::UPDATE_UNUSED_WHILE_PENDING;
        }
        {
            all |= Self::PARTIALLY_BOUND;
        }
        {
            all |= Self::VARIABLE_DESCRIPTOR_COUNT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::UPDATE_AFTER_BIND_EXT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::UPDATE_UNUSED_WHILE_PENDING_EXT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::PARTIALLY_BOUND_EXT;
        }
        #[cfg(feature = "VK_EXT_descriptor_indexing")]
        {
            all |= Self::VARIABLE_DESCRIPTOR_COUNT_EXT;
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
impl std::ops::BitOr for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for DescriptorBindingFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for DescriptorBindingFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for DescriptorBindingFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for DescriptorBindingFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DescriptorBindingFlags> for DescriptorBindingFlags {
    fn extend<T: IntoIterator<Item = DescriptorBindingFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DescriptorBindingFlags> for DescriptorBindingFlags {
    fn from_iter<T: IntoIterator<Item = DescriptorBindingFlags>>(iterator: T) -> DescriptorBindingFlags {
        let mut out = Self::empty();
        <Self as Extend<DescriptorBindingFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for DescriptorBindingFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn from(bit: DescriptorBindingFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn extend<T: IntoIterator<Item = DescriptorBindingFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn from_iter<T: IntoIterator<Item = DescriptorBindingFlagBits>>(iterator: T) -> DescriptorBindingFlags {
        let mut out = Self::empty();
        <Self as Extend<DescriptorBindingFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DescriptorBindingFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DescriptorBindingFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DescriptorBindingFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(DescriptorBindingFlags::UPDATE_AFTER_BIND) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_AFTER_BIND))?;
                    }
                    if self.0.contains(DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_UNUSED_WHILE_PENDING))?;
                    }
                    if self.0.contains(DescriptorBindingFlags::PARTIALLY_BOUND) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PARTIALLY_BOUND))?;
                    }
                    if self.0.contains(DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VARIABLE_DESCRIPTOR_COUNT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::UPDATE_AFTER_BIND_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_AFTER_BIND_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(UPDATE_UNUSED_WHILE_PENDING_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::PARTIALLY_BOUND_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PARTIALLY_BOUND_EXT))?;
                    }
                    #[cfg(feature = "VK_EXT_descriptor_indexing")]
                    if self.0.contains(DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT_EXT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(VARIABLE_DESCRIPTOR_COUNT_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DescriptorBindingFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DescriptorBindingFlags {
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
unsafe impl crate::conv::FromLowLevel for DescriptorBindingFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkResolveModeFlags")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolveModeFlags(u32);
impl ResolveModeFlags {
    #[doc(alias = "VK_RESOLVE_MODE_NONE")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT")]
    pub const SAMPLE_ZERO: Self = Self(1);
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT")]
    pub const AVERAGE: Self = Self(2);
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT")]
    pub const MIN: Self = Self(4);
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT")]
    pub const MAX: Self = Self(8);
    #[doc(alias = "VK_RESOLVE_MODE_NONE_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MIN_KHR: Self = Self::MIN;
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MAX_KHR: Self = Self::MAX;
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
            all |= Self::SAMPLE_ZERO;
        }
        {
            all |= Self::AVERAGE;
        }
        {
            all |= Self::MIN;
        }
        {
            all |= Self::MAX;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::NONE_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::SAMPLE_ZERO_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::AVERAGE_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::MIN_KHR;
        }
        #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
        {
            all |= Self::MAX_KHR;
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
impl std::ops::BitOr for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ResolveModeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ResolveModeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ResolveModeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ResolveModeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ResolveModeFlags> for ResolveModeFlags {
    fn extend<T: IntoIterator<Item = ResolveModeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ResolveModeFlags> for ResolveModeFlags {
    fn from_iter<T: IntoIterator<Item = ResolveModeFlags>>(iterator: T) -> ResolveModeFlags {
        let mut out = Self::empty();
        <Self as Extend<ResolveModeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ResolveModeFlags {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ResolveModeFlagBits> for ResolveModeFlags {
    fn from(bit: ResolveModeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ResolveModeFlagBits> for ResolveModeFlags {
    fn extend<T: IntoIterator<Item = ResolveModeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ResolveModeFlagBits> for ResolveModeFlags {
    fn from_iter<T: IntoIterator<Item = ResolveModeFlagBits>>(iterator: T) -> ResolveModeFlags {
        let mut out = Self::empty();
        <Self as Extend<ResolveModeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ResolveModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ResolveModeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ResolveModeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ResolveModeFlags::NONE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NONE))?;
                    }
                    if self.0.contains(ResolveModeFlags::SAMPLE_ZERO) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SAMPLE_ZERO))?;
                    }
                    if self.0.contains(ResolveModeFlags::AVERAGE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(AVERAGE))?;
                    }
                    if self.0.contains(ResolveModeFlags::MIN) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MIN))?;
                    }
                    if self.0.contains(ResolveModeFlags::MAX) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MAX))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::NONE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NONE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::SAMPLE_ZERO_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SAMPLE_ZERO_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::AVERAGE_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(AVERAGE_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::MIN_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MIN_KHR))?;
                    }
                    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
                    if self.0.contains(ResolveModeFlags::MAX_KHR) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MAX_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ResolveModeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ResolveModeFlags {
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
unsafe impl crate::conv::FromLowLevel for ResolveModeFlags {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSemaphoreWaitFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SemaphoreWaitFlagBits(u32);
impl Default for SemaphoreWaitFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SemaphoreWaitFlagBits {
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT")]
    pub const ANY: Self = Self(1);
    #[doc(alias = "VK_SEMAPHORE_WAIT_ANY_BIT_KHR")]
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub const ANY_KHR: Self = Self::ANY;
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
            x if x == Self::ANY.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for SemaphoreWaitFlagBits {
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
unsafe impl crate::conv::FromLowLevel for SemaphoreWaitFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkDescriptorBindingFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DescriptorBindingFlagBits(u32);
impl Default for DescriptorBindingFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl DescriptorBindingFlagBits {
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT")]
    pub const UPDATE_AFTER_BIND: Self = Self(1);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT")]
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(2);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT")]
    pub const PARTIALLY_BOUND: Self = Self(4);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT")]
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(8);
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    #[doc(alias = "VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT")]
    #[cfg(feature = "VK_EXT_descriptor_indexing")]
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
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
            x if x == Self::UPDATE_AFTER_BIND.bits() => Some(Self(x)),
            x if x == Self::UPDATE_UNUSED_WHILE_PENDING.bits() => Some(Self(x)),
            x if x == Self::PARTIALLY_BOUND.bits() => Some(Self(x)),
            x if x == Self::VARIABLE_DESCRIPTOR_COUNT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_QCOM_extension_369")]
            x if x == Self::RESERVED4_QCOM.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for DescriptorBindingFlagBits {
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
unsafe impl crate::conv::FromLowLevel for DescriptorBindingFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkResolveModeFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ResolveModeFlagBits(u32);
impl Default for ResolveModeFlagBits {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ResolveModeFlagBits {
    #[doc(alias = "VK_RESOLVE_MODE_NONE")]
    pub const NONE: Self = Self(0);
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT")]
    pub const SAMPLE_ZERO: Self = Self(1);
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT")]
    pub const AVERAGE: Self = Self(2);
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT")]
    pub const MIN: Self = Self(4);
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT")]
    pub const MAX: Self = Self(8);
    #[doc(alias = "VK_RESOLVE_MODE_NONE_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const NONE_KHR: Self = Self::NONE;
    #[doc(alias = "VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    #[doc(alias = "VK_RESOLVE_MODE_AVERAGE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    #[doc(alias = "VK_RESOLVE_MODE_MIN_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MIN_KHR: Self = Self::MIN;
    #[doc(alias = "VK_RESOLVE_MODE_MAX_BIT_KHR")]
    #[cfg(feature = "VK_KHR_depth_stencil_resolve")]
    pub const MAX_KHR: Self = Self::MAX;
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
            x if x == Self::SAMPLE_ZERO.bits() => Some(Self(x)),
            x if x == Self::AVERAGE.bits() => Some(Self(x)),
            x if x == Self::MIN.bits() => Some(Self(x)),
            x if x == Self::MAX.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ResolveModeFlagBits {
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
unsafe impl crate::conv::FromLowLevel for ResolveModeFlagBits {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSemaphoreType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum SemaphoreType {
    #[doc(alias = "VK_SEMAPHORE_TYPE_BINARY")]
    #[doc(alias = "VK_SEMAPHORE_TYPE_BINARY_KHR")]
    Binary = 0,
    #[doc(alias = "VK_SEMAPHORE_TYPE_TIMELINE")]
    #[doc(alias = "VK_SEMAPHORE_TYPE_TIMELINE_KHR")]
    Timeline = 1,
}
impl Default for SemaphoreType {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SemaphoreType {
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
            x if x == Self::Binary.bits() => Some(Self::Binary),
            x if x == Self::Timeline.bits() => Some(Self::Timeline),
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
unsafe impl crate::conv::IntoLowLevel for SemaphoreType {
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
unsafe impl crate::conv::FromLowLevel for SemaphoreType {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkSamplerReductionMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum SamplerReductionMode {
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE")]
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT")]
    WeightedAverage = 0,
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MIN")]
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MIN_EXT")]
    Min = 1,
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MAX")]
    #[doc(alias = "VK_SAMPLER_REDUCTION_MODE_MAX_EXT")]
    Max = 2,
}
impl Default for SamplerReductionMode {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl SamplerReductionMode {
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
            x if x == Self::WeightedAverage.bits() => Some(Self::WeightedAverage),
            x if x == Self::Min.bits() => Some(Self::Min),
            x if x == Self::Max.bits() => Some(Self::Max),
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
unsafe impl crate::conv::IntoLowLevel for SamplerReductionMode {
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
unsafe impl crate::conv::FromLowLevel for SamplerReductionMode {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkDriverId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum DriverId {
    Empty = 0,
    #[doc(alias = "VK_DRIVER_ID_AMD_PROPRIETARY")]
    #[doc(alias = "VK_DRIVER_ID_AMD_PROPRIETARY_KHR")]
    AmdProprietary = 1,
    #[doc(alias = "VK_DRIVER_ID_AMD_OPEN_SOURCE")]
    #[doc(alias = "VK_DRIVER_ID_AMD_OPEN_SOURCE_KHR")]
    AmdOpenSource = 2,
    #[doc(alias = "VK_DRIVER_ID_MESA_RADV")]
    #[doc(alias = "VK_DRIVER_ID_MESA_RADV_KHR")]
    MesaRadv = 3,
    #[doc(alias = "VK_DRIVER_ID_NVIDIA_PROPRIETARY")]
    #[doc(alias = "VK_DRIVER_ID_NVIDIA_PROPRIETARY_KHR")]
    NvidiaProprietary = 4,
    #[doc(alias = "VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS")]
    #[doc(alias = "VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR")]
    IntelProprietaryWindows = 5,
    #[doc(alias = "VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA")]
    #[doc(alias = "VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR")]
    IntelOpenSourceMesa = 6,
    #[doc(alias = "VK_DRIVER_ID_IMAGINATION_PROPRIETARY")]
    #[doc(alias = "VK_DRIVER_ID_IMAGINATION_PROPRIETARY_KHR")]
    ImaginationProprietary = 7,
    #[doc(alias = "VK_DRIVER_ID_QUALCOMM_PROPRIETARY")]
    #[doc(alias = "VK_DRIVER_ID_QUALCOMM_PROPRIETARY_KHR")]
    QualcommProprietary = 8,
    #[doc(alias = "VK_DRIVER_ID_ARM_PROPRIETARY")]
    #[doc(alias = "VK_DRIVER_ID_ARM_PROPRIETARY_KHR")]
    ArmProprietary = 9,
    #[doc(alias = "VK_DRIVER_ID_GOOGLE_SWIFTSHADER")]
    #[doc(alias = "VK_DRIVER_ID_GOOGLE_SWIFTSHADER_KHR")]
    GoogleSwiftshader = 10,
    #[doc(alias = "VK_DRIVER_ID_GGP_PROPRIETARY")]
    #[doc(alias = "VK_DRIVER_ID_GGP_PROPRIETARY_KHR")]
    GgpProprietary = 11,
    #[doc(alias = "VK_DRIVER_ID_BROADCOM_PROPRIETARY")]
    #[doc(alias = "VK_DRIVER_ID_BROADCOM_PROPRIETARY_KHR")]
    BroadcomProprietary = 12,
    #[doc(alias = "VK_DRIVER_ID_MESA_LLVMPIPE")]
    MesaLlvmpipe = 13,
    #[doc(alias = "VK_DRIVER_ID_MOLTENVK")]
    Moltenvk = 14,
    #[doc(alias = "VK_DRIVER_ID_COREAVI_PROPRIETARY")]
    CoreaviProprietary = 15,
    #[doc(alias = "VK_DRIVER_ID_JUICE_PROPRIETARY")]
    JuiceProprietary = 16,
    #[doc(alias = "VK_DRIVER_ID_VERISILICON_PROPRIETARY")]
    VerisiliconProprietary = 17,
    #[doc(alias = "VK_DRIVER_ID_MESA_TURNIP")]
    MesaTurnip = 18,
    #[doc(alias = "VK_DRIVER_ID_MESA_V3DV")]
    MesaV3dv = 19,
    #[doc(alias = "VK_DRIVER_ID_MESA_PANVK")]
    MesaPanvk = 20,
    #[doc(alias = "VK_DRIVER_ID_SAMSUNG_PROPRIETARY")]
    SamsungProprietary = 21,
    #[doc(alias = "VK_DRIVER_ID_MESA_VENUS")]
    MesaVenus = 22,
}
impl Default for DriverId {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl DriverId {
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
            x if x == Self::AmdProprietary.bits() => Some(Self::AmdProprietary),
            x if x == Self::AmdOpenSource.bits() => Some(Self::AmdOpenSource),
            x if x == Self::MesaRadv.bits() => Some(Self::MesaRadv),
            x if x == Self::NvidiaProprietary.bits() => Some(Self::NvidiaProprietary),
            x if x == Self::IntelProprietaryWindows.bits() => Some(Self::IntelProprietaryWindows),
            x if x == Self::IntelOpenSourceMesa.bits() => Some(Self::IntelOpenSourceMesa),
            x if x == Self::ImaginationProprietary.bits() => Some(Self::ImaginationProprietary),
            x if x == Self::QualcommProprietary.bits() => Some(Self::QualcommProprietary),
            x if x == Self::ArmProprietary.bits() => Some(Self::ArmProprietary),
            x if x == Self::GoogleSwiftshader.bits() => Some(Self::GoogleSwiftshader),
            x if x == Self::GgpProprietary.bits() => Some(Self::GgpProprietary),
            x if x == Self::BroadcomProprietary.bits() => Some(Self::BroadcomProprietary),
            x if x == Self::MesaLlvmpipe.bits() => Some(Self::MesaLlvmpipe),
            x if x == Self::Moltenvk.bits() => Some(Self::Moltenvk),
            x if x == Self::CoreaviProprietary.bits() => Some(Self::CoreaviProprietary),
            x if x == Self::JuiceProprietary.bits() => Some(Self::JuiceProprietary),
            x if x == Self::VerisiliconProprietary.bits() => Some(Self::VerisiliconProprietary),
            x if x == Self::MesaTurnip.bits() => Some(Self::MesaTurnip),
            x if x == Self::MesaV3dv.bits() => Some(Self::MesaV3dv),
            x if x == Self::MesaPanvk.bits() => Some(Self::MesaPanvk),
            x if x == Self::SamsungProprietary.bits() => Some(Self::SamsungProprietary),
            x if x == Self::MesaVenus.bits() => Some(Self::MesaVenus),
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
unsafe impl crate::conv::IntoLowLevel for DriverId {
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
unsafe impl crate::conv::FromLowLevel for DriverId {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkShaderFloatControlsIndependence")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ShaderFloatControlsIndependence {
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY")]
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR")]
    N32BitOnly = 0,
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL")]
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR")]
    All = 1,
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE")]
    #[doc(alias = "VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR")]
    None = 2,
}
impl Default for ShaderFloatControlsIndependence {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ShaderFloatControlsIndependence {
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
            x if x == Self::N32BitOnly.bits() => Some(Self::N32BitOnly),
            x if x == Self::All.bits() => Some(Self::All),
            x if x == Self::None.bits() => Some(Self::None),
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
unsafe impl crate::conv::IntoLowLevel for ShaderFloatControlsIndependence {
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
unsafe impl crate::conv::FromLowLevel for ShaderFloatControlsIndependence {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
