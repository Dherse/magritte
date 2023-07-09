use crate::{common::vulkan1_0::ImageFormatProperties, cstr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkExternalImageFormatPropertiesNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalImageFormatPropertiesNV {
    #[doc(alias = "imageFormatProperties")]
    pub image_format_properties: ImageFormatProperties,
    #[doc(alias = "externalMemoryFeatures")]
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    #[doc(alias = "exportFromImportedHandleTypes")]
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    #[doc(alias = "compatibleHandleTypes")]
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}
#[doc(alias = "VkExternalMemoryHandleTypeFlagsNV")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryHandleTypeFlagsNV(u32);
impl ExternalMemoryHandleTypeFlagsNV {
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV")]
    pub const OPAQUE_WIN32: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV")]
    pub const OPAQUE_WIN32_KMT: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV")]
    pub const D3D11_IMAGE: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV")]
    pub const D3D11_IMAGE_KMT: Self = Self(8);
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
            all |= Self::OPAQUE_WIN32;
        }
        {
            all |= Self::OPAQUE_WIN32_KMT;
        }
        {
            all |= Self::D3D11_IMAGE;
        }
        {
            all |= Self::D3D11_IMAGE_KMT;
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
impl std::ops::BitOr for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryHandleTypeFlagsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlagsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagsNV>>(
        iterator: T,
    ) -> ExternalMemoryHandleTypeFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalMemoryHandleTypeFlagsNV {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn from(bit: ExternalMemoryHandleTypeFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBitsNV>>(
        iterator: T,
    ) -> ExternalMemoryHandleTypeFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryHandleTypeFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryHandleTypeFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalMemoryHandleTypeFlagsNV::OPAQUE_WIN32) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlagsNV::OPAQUE_WIN32_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_WIN32_KMT))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlagsNV::D3D11_IMAGE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_IMAGE))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlagsNV::D3D11_IMAGE_KMT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(D3D11_IMAGE_KMT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryHandleTypeFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryHandleTypeFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryHandleTypeFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalMemoryFeatureFlagsNV")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalMemoryFeatureFlagsNV(u32);
impl ExternalMemoryFeatureFlagsNV {
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV")]
    pub const DEDICATED_ONLY: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV")]
    pub const EXPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV")]
    pub const IMPORTABLE: Self = Self(4);
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
impl std::ops::BitOr for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryFeatureFlagsNV> for ExternalMemoryFeatureFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlagsNV> for ExternalMemoryFeatureFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlagsNV>>(iterator: T) -> ExternalMemoryFeatureFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl Default for ExternalMemoryFeatureFlagsNV {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn from(bit: ExternalMemoryFeatureFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlagBitsNV>>(iterator: T) -> ExternalMemoryFeatureFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryFeatureFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryFeatureFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryFeatureFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ExternalMemoryFeatureFlagsNV::DEDICATED_ONLY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEDICATED_ONLY))?;
                    }
                    if self.0.contains(ExternalMemoryFeatureFlagsNV::EXPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(EXPORTABLE))?;
                    }
                    if self.0.contains(ExternalMemoryFeatureFlagsNV::IMPORTABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(IMPORTABLE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryFeatureFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryFeatureFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryFeatureFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_external_memory_capabilities");
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalMemoryHandleTypeFlagBitsNV(u32);
impl Default for ExternalMemoryHandleTypeFlagBitsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalMemoryHandleTypeFlagBitsNV {
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV")]
    pub const OPAQUE_WIN32: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV")]
    pub const OPAQUE_WIN32_KMT: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV")]
    pub const D3D11_IMAGE: Self = Self(4);
    #[doc(alias = "VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV")]
    pub const D3D11_IMAGE_KMT: Self = Self(8);
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
            x if x == Self::OPAQUE_WIN32.bits() => Some(Self(x)),
            x if x == Self::OPAQUE_WIN32_KMT.bits() => Some(Self(x)),
            x if x == Self::D3D11_IMAGE.bits() => Some(Self(x)),
            x if x == Self::D3D11_IMAGE_KMT.bits() => Some(Self(x)),
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
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryHandleTypeFlagBitsNV {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryHandleTypeFlagBitsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkExternalMemoryFeatureFlagBitsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ExternalMemoryFeatureFlagBitsNV(u32);
impl Default for ExternalMemoryFeatureFlagBitsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ExternalMemoryFeatureFlagBitsNV {
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV")]
    pub const DEDICATED_ONLY: Self = Self(1);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV")]
    pub const EXPORTABLE: Self = Self(2);
    #[doc(alias = "VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV")]
    pub const IMPORTABLE: Self = Self(4);
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
unsafe impl crate::conv::IntoLowLevel for ExternalMemoryFeatureFlagBitsNV {
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
unsafe impl crate::conv::FromLowLevel for ExternalMemoryFeatureFlagBitsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
