//!# [VK_AMD_pipeline_compiler_control](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_pipeline_compiler_control.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_pipeline_compiler_control/VK_AMD_pipeline_compiler_control.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::ffi::CStr;
///# [VkPipelineCompilerControlCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_pipeline_compiler_control/VkPipelineCompilerControlCreateInfoAMD.md")]
#[doc(alias = "VkPipelineCompilerControlCreateInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "compilerControlFlags")]
    compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
///# [VkPipelineCompilerControlFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_pipeline_compiler_control/VkPipelineCompilerControlFlagBitsAMD.md")]
#[doc(alias = "VkPipelineCompilerControlFlagsAMD")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCompilerControlFlagsAMD(u32);
impl PipelineCompilerControlFlagsAMD {
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
impl const std::ops::BitOr for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PipelineCompilerControlFlagsAMD> for PipelineCompilerControlFlagsAMD {
    fn extend<T: IntoIterator<Item = PipelineCompilerControlFlagsAMD>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<PipelineCompilerControlFlagsAMD> for PipelineCompilerControlFlagsAMD {
    fn from_iter<T: IntoIterator<Item = PipelineCompilerControlFlagsAMD>>(
        iterator: T,
    ) -> PipelineCompilerControlFlagsAMD {
        let mut out = Self::empty();
        <Self as Extend<PipelineCompilerControlFlagsAMD>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for PipelineCompilerControlFlagsAMD {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    fn from(bit: PipelineCompilerControlFlagBitsAMD) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    fn extend<T: IntoIterator<Item = PipelineCompilerControlFlagBitsAMD>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    fn from_iter<T: IntoIterator<Item = PipelineCompilerControlFlagBitsAMD>>(
        iterator: T,
    ) -> PipelineCompilerControlFlagsAMD {
        let mut out = Self::empty();
        <Self as Extend<PipelineCompilerControlFlagBitsAMD>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PipelineCompilerControlFlagsAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PipelineCompilerControlFlagsAMD);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PipelineCompilerControlFlagsAMD::empty() {
                    f.write_str("empty")?;
                } else {
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PipelineCompilerControlFlagsAMD))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION")]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME")]
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_pipeline_compiler_control");
///# [VkPipelineCompilerControlFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_pipeline_compiler_control/VkPipelineCompilerControlFlagBitsAMD.md")]
#[doc(alias = "VkPipelineCompilerControlFlagBitsAMD")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PipelineCompilerControlFlagBitsAMD(u32);
impl PipelineCompilerControlFlagBitsAMD {
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
